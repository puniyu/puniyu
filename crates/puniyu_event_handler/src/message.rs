use super::Handler;
use async_trait::async_trait;
use puniyu_command_builder::HandlerResult;
use puniyu_command_registry::CommandRegistry;
use puniyu_event_context::{Bot, EventContext, create_context_bot, create_event_context};
use puniyu_event_core::Event;
use puniyu_event_message::MessageEvent;

pub struct MessageHandler;

#[async_trait]
impl Handler for MessageHandler {
	async fn handle(&self, event: &Event, plugin_name: &[String], command_name: String) {
		if let Event::Message(adapter, message_event) = event {
			match message_event {
				MessageEvent::Friend(message) => {
					let bot = create_context_bot!(message.contact.clone(), adapter.clone());
					let event = create_event_context!(message.clone());

					for name in plugin_name {
						let func =
							CommandRegistry::get_with_plugin(name.as_str(), command_name.as_str());
						if let Some(command) = func {
							let result = command.builder.run(&bot, &event).await;
							match result {
								HandlerResult::Ok => break,
								HandlerResult::Continue => continue,
							}
						}
					}
				}
			}
		}
	}

	fn name(&self) -> &str {
		"message"
	}
}
