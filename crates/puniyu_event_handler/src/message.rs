use super::Handler;
use async_trait::async_trait;
use puniyu_command_builder::HandlerResult;
use puniyu_command_registry::CommandRegistry;
use puniyu_event_context::{Bot, EventContext, create_context_bot, create_event_context};
use puniyu_event_core::Event;
use puniyu_event_message::{MessageBase, MessageEvent};

macro_rules! handle_message_command {
	($message:expr, $adapter:expr, $message_event:expr) => {{
		let bot = create_context_bot!($message.contact().clone(), $adapter.clone());
		let event = create_event_context!($message_event.clone());

		let command_name =
			$message.elements().iter().find_map(|element| element.as_text().map(|s| s.to_string()));

		if let Some(command_name) = command_name {
			let plugins = CommandRegistry::get_plugins_with_command(command_name.as_str());
			for name in plugins {
				let func = CommandRegistry::get_with_plugin(name.as_str(), command_name.as_str());
				if let Some(command) = func {
					let result = command.builder.run(&bot, &event).await;
					match result {
						HandlerResult::Ok => break,
						HandlerResult::Continue => continue,
					}
				}
			}
		}
	}};
}

pub struct MessageHandler;

#[async_trait]
impl Handler for MessageHandler {
	async fn handle(&self, event: &Event) {
		if let Event::Message(adapter, message_event) = event {
			match message_event {
				MessageEvent::Friend(message) => {
					handle_message_command!(message, adapter, message_event);
				}
				MessageEvent::Group(message) => {
					handle_message_command!(message, adapter, message_event);
				}
			}
		}
	}

	fn name(&self) -> &str {
		"message"
	}
}
