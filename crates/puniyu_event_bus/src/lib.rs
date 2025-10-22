use puniyu_adapter_api::AdapterApi;
use puniyu_event::Event;
use puniyu_handler::{Handler, MessageHandler};
use puniyu_logger::owo_colors::OwoColorize;
use puniyu_logger::warn;
use puniyu_matcher::{Matcher, MessageMatcher};
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

pub type EventSender = mpsc::UnboundedSender<(Arc<dyn AdapterApi>, Event)>;

pub type EventReceiver = mpsc::UnboundedReceiver<(Arc<dyn AdapterApi>, Event)>;

pub struct EventBus {
	pub sender: EventSender,
	pub receiver: Arc<Mutex<Option<EventReceiver>>>,
}

impl Default for EventBus {
	fn default() -> Self {
		let (sender, receiver) = mpsc::unbounded_channel();
		Self { sender, receiver: Arc::new(Mutex::new(Some(receiver))) }
	}
}

impl EventBus {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn send_event(
		&self,
		adapter: Arc<dyn AdapterApi>,
		event: Event,
	) -> Result<(), Box<mpsc::error::SendError<(Arc<dyn AdapterApi>, Event)>>> {
		self.sender.send((Arc::from(adapter), event)).map_err(|e| {
			warn!("[{}]: 事件发送失败 {:?}", "Event".blue(), e);
			Box::new(e)
		})
	}

	pub fn run(&mut self) {
		let receiver = self.receiver.lock().unwrap().take().unwrap();
		tokio::spawn(async move {
			let mut receiver = receiver;
			while let Some((adapter, event)) = receiver.recv().await {
				if MessageMatcher.matches(&event) {
					MessageHandler.handle(&event).await;
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

pub fn send_event(adapter: Arc<dyn AdapterApi>, event: Event) {
	let event_bus = EVENT_BUS.get().unwrap();
	event_bus.lock().unwrap().send_event(adapter, event).unwrap();
}

pub fn setup_event_bus(bus: Arc<Mutex<EventBus>>) {
	EVENT_BUS.get_or_init(|| bus);
}
