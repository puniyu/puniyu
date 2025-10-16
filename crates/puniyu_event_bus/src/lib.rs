mod bus;

use puniyu_event_core::Event;
use puniyu_event_handler::{HandlerRegistry, MessageHandler};
use puniyu_event_matcher::{MatcherRegistry, MessageMatcher};
use std::sync::{Arc, Mutex, OnceLock};
use strum::{Display, EnumString, IntoStaticStr};
use tokio::sync::mpsc;

/// 事件类型枚举
#[derive(Debug, Clone, EnumString, Display, IntoStaticStr)]
pub enum EventType {
	#[strum(serialize = "message")]
	Message,
	#[strum(serialize = "notice")]
	Notice,
	#[strum(serialize = "request")]
	Request,
	#[strum(serialize = "unknown")]
	Unknown,
}

pub static EVENT_BUS: OnceLock<Arc<Mutex<EventBus>>> = OnceLock::new();
static HANDLER_REGISTRY: OnceLock<Arc<Mutex<HandlerRegistry>>> = OnceLock::new();
static MATCHER_REGISTRY: OnceLock<Arc<Mutex<MatcherRegistry>>> = OnceLock::new();

fn get_handler_registry() -> Arc<Mutex<HandlerRegistry>> {
	HANDLER_REGISTRY.get_or_init(|| Arc::new(Mutex::new(HandlerRegistry::new()))).clone()
}

fn get_matcher_registry() -> Arc<Mutex<MatcherRegistry>> {
	MATCHER_REGISTRY.get_or_init(|| Arc::new(Mutex::new(MatcherRegistry::new()))).clone()
}
pub type EventSender = mpsc::UnboundedSender<Event>;
pub type EventReceiver = mpsc::UnboundedReceiver<Event>;

pub struct EventBus {
	pub sender: EventSender,
	pub receiver: Arc<Mutex<Option<EventReceiver>>>,
}

impl Default for EventBus {
	fn default() -> Self {
		let handler_registry = get_handler_registry();
		let matcher_registry = get_matcher_registry();
		handler_registry.lock().unwrap().register(Arc::new(MessageHandler));
		matcher_registry.lock().unwrap().register(Arc::new(MessageMatcher));
		let (sender, receiver) = mpsc::unbounded_channel();
		Self { sender, receiver: Arc::new(Mutex::new(Some(receiver))) }
	}
}

impl EventBus {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn send_event(&self, event: Event) -> Result<(), Box<mpsc::error::SendError<Event>>> {
		self.sender.send(event).map_err(Box::new)
	}

	pub fn run(&mut self) {
		let receiver = self.receiver.lock().unwrap().take().unwrap();
		tokio::spawn(async move {
			let handlers = get_handler_registry().lock().unwrap().get_all();
			let matchers = get_matcher_registry().lock().unwrap().get_all();
			let mut receiver = receiver;
			while let Some(event) = receiver.recv().await {
				let is_matched = matchers.iter().any(|matcher| matcher.matches(&event));

				if is_matched {
					for handler in &handlers {
						handler.handle(&event).await;
					}
				}
			}
		});
	}

	pub fn stop(&self) {
		if let Some(receiver) = self.receiver.lock().unwrap().take() {
			drop(receiver);
		}
	}
}

pub fn init_event_bus() -> &'static Mutex<EventBus> {
	EVENT_BUS.get_or_init(|| Mutex::new(EventBus::default()).into())
}

pub fn send_event(event: Event) {
	let event_bus = EVENT_BUS.get().unwrap();
	event_bus.lock().unwrap().send_event(event).unwrap();
}

pub fn setup_event_bus(bus: Arc<Mutex<EventBus>>) {
	EVENT_BUS.get_or_init(|| bus);
}
