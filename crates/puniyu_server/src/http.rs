use crate::{Error, HttpMount};
use arc_swap::ArcSwap;
use parking_lot::Mutex;
use salvo::{Handler, Router, Service};
use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct MountId(u64);

impl MountId {
	fn next(self) -> Option<Self> {
		self.0.checked_add(1).map(Self)
	}
}

type RouterFactory = Arc<dyn Fn() -> Router + Send + Sync + 'static>;

#[derive(Clone)]
pub(crate) enum MountContent {
	Router(RouterFactory),
	Hoop(Arc<dyn Handler>),
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum HttpState {
	#[default]
	Idle,
	Starting,
	Running,
	Draining,
}

struct Registry {
	state: HttpState,
	next_id: Option<MountId>,
	generation: u64,
	entries: BTreeMap<MountId, MountContent>,
}

impl Default for Registry {
	fn default() -> Self {
		Self {
			state: HttpState::Idle,
			next_id: Some(MountId(0)),
			generation: 0,
			entries: BTreeMap::new(),
		}
	}
}

impl Registry {
	fn allocate_id(&mut self) -> Result<MountId, Error> {
		let id = self.next_id.ok_or(Error::MountIdExhausted)?;
		self.next_id = id.next();
		Ok(id)
	}

	fn changed(&mut self) {
		self.generation = self.generation.wrapping_add(1);
	}
}

pub(crate) struct HttpInner {
	accepting: AtomicBool,
	registry: Mutex<Registry>,
	pub(crate) service: ArcSwap<Service>,
}

impl Default for HttpInner {
	fn default() -> Self {
		Self {
			accepting: AtomicBool::new(false),
			registry: Mutex::new(Registry::default()),
			service: ArcSwap::from_pointee(Service::new(Router::new())),
		}
	}
}

#[derive(Clone)]
pub struct Http {
	pub(crate) inner: Arc<HttpInner>,
}

impl Http {
	pub(crate) fn new() -> Self {
		Self { inner: Arc::new(HttpInner::default()) }
	}

	pub fn router<F>(&self, build: F) -> HttpMount
	where
		F: Fn() -> Router + Send + Sync + 'static,
	{
		HttpMount::new(&self.inner, MountContent::Router(Arc::new(build)))
	}

	pub fn hoop<H>(&self, hoop: H) -> HttpMount
	where
		H: Handler,
	{
		HttpMount::new(&self.inner, MountContent::Hoop(Arc::new(hoop)))
	}

	pub(crate) fn begin_start(&self) -> Result<(), Error> {
		let mut registry = self.inner.registry.lock();
		if registry.state != HttpState::Idle {
			return Err(Error::AlreadyRunning);
		}
		registry.state = HttpState::Starting;
		Ok(())
	}

	pub(crate) fn abort_start(&self) {
		let mut registry = self.inner.registry.lock();
		if registry.state == HttpState::Starting {
			registry.state = HttpState::Idle;
		}
		self.inner.accepting.store(false, Ordering::Release);
	}

	pub(crate) fn mark_running(&self) -> Result<(), Error> {
		let mut registry = self.inner.registry.lock();
		if registry.state != HttpState::Starting {
			return Err(Error::NotRunning);
		}
		registry.state = HttpState::Running;
		self.inner.accepting.store(true, Ordering::Release);
		Ok(())
	}

	pub(crate) fn drain(&self) -> Result<(), Error> {
		let mut registry = self.inner.registry.lock();
		match registry.state {
			HttpState::Running => {
				registry.state = HttpState::Draining;
				self.inner.accepting.store(false, Ordering::Release);
				Ok(())
			}
			HttpState::Draining => Ok(()),
			HttpState::Idle | HttpState::Starting => Err(Error::NotRunning),
		}
	}

	pub(crate) fn finish_stop(&self) {
		let mut registry = self.inner.registry.lock();
		registry.state = HttpState::Idle;
		self.inner.accepting.store(false, Ordering::Release);
	}
}

impl HttpInner {
	pub(crate) fn is_running(&self) -> bool {
		self.accepting.load(Ordering::Acquire)
	}

	pub(crate) fn mount(&self, content: MountContent) -> Result<MountId, Error> {
		let id = {
			let mut registry = self.registry.lock();
			match registry.state {
				HttpState::Running => {}
				HttpState::Draining => return Err(Error::Draining),
				HttpState::Idle | HttpState::Starting => return Err(Error::NotRunning),
			}
			let id = registry.allocate_id()?;
			registry.entries.insert(id, content);
			registry.changed();
			id
		};
		self.publish_latest();
		Ok(id)
	}

	pub(crate) fn unmount(&self, id: MountId) {
		{
			let mut registry = self.registry.lock();
			if registry.entries.remove(&id).is_none() {
				return;
			};
			registry.changed();
		}
		self.publish_latest();
	}

	fn publish_latest(&self) {
		loop {
			let (generation, entries) = {
				let registry = self.registry.lock();
				(registry.generation, registry.entries.values().cloned().collect::<Vec<_>>())
			};
			let service = build_service(entries);
			let registry = self.registry.lock();
			if registry.generation == generation {
				self.service.store(Arc::new(service));
				return;
			}
		}
	}
}

fn build_service(entries: Vec<MountContent>) -> Service {
	let mut root = Router::new();
	let mut hoops = Vec::new();
	for content in entries {
		match content {
			MountContent::Router(build) => root = root.push(build()),
			MountContent::Hoop(hoop) => hoops.push(hoop),
		}
	}
	let mut service = Service::new(root);
	service.hoops = hoops;
	service
}
