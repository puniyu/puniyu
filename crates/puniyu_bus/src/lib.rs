use puniyu_command::CommandHandler;
pub use puniyu_types::bus::{EVENT_BUS, EventBus, init_event_bus, send_event, stop_event_bus};
use puniyu_types::handler::{Handler, Matcher};

pub trait EventBusExt {
	fn run(&self);
}

impl EventBusExt for EventBus {
	fn run(&self) {
		let receiver_pair = self.take_receiver().expect("事件总线已经在运行中");
		let (mut receiver, mut shutdown_rx) = receiver_pair;

		tokio::spawn(async move {
			loop {
				tokio::select! {
					Some((bot, event)) = receiver.recv() => {
						if CommandHandler.matches(&event) {
							CommandHandler.handle(&bot, &event).await.expect("处理事件时出错");
						}

					}
					_ = shutdown_rx.recv() => {
						break;
					}
				}
			}
		});
	}
}
