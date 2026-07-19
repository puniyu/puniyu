use async_trait::async_trait;
use puniyu_api::{pkg_name, pkg_version};
use puniyu_context::{PluginContext, ScopeId};
use puniyu_error::AnyError;
use puniyu_event::Event;
use puniyu_middleware::{Middleware, MiddlewareContext};
use semver::Version;
use smol_str::SmolStr;
use std::collections::{BTreeMap, HashMap};
use std::sync::atomic::{AtomicU8, AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};
use thiserror::Error;
use tokio::sync::Notify;

pub const NAME: &str = "puniyu_plugin_event_bus";

const LOADED: u8 = 0;
const RUNNING: u8 = 1;
const STOPPING: u8 = 2;
const STOPPED: u8 = 3;
const UNLOADED: u8 = 4;

#[derive(Debug, Error)]
pub enum Error {
	#[error("event bus is not running")]
	NotRunning,
	#[error("middleware '{middleware}' is already registered by '{owner}'")]
	Conflict { middleware: SmolStr, owner: SmolStr },
	#[error("event bus middleware id exhausted")]
	IdExhausted,
	#[error("event bus state lock is poisoned")]
	Poisoned,
}

struct Registration {
	owner: ScopeId,
	middleware: Arc<dyn Middleware>,
}

#[derive(Default)]
struct Registry {
	next_id: u64,
	entries: BTreeMap<(u32, u64), Registration>,
	owners: HashMap<SmolStr, SmolStr>,
}

struct Inner {
	phase: AtomicU8,
	registry: RwLock<Registry>,
	snapshot: RwLock<Vec<Arc<dyn Middleware>>>,
	in_flight: AtomicUsize,
	drained: Notify,
}

impl Default for Inner {
	fn default() -> Self {
		Self {
			phase: AtomicU8::new(LOADED),
			registry: RwLock::new(Registry::default()),
			snapshot: RwLock::new(Vec::new()),
			in_flight: AtomicUsize::new(0),
			drained: Notify::new(),
		}
	}
}

#[derive(Clone, Default)]
pub struct EventBus {
	inner: Arc<Inner>,
}

impl EventBus {
	fn new() -> Self {
		Self::default()
	}

	pub fn register(
		&self,
		ctx: &PluginContext,
		middleware: impl Into<Arc<dyn Middleware>>,
	) -> Result<(), Error> {
		let middleware = middleware.into();
		let middleware_name = SmolStr::new(middleware.name());
		let mut registry = self.inner.registry.write().map_err(|_| Error::Poisoned)?;
		if matches!(self.inner.phase.load(Ordering::Acquire), STOPPING | UNLOADED) {
			return Err(Error::NotRunning);
		}
		if let Some(owner) = registry.owners.get(&middleware_name) {
			return Err(Error::Conflict { middleware: middleware_name, owner: owner.clone() });
		}

		let id = registry.next_id;
		registry.next_id = registry.next_id.checked_add(1).ok_or(Error::IdExhausted)?;
		registry.owners.insert(middleware_name, SmolStr::new(ctx.plugin_name()));
		registry.entries.insert(
			(middleware.priority(), id),
			Registration { owner: ctx.scope_id(), middleware },
		);
		if self.inner.phase.load(Ordering::Acquire) == RUNNING {
			self.refresh_snapshot(&registry)?;
		}
		Ok(())
	}

	/// 移除当前插件作用域注册的全部中间件。
	pub fn unregister(&self, ctx: &PluginContext) -> Result<(), Error> {
		let mut registry = self.inner.registry.write().map_err(|_| Error::Poisoned)?;
		if matches!(self.inner.phase.load(Ordering::Acquire), STOPPING | UNLOADED) {
			return Err(Error::NotRunning);
		}
		registry.entries.retain(|_, entry| entry.owner != ctx.scope_id());
		let active_names = registry
			.entries
			.values()
			.map(|entry| SmolStr::new(entry.middleware.name()))
			.collect::<Vec<_>>();
		registry.owners.retain(|name, _| active_names.contains(name));
		if self.inner.phase.load(Ordering::Acquire) == RUNNING {
			self.refresh_snapshot(&registry)?;
		}
		Ok(())
	}

