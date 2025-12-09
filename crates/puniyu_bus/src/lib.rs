use puniyu_handler_command::CommandHandler;
use puniyu_logger::{owo_colors::OwoColorize, warn};
use puniyu_matcher_command::CommandMatcher;
use puniyu_types::bot::Bot;
use puniyu_types::event::Event;
use puniyu_types::handler::Handler;
use puniyu_types::matcher::Matcher;
use std::sync::{Arc, Mutex, OnceLock};
use tokio::runtime::Handle;
use tokio::sync::mpsc;

pub static EVENT_BUS: OnceLock<Arc<EventBus>> = OnceLock::new();

type EventSender = mpsc::Sender<(Bot, Event)>;
type EventReceiver = mpsc::Receiver<(Bot, Event)>;
type ReceiverPair = (EventReceiver, mpsc::UnboundedReceiver<()>);

pub struct EventBus {
	sender: EventSender,
	shutdown_tx: mpsc::UnboundedSender<()>,
	receiver: Arc<Mutex<Option<ReceiverPair>>>,
	handle: Handle,
}

impl EventBus {
	fn new() -> Self {
		let (sender, receiver) = mpsc::channel(5000);
		let (shutdown_tx, shutdown_rx) = mpsc::unbounded_channel();
		Self {
			sender,
			shutdown_tx,
			receiver: Arc::new(Mutex::new(Some((receiver, shutdown_rx)))),
			handle: Handle::current(),
		}
	}

	fn send_event(&self, bot: Bot, event: Event) {
		let sender = self.sender.clone();
		self.handle.spawn(async move {
			if let Err(e) = sender.send((bot, event)).await {
				warn!("[{}]: 事件发送失败 {:?}", "Event".blue(), e);
			}
		});
	}

	pub fn run(&self) {
		let receiver_pair = self
			.receiver
			.lock()
			.unwrap()
			.take()
			.expect("事件总线已经在运行中");
		let (mut receiver, mut shutdown_rx) = receiver_pair;

		tokio::spawn(async move {
			loop {
				tokio::select! {
					Some((bot, event)) = receiver.recv() => {
						if let Some(result) = CommandMatcher.matches(&event) {
							CommandHandler.handle(bot, event, Some(result)).await;
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
