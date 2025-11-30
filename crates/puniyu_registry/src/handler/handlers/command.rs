use crate::command::CommandRegistry;
use async_trait::async_trait;
use puniyu_logger::{info, owo_colors::OwoColorize};
use puniyu_types::bot::Bot;
use puniyu_types::command::{Arg, ArgMode, ArgType, ArgValue, HandlerAction};
use puniyu_types::context::{BotContext, MessageContext};
use puniyu_types::event::{
	Event, EventBase,
	message::{MessageBase, MessageEvent},
};
use puniyu_types::handler::Handler;
use puniyu_types::{create_context_bot, create_message_event_context};
use std::collections::HashMap;
use thiserror::Error;

fn arg_type_name(arg_type: ArgType) -> &'static str {
	match arg_type {
		ArgType::String => "字符串",
		ArgType::Int => "整数",
		ArgType::Float => "浮点数",
		ArgType::Bool => "布尔值",
	}
}

#[derive(Debug, Error)]
enum ValidateError {
	#[error("参数 {0} 未提供")]
	MissingRequired(String),
	#[error("参数 {name} 输入无效，请提供一个{}", arg_type_name(*.expected))]
	TypeMismatch { name: String, expected: ArgType },
}

enum ValidateResult {
	Ok(HashMap<String, ArgValue>),
	Err(ValidateError),
}

pub struct ParsedArgs {
	pub positional: Vec<String>,
	pub named: HashMap<String, String>,
}

pub struct Command;

impl Command {
	pub fn parse(input: &str) -> (String, ParsedArgs) {
		let mut tokens = input.split_whitespace();
		let command_name = tokens.next().map(|s| s.to_string()).unwrap_or_default();

		let mut positional = Vec::new();
		let mut named = HashMap::new();
		let mut current_flag: Option<String> = None;

		for token in tokens {
			if let Some(flag) = token.strip_prefix("--") {
				if let Some(prev_flag) = current_flag.take() {
					named.insert(prev_flag, String::new());
				}
				current_flag = Some(flag.to_string());
			} else if let Some(flag) = current_flag.take() {
				named.insert(flag, token.to_string());
			} else {
				positional.push(token.to_string());
			}
		}

		if let Some(flag) = current_flag {
			named.insert(flag, String::new());
		}

		(command_name, ParsedArgs { positional, named })
	}

	pub async fn handle(bot: Bot, message_type: MessageEvent) {
		let (bot_ctx, text_content, message_event) = match &message_type {
			MessageEvent::Friend(msg) => (
				create_context_bot!(bot.clone(), msg.contact().into()),
				Self::extract_text(msg.elements()),
				MessageEvent::Friend(msg.clone()),
			),
			MessageEvent::Group(msg) => (
				create_context_bot!(bot.clone(), msg.contact().into()),
				Self::extract_text(msg.elements()),
				MessageEvent::Group(msg.clone()),
			),
		};

		let (command_name, command_args) = Self::parse(&text_content);

		let plugin_args = match Self::validate_args(&command_name, &command_args) {
			ValidateResult::Ok(args) => args,
			ValidateResult::Err(e) => {
				let _ = bot_ctx.reply(e.to_string().into()).await;
				return;
			}
		};

		let event = create_message_event_context!(message_event, plugin_args);
		Self::execute_plugins(&bot_ctx, &event, &command_name).await;
	}

	fn extract_text(elements: Vec<puniyu_types::element::receive::Elements>) -> String {
		elements.iter().filter_map(|e| e.as_text()).collect::<Vec<_>>().join(" ")
	}
	fn validate_args(command_name: &str, parsed: &ParsedArgs) -> ValidateResult {
		let Some(command) = CommandRegistry::get_with_name(command_name) else {
			return ValidateResult::Ok(HashMap::new());
		};

		let arg_definitions = command.builder.args();
		let mut args = HashMap::new();
		let mut pos_index = 0;

		for arg_def in &arg_definitions {
			let value = match arg_def.mode {
				ArgMode::Positional => {
					let val = parsed.positional.get(pos_index).cloned();
					if val.is_some() {
						pos_index += 1;
					}
					val
				}
				ArgMode::Named => parsed.named.get(arg_def.name).cloned(),
			};

			match Self::parse_arg_value(arg_def, value.as_ref()) {
				Ok(Some(val)) => {
					args.insert(arg_def.name.to_string(), val);
				}
				Ok(None) if arg_def.required => {
					return ValidateResult::Err(ValidateError::MissingRequired(
						arg_def.name.to_string(),
					));
				}
				Ok(None) => {}
				Err(_) => {
					return ValidateResult::Err(ValidateError::TypeMismatch {
						name: arg_def.name.to_string(),
						expected: arg_def.arg_type,
					});
				}
			}
		}

		ValidateResult::Ok(args)
	}

	fn parse_arg_value(arg_def: &Arg, value: Option<&String>) -> Result<Option<ArgValue>, String> {
		match value {
			Some(v) => {
				let parsed = match arg_def.arg_type {
					ArgType::Int => v.parse::<i64>().map(ArgValue::Int).map_err(|_| v.clone())?,
					ArgType::Float => {
						v.parse::<f64>().map(ArgValue::Float).map_err(|_| v.clone())?
					}
					ArgType::Bool => match v.to_lowercase().as_str() {
						"true" | "1" | "yes" | "" => ArgValue::Bool(true),
						"false" | "0" | "no" => ArgValue::Bool(false),
						_ => return Err(v.clone()),
					},
					ArgType::String => ArgValue::String(v.clone()),
				};
				Ok(Some(parsed))
			}
			None => Ok(arg_def.default.clone()),
		}
	}

	async fn execute_plugins(bot: &BotContext, event: &MessageContext, command_name: &str) {
		let plugins = CommandRegistry::get_plugins(command_name);
		for name in plugins {
			let func = CommandRegistry::get_with_plugin(&name, command_name);
			if let Some(command) = func {
				let start_time = std::time::Instant::now();
				info!("{} 开始执行", format!("[command:{}:{}]", &name, command_name).yellow());
				let result = command.builder.run(bot, event).await;
				info!(
					"{} 执行完毕, 耗时{}ms",
					format!("[command:{}:{}]", &name, command_name).yellow(),
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
	}
}

pub struct CommandHandler;

#[async_trait]
impl Handler for CommandHandler {
	fn name(&self) -> &str {
		"command"
	}

	async fn handle(&self, bot: Bot, event: Event) {
		if let Event::Message(message_event) = event {
			let message_type = match message_event.as_ref() {
				MessageEvent::Friend(message) => MessageEvent::Friend(message.clone()),
				MessageEvent::Group(message) => MessageEvent::Group(message.clone()),
			};
			Command::handle(bot, message_type).await;
		}
	}
}
