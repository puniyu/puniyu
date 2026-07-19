use crate::dispatcher::HttpDispatcher;
use crate::{Error, Http, ServerOptions};
use salvo::conn::{Listener, TcpListener};
use salvo::prelude::{Router, Service};
use salvo::server::ServerHandle;
use std::net::SocketAddr;
use std::sync::Mutex;
use tokio::task::JoinHandle;

struct Running {
	handle: ServerHandle,
	task: JoinHandle<()>,
}

pub struct Server {
	options: ServerOptions,
	http: Http,
	runtime: Mutex<Option<Running>>,
}

impl Server {
	pub fn new(options: ServerOptions, http: Http) -> Result<Self, Error> {
		http.attach()?;
		Ok(Self { options, http, runtime: Mutex::new(None) })
	}

	pub async fn start(&self) -> Result<(), Error> {
		self.http.begin_start()?;
		let address = SocketAddr::new(self.options.host, self.options.port);
		let listener = TcpListener::new(address)
			.try_bind()
			.await
			.map_err(|error| Error::Bind(error.to_string()))?;
		let dispatcher = HttpDispatcher::new(self.http.inner.clone());
		let entry = Router::with_path("{**path}").goal(dispatcher);
		let service = Service::new(entry);
		let server = salvo::Server::new(listener);
		let handle = server.handle();
		let task = tokio::spawn(server.serve(service));

		let mut runtime = self.runtime.lock().map_err(|_| Error::Poisoned)?;
		if runtime.is_some() {
			handle.stop_forceful();
			return Err(Error::AlreadyRunning);
		}
		*runtime = Some(Running { handle, task });
		self.http.mark_running();
		log::info!("Server running on {address}");
		Ok(())
	}

	pub fn drain(&self) -> Result<(), Error> {
		self.http.drain()?;
		if let Some(running) = self.runtime.lock().map_err(|_| Error::Poisoned)?.as_ref() {
			running.handle.stop_graceful(Some(self.options.shutdown_timeout));
		}
		Ok(())
	}

	pub async fn stop(&self) -> Result<(), Error> {
		let _ = self.drain();
		let running = self.runtime.lock().map_err(|_| Error::Poisoned)?.take();
		if let Some(running) = running {
			running.task.await?;
		}
		self.http.mark_stopped();
		Ok(())
	}
}
