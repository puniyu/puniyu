mod error;
pub use error::Error;

use arc_swap::ArcSwap;
use async_trait::async_trait;
use parking_lot::Mutex;
use puniyu_api::{pkg_name, pkg_version};
use puniyu_context::ServiceContext;
use puniyu_error::AnyError;
use puniyu_event::{Event, EventType};
use puniyu_handler::{Handler, HandlerContext};
use puniyu_registry::Registry;
use semver::Version;
use smol_str::SmolStr;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Weak};
use tokio::sync::Notify;



#[derive(Clone)]
struct Registration {
	name: SmolStr,
	priority: u32,
	handler: Weak<dyn Handler>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum State {
	#[default]
	Starting,
	Running,
	Stopping,
	Stopped,
}

type Snapshot = HashMap<EventType, Vec<Weak<dyn Handler>>>;

struct Inner {
	accepting: AtomicBool,
	state: Mutex<State>,
	topics: Mutex<HashMap<EventType, Registry<Registration>>>,
	snapshot: ArcSwap<Snapshot>,
	in_flight: AtomicUsize,
	drained: Notify,
}

impl Default for Inner {
	fn default() -> Self {
		Self {
			accepting: AtomicBool::new(false),
			state: Mutex::new(State::default()),
			topics: Mutex::new(HashMap::new()),
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
		handler: impl Into<Arc<dyn Handler>>,
	) -> Result<(), Error> {
		let handler = handler.into();
		let name = SmolStr::new(handler.name());
		let priority = handler.priority();

		let state = self.inner.state.lock();
		if *state != State::Running {
			return Err(Error::NotRunning);
		}

		let mut topics = self.inner.topics.lock();
		let topic = topics.entry(event_type).or_default();


		let mut duplicate = false;
		topic.iter(|_, reg| {
			if reg.name == name {
				duplicate = true;
			}
		});
		if duplicate {
			return Err(Error::AlreadyListening { event_type, handler: name });
		}

		topic.retain(|_, reg| reg.handler.strong_count() > 0);

		topic.insert(Registration { name, priority, handler: Arc::downgrade(&handler) });
		drop(topics);
		drop(state);

		self.inner.publish();
		Ok(())
	}

	pub fn off(&self, event_type: EventType, handler: impl Into<Arc<dyn Handler>>) {
		let target = Arc::downgrade(&handler.into());
		let mut topics = self.inner.topics.lock();
		if let Some(topic) = topics.get(&event_type) {
			topic.retain(|_, reg| {
				reg.handler.strong_count() > 0 && !Weak::ptr_eq(&reg.handler, &target)
			});
			if topic.is_empty() {
				topics.remove(&event_type);
			}
		}
		drop(topics);
		self.inner.publish();
	}

	pub async fn emit(&self, event: Event) -> Result<(), Error> {
		let _guard = self.inner.enter()?;
		let event_type = event.event_type();
		let snapshot = self.inner.snapshot.load();
		let handlers = snapshot
			.get(&event_type)
			.into_iter()
			.flatten()
			.filter_map(Weak::upgrade)
			.collect::<Vec<_>>();
		HandlerContext::new(&event, &handlers).next().await;
		Ok(())
	}

	fn start(&self) -> Result<(), Error> {
		let mut state = self.inner.state.lock();
		match *state {
			State::Running => return Ok(()),
			State::Stopping => return Err(Error::NotRunning),
			State::Starting | State::Stopped => *state = State::Running,
		}
		drop(state);
		self.inner.accepting.store(true, Ordering::Release);
		self.inner.publish();
		Ok(())
	}

	async fn stop(&self) {
		{
			let mut state = self.inner.state.lock();
			self.inner.accepting.store(false, Ordering::Release);
			match *state {
				State::Running | State::Stopping => {
					*state = State::Stopping;
				}
				State::Starting | State::Stopped => {
					self.inner.topics.lock().clear();
					self.inner.publish();
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

		self.inner.topics.lock().clear();
		*self.inner.state.lock() = State::Stopped;
		self.inner.publish();
	}
}

impl Inner {
	fn publish(&self) {
		let topics = self.topics.lock();
		let snapshot: Snapshot = topics
			.iter()
			.filter_map(|(event_type, topic)| {
				let mut handlers: Vec<(u32, Weak<dyn Handler>)> = Vec::new();
				topic.iter(|_, reg| {
					if reg.handler.strong_count() > 0 {
						handlers.push((reg.priority, reg.handler.clone()));
					}
				});
				if handlers.is_empty() {
					return None;
				}
				handlers.sort_by_key(|(p, _)| *p);
				Some((*event_type, handlers.into_iter().map(|(_, h)| h).collect()))
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
pub struct Service;

#[async_trait]
impl puniyu_service::Service for Service {
	fn name(&self) -> &str {
		pkg_name!()
	}

	fn version(&self) -> Version {
		pkg_version!()
	}

	async fn setup(&self, ctx: &ServiceContext) -> AnyError {
		let emitter = EventEmitter::new();
		emitter.start()?;
		if let Err(error) = ctx.provide(emitter.clone()) {
			emitter.stop().await;
			return Err(Box::new(error));
		}
		Ok(())
	}

	async fn cleanup(&self, ctx: &ServiceContext) -> AnyError {
		if let Some(emitter) = ctx.remove::<EventEmitter>() {
			emitter.stop().await;
		}
		Ok(())
	}
}
