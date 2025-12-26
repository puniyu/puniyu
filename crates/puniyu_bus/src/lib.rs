use puniyu_handler_command::CommandHandler;
use puniyu_matcher_command::CommandMatcher;
use puniyu_types::handler::Handler;
use puniyu_types::matcher::Matcher;
pub use puniyu_types::bus::{EVENT_BUS, EventBus, init_event_bus, send_event, stop_event_bus};

/// EventBus 的运行时扩展
pub trait EventBusExt {
	fn run(&self);
}

impl EventBusExt for EventBus {
	fn run(&self) {
		let receiver_pair = self
			.take_receiver()
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
}
