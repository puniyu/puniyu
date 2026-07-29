use crate::{Error, HttpMount, ServerOptions};
use arc_swap::ArcSwap;
use parking_lot::Mutex;
use salvo::conn::TcpListener;
use salvo::prelude::{Handler, Router, Service};
use salvo::server::ServerHandle;
use salvo::Listener;
use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use tokio::task::JoinHandle;

type RouterFactory = Arc<dyn Fn() -> Router + Send + Sync + 'static>;

#[derive(Clone)]
pub(crate) enum MountContent {
	Router(RouterFactory),
	Hoop(Arc<dyn Handler>),
}

struct Running {
	handle: ServerHandle,
	task: JoinHandle<std::io::Result<()>>,
}

pub struct Server {
	options: ServerOptions,
	mounts: Mutex<BTreeMap<u64, MountContent>>,
	next_id: AtomicU64,
	service: Arc<ArcSwap<Service>>,
	accepting: AtomicBool,
	running: Mutex<Option<Running>>,
}

impl Server {
	pub fn new(options: ServerOptions) -> Arc<Self> {
		Arc::new(Self {
			options,
			mounts: Mutex::new(BTreeMap::new()),
			next_id: AtomicU64::new(0),
			service: Arc::new(ArcSwap::from_pointee(Service::new(Router::new()))),
			accepting: AtomicBool::new(false),
			running: Mutex::new(None),
		})
	}

	pub fn router<F>(self: &Arc<Self>, build: F) -> HttpMount
	where
		F: Fn() -> Router + Send + Sync + 'static,
	{
		HttpMount::new(Arc::downgrade(self), MountContent::Router(Arc::new(build)))
	}

	pub fn hoop<H>(self: &Arc<Self>, hoop: H) -> HttpMount
	where
		H: Handler,
	{
		HttpMount::new(Arc::downgrade(self), MountContent::Hoop(Arc::new(hoop)))
	}

	pub async fn start(self: &Arc<Self>) -> Result<(), Error> {
		{
			let running = self.running.lock();
			if running.is_some() {
				return Err(Error::AlreadyRunning);
			}
		}
		let address = SocketAddr::new(self.options.host, self.options.port);
		let listener = match TcpListener::new(address).try_bind().await {
			Ok(l) => l,
			Err(e) => return Err(Error::Bind(e.to_string())),
		};
		let mut running = self.running.lock();
		self.rebuild_service();
		self.accepting.store(true, Ordering::Release);
		let proxy = ServiceProxy(Arc::clone(&self.service));
		let server = salvo::Server::new(listener);
		let handle = server.handle();
		let task = tokio::spawn(server.try_serve(Service::new(Router::with_path("{**}").goal(proxy))));
		*running = Some(Running { handle, task });
		log::info!("Server running on {address}");
		Ok(())
	}

	pub async fn stop(&self) -> Result<(), Error> {
		let running = self.running.lock().take();
		let Some(running) = running else {
			return Ok(());
		};
		self.accepting.store(false, Ordering::Release);
		running.handle.stop_graceful(Some(self.options.shutdown_timeout));
		match running.task.await {
			Ok(Ok(())) => Ok(()),
			Ok(Err(e)) => Err(Error::Serve(e)),
			Err(e) => Err(Error::Task(e)),
		}
	}

	pub(crate) fn mount(&self, content: MountContent) -> Result<u64, Error> {
		if !self.accepting.load(Ordering::Acquire) {
			return Err(Error::NotRunning);
		}
		let id = self.next_id.fetch_add(1, Ordering::Relaxed);
		self.mounts.lock().insert(id, content);
		self.rebuild_service();
		Ok(id)
	}

	pub(crate) fn unmount(&self, id: u64) {
		self.mounts.lock().remove(&id);
		self.rebuild_service();
	}

	fn rebuild_service(&self) {
		let mounts = self.mounts.lock();
		let mut root = Router::new();
		let mut hoops = Vec::new();
		for content in mounts.values() {
			match content {
				MountContent::Router(build) => root = root.push(build()),
				MountContent::Hoop(hoop) => hoops.push(hoop.clone()),
			}
		}
		let mut svc = Service::new(root);
		svc.hoops = hoops;
		self.service.store(Arc::new(svc));
	}
}

impl Drop for Server {
	fn drop(&mut self) {
		self.accepting.store(false, Ordering::Release);
		if let Some(running) = self.running.get_mut().take() {
			running.handle.stop_forceful();
			running.task.abort();
		}
	}
}

struct ServiceProxy(Arc<ArcSwap<Service>>);

#[salvo::async_trait]
impl Handler for ServiceProxy {
	async fn handle(
		&self,
		req: &mut salvo::Request,
		_depot: &mut salvo::Depot,
		res: &mut salvo::Response,
		ctrl: &mut salvo::FlowCtrl,
	) {
		let service = self.0.load_full();
		let handler = service.hyper_handler(
			req.local_addr().clone(),
			req.remote_addr().clone(),
			req.scheme().clone(),
			None,
			ctrl.conn().clone(),
			None,
		);
		*res = handler.handle(std::mem::take(req)).await;
		ctrl.skip_rest();
	}
}
