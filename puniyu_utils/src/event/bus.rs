use crate::event::handler::EventHandler;
use crate::event::matcher::Matcher;
use crate::event::message::{MessageBase, MessageEvent};
use puniyu_logger::info;
use std::sync::{Arc, Mutex, OnceLock};
use tokio::sync::mpsc;

pub type EventSender = mpsc::UnboundedSender<Event>;
pub type EventReceiver = mpsc::UnboundedReceiver<Event>;

pub enum Event {
	Message(MessageEvent),
}

#[derive(Debug, Clone)]
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

	pub fn send_event(&self, event: Event) -> Result<(), mpsc::error::SendError<Event>> {
		self.sender.send(event)
	}

	pub fn run(&mut self) {
		let receiver = self.receiver.lock().unwrap().take().unwrap();
		tokio::spawn(async move {
			let mut receiver = receiver;
			while let Some(event) = receiver.recv().await {
				match event {
					Event::Message(message) => match message {
						MessageEvent::Friend(message) => {
							info!("收到好友事件: {}", message);
						}
					},
				}
			}
		});
	}
}

pub static EVENT_BUS: OnceLock<Arc<Mutex<EventBus>>> = OnceLock::new();

pub fn init_event_bus() -> &'static Mutex<EventBus> {
	EVENT_BUS.get_or_init(|| Mutex::new(EventBus::default()).into())
}