	pub async fn emit(&self, event: Event) -> Result<(), Error> {
		let _guard = self.enter()?;
		let middlewares = self.inner.snapshot.read().map_err(|_| Error::Poisoned)?.clone();
		MiddlewareContext::new(&event, &middlewares).next().await;
		Ok(())
	}

	pub fn is_running(&self) -> bool {
		self.inner.phase.load(Ordering::Acquire) == RUNNING
	}

	fn start(&self) -> Result<(), Error> {
		let registry = self.inner.registry.write().map_err(|_| Error::Poisoned)?;
		match self.inner.phase.load(Ordering::Acquire) {
			RUNNING => return Ok(()),
			STOPPING | UNLOADED => return Err(Error::NotRunning),
			_ => {}
		}
		let mut snapshot = self.inner.snapshot.write().map_err(|_| Error::Poisoned)?;
		*snapshot = registry.entries.values().map(|entry| Arc::clone(&entry.middleware)).collect();
		self.inner.phase.store(RUNNING, Ordering::Release);
		Ok(())
	}

	fn refresh_snapshot(&self, registry: &Registry) -> Result<(), Error> {
		let mut snapshot = self.inner.snapshot.write().map_err(|_| Error::Poisoned)?;
		*snapshot = registry.entries.values().map(|entry| Arc::clone(&entry.middleware)).collect();
		Ok(())
	}

	async fn stop(&self) {
		let previous = self.inner.phase.swap(STOPPING, Ordering::AcqRel);
		if matches!(previous, LOADED | STOPPED | UNLOADED) {
			self.inner.phase.store(STOPPED, Ordering::Release);
			return;
		}

		loop {
			let notified = self.inner.drained.notified();
			if self.inner.in_flight.load(Ordering::Acquire) == 0 {
				break;
			}
			notified.await;
		}
		self.inner.phase.store(STOPPED, Ordering::Release);
	}

	fn clear(&self) -> Result<(), Error> {
		let mut registry = self.inner.registry.write().map_err(|_| Error::Poisoned)?;
		let mut snapshot = self.inner.snapshot.write().map_err(|_| Error::Poisoned)?;
		registry.entries.clear();
		registry.owners.clear();
		*snapshot = Vec::new();
		self.inner.phase.store(UNLOADED, Ordering::Release);
		Ok(())
	}

	fn enter(&self) -> Result<DispatchGuard, Error> {
		if self.inner.phase.load(Ordering::Acquire) != RUNNING {
			return Err(Error::NotRunning);
		}
		self.inner.in_flight.fetch_add(1, Ordering::AcqRel);
		if self.inner.phase.load(Ordering::Acquire) != RUNNING {
			if self.inner.in_flight.fetch_sub(1, Ordering::AcqRel) == 1 {
				self.inner.drained.notify_waiters();
			}
			return Err(Error::NotRunning);
		}
		Ok(DispatchGuard { inner: Arc::clone(&self.inner) })
	}
}

struct DispatchGuard {
	inner: Arc<Inner>,
}

impl Drop for DispatchGuard {
	fn drop(&mut self) {
		if self.inner.in_flight.fetch_sub(1, Ordering::AcqRel) == 1 {
			self.inner.drained.notify_waiters();
		}
	}
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Plugin;

#[async_trait]
impl puniyu_plugin_core::Plugin for Plugin {
	fn name(&self) -> &str {
		pkg_name!()
	}

	fn version(&self) -> Version {
		pkg_version!()
	}

	async fn on_start(&self, ctx: &PluginContext) -> AnyError {
		let bus = EventBus::new();
		bus.start()?;
		if let Err(error) = ctx.provide(bus.clone()) {
			bus.stop().await;
			bus.clear()?;
			return Err(Box::new(error));
		}
		Ok(())
	}

	async fn on_stop(&self, ctx: &PluginContext) -> AnyError {
		if let Some(bus) = ctx.remove::<EventBus>() {
			bus.stop().await;
			bus.clear()?;
		}
		Ok(())
	}
}
