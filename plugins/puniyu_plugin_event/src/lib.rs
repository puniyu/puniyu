use arc_swap::ArcSwap;
use async_trait::async_trait;
use parking_lot::Mutex;
use puniyu_api::{pkg_name, pkg_version};
use puniyu_context::PluginContext;
use puniyu_error::AnyError;
use puniyu_event::{Event, EventType};
use puniyu_middleware::{Middleware, MiddlewareContext};
use semver::Version;
use smol_str::SmolStr;
use std::collections::{BTreeMap, HashMap};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Weak};
use thiserror::Error;
use tokio::sync::Notify;

pub const NAME: &str = "puniyu_plugin_event";

#[derive(Debug, Error)]
pub enum Error {
	#[error("event emitter is not running")]
	NotRunning,
	#[error("middleware '{middleware}' is already listening to '{event_type}'")]
	AlreadyListening { event_type: EventType, middleware: SmolStr },
	#[error("event emitter middleware id exhausted")]
	IdExhausted,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct MiddlewareId(u64);

impl MiddlewareId {
	fn next(self) -> Option<Self> {
		self.0.checked_add(1).map(Self)
	}
}

struct Registration {
	name: SmolStr,
	middleware: Weak<dyn Middleware>,
}

#[derive(Default)]
struct Topic {
	entries: BTreeMap<(u32, MiddlewareId), Registration>,
}

impl Topic {
	fn prune(&mut self) {
		self.entries.retain(|_, entry| entry.middleware.strong_count() > 0);
	}
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum EventState {
	#[default]
	Idle,
	Running,
	Stopping,
	Stopped,
}

struct Registry {
	state: EventState,
	next_id: Option<MiddlewareId>,
	topics: HashMap<EventType, Topic>,
}

impl Default for Registry {
	fn default() -> Self {
		Self { state: EventState::Idle, next_id: Some(MiddlewareId(0)), topics: HashMap::new() }
	}
}

impl Registry {
	fn allocate_id(&mut self) -> Result<MiddlewareId, Error> {
		let id = self.next_id.ok_or(Error::IdExhausted)?;
		self.next_id = id.next();
		Ok(id)
	}
}

type Snapshot = HashMap<EventType, Vec<Weak<dyn Middleware>>>;

struct Inner {
	accepting: AtomicBool,
	registry: Mutex<Registry>,
	snapshot: ArcSwap<Snapshot>,
	in_flight: AtomicUsize,
	drained: Notify,
}

impl Default for Inner {
	fn default() -> Self {
		Self {
			accepting: AtomicBool::new(false),
			registry: Mutex::new(Registry::default()),
			snapshot: ArcSwap::from_pointee(HashMap::new()),
			in_flight: AtomicUsize::new(0),
			drained: Notify::new(),
		}
	}
}

#[derive(Clone)]
pub struct EventEmitter {
	inner: Arc<Inner>,
}

impl EventEmitter {
	fn new() -> Self {
		Self { inner: Arc::new(Inner::default()) }
	}

	pub fn on(
		&self,
		event_type: EventType,
		middleware: impl Into<Arc<dyn Middleware>>,
	) -> Result<(), Error> {
		let middleware = middleware.into();
		let name = SmolStr::new(middleware.name());
		let mut registry = self.inner.registry.lock();
		if registry.state != EventState::Running {
			return Err(Error::NotRunning);
		}

		{
			let topic = registry.topics.entry(event_type).or_default();
			topic.prune();
			if topic.entries.values().any(|entry| entry.name == name) {
				return Err(Error::AlreadyListening { event_type, middleware: name });
			}
		}

		let id = registry.allocate_id()?;
		registry.topics.entry(event_type).or_default().entries.insert(
			(middleware.priority(), id),
			Registration { name, middleware: Arc::downgrade(&middleware) },
		);
		self.inner.publish(&registry);
		Ok(())
	}

	pub fn off(&self, event_type: EventType, middleware: impl Into<Arc<dyn Middleware>>) {
		let middleware = middleware.into();
		let target = Arc::downgrade(&middleware);
		let mut registry = self.inner.registry.lock();
		let remove_topic = if let Some(topic) = registry.topics.get_mut(&event_type) {
			topic.entries.retain(|_, entry| {
				entry.middleware.strong_count() > 0 && !Weak::ptr_eq(&entry.middleware, &target)
			});
			topic.entries.is_empty()
		} else {
			false
		};
		if remove_topic {
			registry.topics.remove(&event_type);
		}
		self.inner.publish(&registry);
	}

	pub async fn emit(&self, event: Event) -> Result<(), Error> {
		let _guard = self.inner.enter()?;
		let event_type = event.event_type();
		let snapshot = self.inner.snapshot.load();
		let middlewares = snapshot
			.get(&event_type)
			.into_iter()
			.flatten()
			.filter_map(Weak::upgrade)
			.collect::<Vec<_>>();
		MiddlewareContext::new(&event, &middlewares).next().await;
		Ok(())
	}

	fn start(&self) -> Result<(), Error> {
		let mut registry = self.inner.registry.lock();
		match registry.state {
			EventState::Running => return Ok(()),
			EventState::Stopping => return Err(Error::NotRunning),
			EventState::Idle | EventState::Stopped => registry.state = EventState::Running,
		}
		self.inner.accepting.store(true, Ordering::Release);
		self.inner.publish(&registry);
		Ok(())
	}

	async fn stop(&self) {
		{
			let mut registry = self.inner.registry.lock();
			self.inner.accepting.store(false, Ordering::Release);
			match registry.state {
				EventState::Running | EventState::Stopping => {
					registry.state = EventState::Stopping;
				}
				EventState::Idle | EventState::Stopped => {
					registry.state = EventState::Stopped;
					registry.topics.clear();
					self.inner.publish(&registry);
					return;
				}
			}
		}

		loop {
			let notified = self.inner.drained.notified();
			if self.inner.in_flight.load(Ordering::Acquire) == 0 {
				break;
			}
			notified.await;
		}

		let mut registry = self.inner.registry.lock();
		registry.topics.clear();
		registry.state = EventState::Stopped;
		self.inner.publish(&registry);
	}
}

impl Inner {
	fn publish(&self, registry: &Registry) {
		let snapshot = registry
			.topics
			.iter()
			.filter_map(|(event_type, topic)| {
				let middlewares = topic
					.entries
					.values()
					.filter(|entry| entry.middleware.strong_count() > 0)
					.map(|entry| entry.middleware.clone())
					.collect::<Vec<_>>();
				(!middlewares.is_empty()).then_some((*event_type, middlewares))
			})
			.collect();
		self.snapshot.store(Arc::new(snapshot));
	}

	fn enter(self: &Arc<Self>) -> Result<DispatchGuard, Error> {
		if !self.accepting.load(Ordering::Acquire) {
			return Err(Error::NotRunning);
		}
		self.in_flight.fetch_add(1, Ordering::AcqRel);
		if !self.accepting.load(Ordering::Acquire) {
			if self.in_flight.fetch_sub(1, Ordering::AcqRel) == 1 {
				self.drained.notify_waiters();
			}
			return Err(Error::NotRunning);
		}
		Ok(DispatchGuard { inner: Arc::clone(self) })
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
		let emitter = EventEmitter::new();
		emitter.start()?;
		if let Err(error) = ctx.provide(emitter.clone()) {
			emitter.stop().await;
			return Err(Box::new(error));
		}
		Ok(())
	}

	async fn on_stop(&self, ctx: &PluginContext) -> AnyError {
		if let Some(emitter) = ctx.remove::<EventEmitter>() {
			emitter.stop().await;
		}
		Ok(())
	}
}
