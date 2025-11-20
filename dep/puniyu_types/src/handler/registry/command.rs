use super::Handler;
use async_trait::async_trait;
use crate::adapter::AdapterApi;
use crate::command::{CommandRegistry, HandlerResult};
use crate::event::{message::{MessageEvent, MessageBase}, Event, EventBase};
use crate::context::{BotContext, MessageContext};
use crate::contact::ContactType;
use crate::{create_context_bot, create_message_event_context};
use std::collections::HashMap;

/// TODO: 此结构体需重构, 宏简化
macro_rules! handle_command {
	($message:expr, $adapter:expr, $message_event:expr) => {{
		let bot = create_context_bot!($message.contact().into(), $adapter);

		let (command_name, command_args) = {
			let text_content = $message
				.elements()
				.iter()
				.filter_map(|element| element.as_text())
				.collect::<Vec<_>>()
				.join(" ");
			parse_command!(&text_content)
		};

		let plugin_args =
			if let Some(command) = CommandRegistry::get_with_name(command_name.as_str()) {
				let arg_definitions = command.builder.args();
				let mut args_map = HashMap::new();
				for arg_name in arg_definitions {
					let value = command_args.get(arg_name.as_str()).cloned().flatten();
					args_map.insert(arg_name.clone(), value);
				}
				args_map
			} else {
				HashMap::new()
			};
		let event = create_message_event_context!($message_event, plugin_args);

		let plugins = CommandRegistry::get_plugins(command_name.as_str());
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
	}};
}

macro_rules! parse_command {
	($input:expr) => {{
		let mut args = $input.split_whitespace();
		let command_name = args.next().map(|s| s.to_string()).unwrap_or_default();

		let mut params = HashMap::new();
		let mut current_flag: Option<String> = None;

		for arg in args {
			if let Some(flag) = arg.strip_prefix("--") {
				if let Some(prev_flag) = current_flag.take() {
					params.insert(prev_flag, None);
				}
				current_flag = Some(flag.to_string());
			} else if let Some(flag) = current_flag.take() {
				params.insert(flag, Some(arg.to_string()));
			}
		}

		if let Some(flag) = current_flag {
			params.insert(flag, None);
		}

		(command_name, params)
	}};
}

pub struct CommandHandler;

#[async_trait]
impl Handler for CommandHandler {
	async fn handle(&self, adapter: &'static dyn AdapterApi, event: Event) {
		if let Event::Message(message_event) = event {
			match message_event.as_ref() {
				MessageEvent::Friend(message) => {
					handle_command!(message, ContactType::Friend(adapter), *message_event);
				}
				MessageEvent::Group(message) => {
					handle_command!(message, adapter, *message_event);
				}
			}
		}
	}

	fn name(&self) -> &str {
		"message"
	}
}
