use arc_swap::ArcSwap;
use async_trait::async_trait;
use parking_lot::Mutex;
use puniyu_api::{pkg_name, pkg_version};
use puniyu_context::PluginContext;
use puniyu_error::AnyError;
use puniyu_event::{Event, EventType, message::MessageEvent};
use puniyu_handler::{Handler, HandlerContext};
use puniyu_logger::owo_colors::OwoColorize;
use semver::Version;
use smol_str::SmolStr;
use std::collections::{BTreeMap, HashMap};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Weak};
use std::time::Instant;
use thiserror::Error;
use tokio::sync::Notify;


#[derive(Debug, Error)]
pub enum Error {
	#[error("event emitter is not running")]
	NotRunning,
	#[error("handler '{handler}' is already listening to '{event_type}'")]
	AlreadyListening { event_type: EventType, handler: SmolStr },
	#[error("event emitter handler id exhausted")]
	IdExhausted,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct HandlerId(u64);

impl HandlerId {
	fn next(self) -> Option<Self> {
		self.0.checked_add(1).map(Self)
	}
}

struct Registration {
	name: SmolStr,
	handler: Weak<dyn Handler>,
}

#[derive(Default)]
struct Topic {
	entries: BTreeMap<(u32, HandlerId), Registration>,
}

impl Topic {
	fn prune(&mut self) {
		self.entries.retain(|_, entry| entry.handler.strong_count() > 0);
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
	next_id: Option<HandlerId>,
	topics: HashMap<EventType, Topic>,
}

impl Default for Registry {
	fn default() -> Self {
		Self { state: EventState::Idle, next_id: Some(HandlerId(0)), topics: HashMap::new() }
	}
}

impl Registry {
	fn allocate_id(&mut self) -> Result<HandlerId, Error> {
		let id = self.next_id.ok_or(Error::IdExhausted)?;
		self.next_id = id.next();
		Ok(id)
	}
}

type Snapshot = HashMap<EventType, Vec<Weak<dyn Handler>>>;

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
		handler: impl Into<Arc<dyn Handler>>,
	) -> Result<(), Error> {
		let handler = handler.into();
		let name = SmolStr::new(handler.name());
		let mut registry = self.inner.registry.lock();
		if registry.state != EventState::Running {
			return Err(Error::NotRunning);
		}

		{
			let topic = registry.topics.entry(event_type).or_default();
			topic.prune();
			if topic.entries.values().any(|entry| entry.name == name) {
				return Err(Error::AlreadyListening { event_type, handler: name });
			}
		}

		let id = registry.allocate_id()?;
		registry.topics.entry(event_type).or_default().entries.insert(
			(handler.priority(), id),
			Registration { name, handler: Arc::downgrade(&handler) },
		);
		self.inner.publish(&registry);
		Ok(())
	}

	pub fn off(&self, event_type: EventType, handler: impl Into<Arc<dyn Handler>>) {
		let handler = handler.into();
		let target = Arc::downgrade(&handler);
		let mut registry = self.inner.registry.lock();
		let remove_topic = if let Some(topic) = registry.topics.get_mut(&event_type) {
			topic.entries.retain(|_, entry| {
				entry.handler.strong_count() > 0 && !Weak::ptr_eq(&entry.handler, &target)
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
				let handlers = topic
					.entries
					.values()
					.filter(|entry| entry.handler.strong_count() > 0)
					.map(|entry| entry.handler.clone())
					.collect::<Vec<_>>();
				(!handlers.is_empty()).then_some((*event_type, handlers))
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

	async fn on_load(&self, ctx: &PluginContext) -> AnyError {
		let emitter = ctx.require::<EventEmitter>()?;
		let handler: Arc<dyn Handler> = Arc::new(EventLog);
		emitter.on(EventType::Message, Arc::clone(&handler))?;
		if let Err(error) =
			ctx.provide(Arc::new(EventLogInner { handler: Arc::clone(&handler) }))
		{
			emitter.off(EventType::Message, Arc::clone(&handler));
			return Err(Box::new(error));
		}
		Ok(())
	}

	async fn on_unload(&self, ctx: &PluginContext) -> AnyError {
		let emitter = ctx.require::<EventEmitter>()?;
		if let Some(inner) = ctx.remove::<Arc<EventLogInner>>() {
			emitter.off(EventType::Message, Arc::clone(&inner.handler));
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

struct EventLogInner {
	handler: Arc<dyn Handler>,
}

#[derive(Debug, Default, Clone, Copy)]
struct EventLog;

#[async_trait]
impl Handler for EventLog {
	fn name(&self) -> &'static str {
		"event_log"
	}

	fn priority(&self) -> u32 {
		100
	}

	async fn handle(&self, mut ctx: HandlerContext<'_>) {
		let Some(message) = ctx.as_message() else {
			ctx.next().await;
			return;
		};

		let event_id = message.event_id().to_owned();
		log::info!("{}", format_message(message));
		let started_at = Instant::now();
		ctx.next().await;
		let elapsed = started_at.elapsed().as_millis();
		log::info!(
			"[{}:{}] 处理完成, 耗时{}ms",
			"Event".yellow(),
			event_id.green(),
			elapsed
		);
	}
}

fn format_message(event: &MessageEvent) -> String {
	use puniyu_event::EventBase;

	let raw_message =
		event.elements().iter().map(format_element).collect::<Vec<_>>().join("");

	if let Some(event) = event.as_group() {
		return format!(
			"[{}:{}][{}:{}-{}]: {}",
			"Bot".yellow(),
			event.self_id().green(),
			"GroupMessage".yellow(),
			event.group_id().green(),
			event.user_id().green(),
			raw_message
		);
	}
	if let Some(event) = event.as_group_temp() {
		return format!(
			"[{}:{}][{}:{}-{}]: {}",
			"Bot".yellow(),
			event.self_id().green(),
			"GroupTempMessage".yellow(),
			event.group_id().green(),
			event.user_id().green(),
			raw_message
		);
	}
	if let Some(event) = event.as_guild() {
		return format!(
			"[{}:{}][{}:{}-{}]: {}",
			"Bot".yellow(),
			event.self_id().green(),
			"GuildMessage".yellow(),
			event.guild_id().green(),
			event.user_id().green(),
			raw_message
		);
	}

	format!(
		"[{}:{}][{}:{}]: {}",
		"Bot".yellow(),
		event.self_id().green(),
		"FriendMessage".yellow(),
		event.user_id().green(),
		raw_message
	)
}

fn format_element(element: &puniyu_element::receive::Elements) -> String {
	match element {
		puniyu_element::receive::Elements::Text(value) => format!("text:{}", value.text),
		puniyu_element::receive::Elements::At(value) => format!("at:{}", value.target_id),
		puniyu_element::receive::Elements::Reply(value) => format!("reply:{}", value.message_id),
		puniyu_element::receive::Elements::Face(value) => format!("face:{}", value.id),
		puniyu_element::receive::Elements::Image(value) => {
			format!("image:{}", value.summary.as_deref().unwrap_or(value.file_name.as_str()))
		}
		puniyu_element::receive::Elements::File(value) => format!("file:{}", value.file_name),
		puniyu_element::receive::Elements::Video(value) => format!("video:{}", value.file_name),
		puniyu_element::receive::Elements::Record(value) => format!("record:{}", value.file_name),
		puniyu_element::receive::Elements::Json(value) => format!("json:{}", value.data),
		puniyu_element::receive::Elements::Xml(value) => format!("xml:{}", value.data),
	}
}
