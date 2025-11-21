use puniyu_logger::owo_colors::OwoColorize;
use puniyu_logger::warn;
use puniyu_registry::handler::CommandHandler;
use puniyu_registry::matcher::CommandMatcher;
use puniyu_types::bot::Bot;
use puniyu_types::event::Event;
use puniyu_types::handler::Handler;
use puniyu_types::matcher::Matcher;
use std::sync::{Arc, Mutex, OnceLock};
use tokio::sync::mpsc;

pub static EVENT_BUS: OnceLock<Arc<EventBus>> = OnceLock::new();

type EventSender = mpsc::UnboundedSender<(Bot, Event)>;
type EventReceiver = mpsc::UnboundedReceiver<(Bot, Event)>;
type ReceiverPair = (EventReceiver, mpsc::UnboundedReceiver<()>);

pub struct EventBus {
	sender: EventSender,
	shutdown_tx: mpsc::UnboundedSender<()>,
	receiver: Arc<Mutex<Option<ReceiverPair>>>,
}

impl EventBus {
	fn new() -> Self {
		let (sender, receiver) = mpsc::unbounded_channel();
		let (shutdown_tx, shutdown_rx) = mpsc::unbounded_channel();
		Self { sender, shutdown_tx, receiver: Arc::new(Mutex::new(Some((receiver, shutdown_rx)))) }
	}

	fn send_event(&self, bot: Bot, event: Event) {
		if let Err(e) = self.sender.send((bot, event)) {
			warn!("[{}]: 事件发送失败 {:?}", "Event".blue(), e);
		}
	}

	pub fn run(&self) {
		let receiver_pair = self.receiver.lock().unwrap().take().expect("事件总线已经在运行中");
		let (mut receiver, mut shutdown_rx) = receiver_pair;

		tokio::spawn(async move {
			loop {
				tokio::select! {
					Some((bot, event)) = receiver.recv() => {
						if CommandMatcher.matches(&event) {
							CommandHandler.handle(bot, event).await;
						}
					}
					_ = shutdown_rx.recv() => {
						break;
					}
				}
			}
		});
	}

	pub fn stop(&self) {
		let _ = self.shutdown_tx.send(());
	}
}

pub fn init_event_bus() {
	let bus = EventBus::new();
	EVENT_BUS.set(Arc::from(bus)).ok();
}

pub fn send_event(bot: Bot, event: Event) {
	if let Some(bus) = EVENT_BUS.get() {
		bus.send_event(bot, event);
	}
}

pub fn stop_event_bus() {
	if let Some(bus) = EVENT_BUS.get() {
		bus.stop();
	}
}


pub fn setup_event_bus(bus: Arc<EventBus>) {
	EVENT_BUS.get_or_init(|| bus);
}