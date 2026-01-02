mod tools;

use puniyu_command::CommandHandler;
use puniyu_logger::owo_colors::OwoColorize;
use puniyu_logger::warn;
use puniyu_registry::HandlerRegistry;
use puniyu_types::bus::{EVENT_BUS, EventBus as EventBusTrait, EventReceiver, EventSender};
use puniyu_types::event::Event;
use std::sync::{Arc, Mutex};
use tokio::runtime::Handle;
use tokio::sync::{broadcast, mpsc};
use tokio::task::JoinHandle;
use tools::dispatch_event;

pub struct EventBus {
	sender: EventSender,
	receiver: Arc<Mutex<Option<EventReceiver>>>,
	shutdown_tx: broadcast::Sender<()>,
	handle: Handle,
}

impl EventBus {
	pub fn new() -> Self {
		let (sender, receiver) = mpsc::channel(5000);
		let (shutdown_tx, _) = broadcast::channel(1);
		Self {
			sender,
			receiver: Arc::new(Mutex::new(Some(receiver))),
			shutdown_tx,
			handle: Handle::current(),
		}
	}
}

impl Default for EventBus {
	fn default() -> Self {
		Self::new()
	}
}

impl EventBusTrait for EventBus {
	fn run(&self) -> JoinHandle<()> {
		let receiver = self.receiver.clone();
		let mut shutdown_rx = self.shutdown_tx.subscribe();

		self.handle.spawn(async move {
			let receiver = {
				let mut guard = receiver.lock().unwrap();
				guard.take()
			};
			let Some(mut receiver) = receiver else {
				return;
			};

			loop {
				tokio::select! {
					_ = shutdown_rx.recv() => {
						warn!("事件总线已关闭");
						break;
					}

					event_pair = receiver.recv() => {
						match event_pair {
							Some(event) => {
								dispatch_event(&event).await;
							}
							None => {
								warn!("[{}]: 事件通道已关闭", "Event".blue());
								break;
							}
						}
					}
				}
			}
		})
	}

	fn send_event(&self, event: Arc<Event>) {
		let sender = self.sender.clone();
		self.handle.spawn(async move {
			if let Err(e) = sender.send(event).await {
				warn!("[{}]: 事件发送失败: {:?}", "Event".blue(), e);
			}
		});
	}

	fn shutdown(&self) {
		let _ = self.shutdown_tx.send(());
	}
}

pub fn init_event_bus() {
	HandlerRegistry::register(Arc::new(CommandHandler));
	EVENT_BUS.get_or_init(|| {
		let event_bus = EventBus::new();
		Arc::new(event_bus) as Arc<dyn EventBusTrait>
	});
	let event_bus = EVENT_BUS.get().unwrap();
	event_bus.run();
}
