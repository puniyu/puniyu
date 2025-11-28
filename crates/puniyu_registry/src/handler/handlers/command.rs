use crate::command::CommandRegistry;
use async_trait::async_trait;
use puniyu_types::bot::Bot;
use puniyu_types::command::HandlerAction;
use puniyu_types::context::{BotContext, MessageContext};
use puniyu_types::event::{
	Event, EventBase,
	message::{MessageBase, MessageEvent},
};
use puniyu_types::handler::Handler;
use puniyu_types::{create_context_bot, create_message_event_context};

use puniyu_logger::{info, owo_colors::OwoColorize};
use std::collections::HashMap;

/// TODO: 此结构体需重构, 宏简化
macro_rules! handle_command {
	($bot:expr, $message:expr, $message_type:ident) => {{
		let bot = create_context_bot!($bot, $message.contact().into());

		let (command_name, command_args) = {
			let text_content = $message
				.elements()
				.iter()
				.filter_map(|element| element.as_text())
				.collect::<Vec<_>>()
				.join(" ");
			parse_command!(&text_content)
		};

		let plugin_args = if let Some(command) = CommandRegistry::get_with_name(&command_name) {
			let arg_definitions = command.builder.args();
			let mut args_map = HashMap::new();
			for arg_name in arg_definitions {
				if let Some(value) = command_args.get(arg_name).cloned().flatten() {
					args_map.insert(arg_name.to_string(), value);
				}
			}
			args_map
		} else {
			HashMap::new()
		};
		let event = create_message_event_context!(
			MessageEvent::$message_type($message.clone()),
			plugin_args
		);

		let plugins = CommandRegistry::get_plugins(&command_name);
		for name in plugins {
			println!("{}", &name);
			let func = CommandRegistry::get_with_plugin(&name, &command_name);
			if let Some(command) = func {
				let start_time = std::time::Instant::now();
				info!("{} 开始执行", format!("[command:{}:{}]", &name, &command_name).yellow());
				let result = command.builder.run(&bot, &event).await;
				info!(
					"{} 执行完毕, 耗时{}ms",
					format!("[command:{}:{}]", &name, &command_name).yellow(),
					start_time.elapsed().as_millis()
				);
				match result {
					Ok(action) => match action {
						HandlerAction::Done => break,
						HandlerAction::Continue => continue,
					},
					Err(_) => continue,
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
	fn name(&self) -> &str {
		"message"
	}

	async fn handle(&self, bot: Bot, event: Event) {
		if let Event::Message(message_event) = event {
			match message_event.as_ref() {
				MessageEvent::Friend(message) => {
					handle_command!(bot, message, Friend);
				}
				MessageEvent::Group(message) => {
					handle_command!(bot, message, Group);
				}
			}
		}
	}
}
