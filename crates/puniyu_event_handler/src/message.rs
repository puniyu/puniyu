use super::Handler;
use async_trait::async_trait;
use puniyu_command_builder::HandlerResult;
use puniyu_command_registry::CommandRegistry;
use puniyu_event_context::{Bot, EventContext, create_context_bot, create_event_context};
use puniyu_event_core::Event;
use puniyu_event_message::{MessageBase, MessageEvent};
use std::collections::HashMap;

macro_rules! handle_message_command {
	($message:expr, $adapter:expr, $message_event:expr) => {{
		let bot = create_context_bot!($message.contact().clone(), $adapter.clone());

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
		let event = create_event_context!($message_event.clone(), plugin_args);

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
	}};
}

macro_rules! parse_command {
	($input:expr) => {{
		let mut args = $input.split_whitespace();
		let command_name = args.next().map(|s| s.to_string()).unwrap();

		let mut params = HashMap::new();
		let mut current_flag = None;

		for arg in args {
			match (arg.starts_with("--"), current_flag.take()) {
				(true, Some(prev_flag)) => {
					params.insert(prev_flag, None);
					current_flag = Some(arg[2..].to_string());
				}
				(true, None) => {
					current_flag = Some(arg[2..].to_string());
				}
				(false, Some(flag)) => {
					params.insert(flag, Some(arg.to_string()));
				}
				_ => {}
			}
		}

		if let Some(flag) = current_flag {
			params.insert(flag, None);
		}

		(command_name, params)
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
