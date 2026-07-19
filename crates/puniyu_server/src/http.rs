use crate::{Error, HttpMount};
use arc_swap::ArcSwap;
use salvo::{Handler, Router, Service};
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::{Arc, Mutex};

pub(crate) type MountId = u64;
type RouterFactory = Arc<dyn Fn() -> Router + Send + Sync + 'static>;

const DETACHED: u8 = 0;
const STARTING: u8 = 1;
const RUNNING: u8 = 2;
const DRAINING: u8 = 3;
const STOPPED: u8 = 4;

#[derive(Clone)]
enum Mounted {
	Router(RouterFactory),
	Hoop(Arc<dyn Handler>),
}

#[derive(Default)]
struct Registry {
	next_id: MountId,
	generation: u64,
	entries: BTreeMap<MountId, Mounted>,
}

impl Registry {
	fn next_id(&mut self) -> Result<MountId, Error> {
		let id = self.next_id;
		self.next_id = self.next_id.checked_add(1).ok_or(Error::MountIdExhausted)?;
		Ok(id)
	}

	fn changed(&mut self) {
		self.generation = self.generation.wrapping_add(1);
	}
}

pub(crate) struct HttpInner {
	phase: AtomicU8,
	attached: AtomicBool,
	registry: Mutex<Registry>,
	pub(crate) service: ArcSwap<Service>,
}

impl Default for HttpInner {
	fn default() -> Self {
		Self {
			phase: AtomicU8::new(DETACHED),
			attached: AtomicBool::new(false),
			registry: Mutex::new(Registry::default()),
			service: ArcSwap::from_pointee(Service::new(Router::new())),
		}
	}
}

#[derive(Clone, Default)]
pub struct Http {
	pub(crate) inner: Arc<HttpInner>,
}

impl Http {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn router<F>(&self, build: F) -> Result<HttpMount, Error>
	where
		F: Fn() -> Router + Send + Sync + 'static,
	{
		self.inner.insert(Mounted::Router(Arc::new(build)))
	}

	pub fn hoop<H>(&self, hoop: H) -> Result<HttpMount, Error>
	where
		H: Handler,
	{
		self.inner.insert(Mounted::Hoop(Arc::new(hoop)))
	}

	pub(crate) fn attach(&self) -> Result<(), Error> {
		self.inner
			.attached
			.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
			.map(|_| ())
			.map_err(|_| Error::AlreadyAttached)
	}

	pub(crate) fn begin_start(&self) -> Result<(), Error> {
		self.inner
			.phase
			.compare_exchange(DETACHED, STARTING, Ordering::AcqRel, Ordering::Acquire)
			.map(|_| ())
			.map_err(|_| Error::AlreadyRunning)
	}

	pub(crate) fn mark_running(&self) {
		self.inner.phase.store(RUNNING, Ordering::Release);
	}

	pub(crate) fn drain(&self) -> Result<(), Error> {
		let _registry = self.inner.registry.lock().map_err(|_| Error::Poisoned)?;
		match self.inner.phase.load(Ordering::Acquire) {
			RUNNING => {
				self.inner.phase.store(DRAINING, Ordering::Release);
				Ok(())
			}
			DRAINING | STOPPED => Ok(()),
			_ => Err(Error::NotRunning),
		}
	}

	pub(crate) fn mark_stopped(&self) {
		self.inner.phase.store(STOPPED, Ordering::Release);
	}
}

impl HttpInner {
	pub(crate) fn is_running(&self) -> bool {
		self.phase.load(Ordering::Acquire) == RUNNING
	}

	fn insert(self: &Arc<Self>, mounted: Mounted) -> Result<HttpMount, Error> {
		let id = {
			let mut registry = self.registry.lock().map_err(|_| Error::Poisoned)?;
			match self.phase.load(Ordering::Acquire) {
				RUNNING => {}
				DRAINING => return Err(Error::Draining),
				_ => return Err(Error::NotRunning),
			}
			let id = registry.next_id()?;
			registry.entries.insert(id, mounted);
			registry.changed();
			id
		};
		self.publish_latest()?;
		Ok(HttpMount { inner: Arc::downgrade(self), id, mounted: true })
	}

	pub(crate) fn unmount(&self, id: MountId) -> Result<(), Error> {
		let publish = {
			let mut registry = self.registry.lock().map_err(|_| Error::Poisoned)?;
			if registry.entries.remove(&id).is_none() {
				return Ok(());
			}
			registry.changed();
			matches!(self.phase.load(Ordering::Acquire), RUNNING | DRAINING)
		};
		if publish {
			self.publish_latest()?;
		}
		Ok(())
	}

	fn publish_latest(&self) -> Result<(), Error> {
		loop {
			let (generation, entries) = {
				let registry = self.registry.lock().map_err(|_| Error::Poisoned)?;
				(registry.generation, registry.entries.values().cloned().collect::<Vec<_>>())
			};
			let service = build_service(entries);
			let registry = self.registry.lock().map_err(|_| Error::Poisoned)?;
			if registry.generation == generation {
				self.service.store(Arc::new(service));
				return Ok(());
			}
		}
	}
}

fn build_service(entries: Vec<Mounted>) -> Service {
	let mut root = Router::new();
	let mut hoops = Vec::new();
	for mounted in entries {
		match mounted {
			Mounted::Router(build) => root = root.push(build()),
			Mounted::Hoop(hoop) => hoops.push(hoop),
		}
	}
	let mut service = Service::new(root);
	service.hoops = hoops;
	service
}
