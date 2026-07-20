use crate::proxy::HttpServiceProxy;
use crate::{Error, Http, ServerOptions};
use salvo::conn::{Listener, TcpListener};
use salvo::prelude::{Router, Service};
use salvo::server::ServerHandle;
use std::net::SocketAddr;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

struct Running {
	handle: ServerHandle,
	task: JoinHandle<std::io::Result<()>>,
}

enum ServerState {
	Idle,
	Running(Running),
	Draining(Running),
}

pub struct Server {
	options: ServerOptions,
	http: Http,
	state: Mutex<ServerState>,
}

impl Server {
	pub fn new(options: ServerOptions) -> Self {
		Self { options, http: Http::new(), state: Mutex::new(ServerState::Idle) }
	}

	pub fn http(&self) -> Http {
		self.http.clone()
	}

	pub async fn start(&self) -> Result<(), Error> {
		let mut state = self.state.lock().await;
		if !matches!(*state, ServerState::Idle) {
			return Err(Error::AlreadyRunning);
		}
		self.http.begin_start()?;
		let address = SocketAddr::new(self.options.host, self.options.port);
		let listener = match TcpListener::new(address).try_bind().await {
			Ok(listener) => listener,
			Err(error) => {
				self.http.abort_start();
				return Err(Error::Bind(error.to_string()));
			}
		};
		let proxy = HttpServiceProxy::new(self.http.inner.clone());
		let entry = Router::with_path("{**path}").goal(proxy);
		let service = Service::new(entry);
		let server = salvo::Server::new(listener);
		let handle = server.handle();

		if let Err(error) = self.http.mark_running() {
			self.http.abort_start();
			return Err(error);
		}
		let task = tokio::spawn(server.try_serve(service));
		*state = ServerState::Running(Running { handle, task });
		log::info!("Server running on {address}");
		Ok(())
	}

	pub async fn drain(&self) -> Result<(), Error> {
		let mut state = self.state.lock().await;
		let current = std::mem::replace(&mut *state, ServerState::Idle);
		match current {
			ServerState::Idle => Err(Error::NotRunning),
			ServerState::Draining(running) => {
				*state = ServerState::Draining(running);
				Ok(())
			}
			ServerState::Running(running) => {
				if let Err(error) = self.http.drain() {
					*state = ServerState::Running(running);
					return Err(error);
				}
				running.handle.stop_graceful(Some(self.options.shutdown_timeout));
				*state = ServerState::Draining(running);
				Ok(())
			}
		}
	}

	pub async fn stop(&self) -> Result<(), Error> {
		let mut state = self.state.lock().await;
		let current = std::mem::replace(&mut *state, ServerState::Idle);
		let running = match current {
			ServerState::Idle => {
				self.http.finish_stop();
				return Ok(());
			}
			ServerState::Draining(running) => running,
			ServerState::Running(running) => {
				if let Err(error) = self.http.drain() {
					*state = ServerState::Running(running);
					return Err(error);
				}
				running.handle.stop_graceful(Some(self.options.shutdown_timeout));
				running
			}
		};
		let result = match running.task.await {
			Ok(Ok(())) => Ok(()),
			Ok(Err(error)) => Err(Error::Serve(error)),
			Err(error) => Err(Error::Task(error)),
		};
		self.http.finish_stop();
		result
	}
}

impl Drop for Server {
	fn drop(&mut self) {
		self.http.finish_stop();
		let state = std::mem::replace(self.state.get_mut(), ServerState::Idle);
		if let ServerState::Running(running) | ServerState::Draining(running) = state {
			running.handle.stop_forceful();
			running.task.abort();
		}
	}
}
