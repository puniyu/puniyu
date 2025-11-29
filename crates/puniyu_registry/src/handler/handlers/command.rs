use crate::command::CommandRegistry;
use async_trait::async_trait;
use puniyu_types::bot::Bot;
use puniyu_types::command::{Arg, ArgType, ArgValue, HandlerAction};
use puniyu_types::context::{BotContext, MessageContext};
use puniyu_types::event::{
    Event, EventBase,
    message::{FriendMessage, GroupMessage, MessageBase, MessageEvent},
};
use puniyu_types::handler::Handler;
use puniyu_types::{create_context_bot, create_message_event_context};

use puniyu_logger::{info, owo_colors::OwoColorize};
use std::collections::HashMap;

pub enum MessageType {
	Friend(FriendMessage),
	Group(GroupMessage),
}

/// 参数校验结果
enum ValidateResult {
	/// 校验通过
	Ok(HashMap<String, ArgValue>),
	/// 缺少必填参数
	MissingRequired(String),
	/// 类型错误
	TypeError { name: String, expected: ArgType },
}

fn arg_type_name(arg_type: ArgType) -> &'static str {
	match arg_type {
		ArgType::String => "字符串",
		ArgType::Int => "整数",
		ArgType::Float => "浮点数",
		ArgType::Bool => "布尔值",
	}
}

pub struct Command;

impl Command {
	pub fn parse(input: &str) -> (String, HashMap<String, String>) {
		let mut args = input.split_whitespace();
		let command_name = args.next().map(|s| s.to_string()).unwrap_or_default();

		let mut params = HashMap::new();
		let mut current_flag: Option<String> = None;

		for arg in args {
			if let Some(flag) = arg.strip_prefix("--") {
				if let Some(prev_flag) = current_flag.take() {
					params.insert(prev_flag, String::new());
				}
				current_flag = Some(flag.to_string());
			} else if let Some(flag) = current_flag.take() {
				params.insert(flag, arg.to_string());
			}
		}

		if let Some(flag) = current_flag {
			params.insert(flag, String::new());
		}

		(command_name, params)
	}

	pub async fn handle(bot: Bot, message_type: MessageType) {
		match message_type {
			MessageType::Friend(message) => {
				let bot = create_context_bot!(bot, message.contact().into());

				let (command_name, command_args) = {
					let text_content = message
						.elements()
						.iter()
						.filter_map(|element| element.as_text())
						.collect::<Vec<_>>()
						.join(" ");
					Self::parse(&text_content)
				};

				let plugin_args = match Self::validate_args(&command_name, &command_args) {
					ValidateResult::Ok(args) => args,
					ValidateResult::MissingRequired(name) => {
						let _ = bot.reply(format!("参数 {} 未提供", name).into()).await;
						return;
					}
					ValidateResult::TypeError { name, expected } => {
						let _ = bot.reply(format!(
							"参数 {} 输入无效，请提供一个{}",
							name, arg_type_name(expected)
						).into()).await;
						return;
					}
				};

				let event = create_message_event_context!(
					MessageEvent::Friend(message),
					plugin_args
				);

				Self::execute_plugins(&bot, &event, &command_name).await;
			}
			MessageType::Group(message) => {
				let bot = create_context_bot!(bot, message.contact().into());

				let (command_name, command_args) = {
					let text_content = message
						.elements()
						.iter()
						.filter_map(|element| element.as_text())
						.collect::<Vec<_>>()
						.join(" ");
					Self::parse(&text_content)
				};

				let plugin_args = match Self::validate_args(&command_name, &command_args) {
					ValidateResult::Ok(args) => args,
					ValidateResult::MissingRequired(name) => {
						let _ = bot.reply(format!("参数 {} 未提供", name).into()).await;
						return;
					}
					ValidateResult::TypeError { name, expected } => {
						let _ = bot.reply(format!(
							"参数 {} 输入无效，请提供一个{}",
							name, arg_type_name(expected)
						).into()).await;
						return;
					}
				};

				let event = create_message_event_context!(
					MessageEvent::Group(message),
					plugin_args
				);

				Self::execute_plugins(&bot, &event, &command_name).await;
			}
		}
	}

	fn validate_args(
		command_name: &str,
		command_args: &HashMap<String, String>,
	) -> ValidateResult {
		let Some(command) = CommandRegistry::get_with_name(command_name) else {
			return ValidateResult::Ok(HashMap::new());
		};

		let arg_definitions = command.builder.args();
		println!("[DEBUG] command: {}, args: {:?}, defs: {:?}", command_name, command_args, arg_definitions.iter().map(|a| (a.name, a.required)).collect::<Vec<_>>());
		let mut args = HashMap::new();

		for arg_def in arg_definitions {
			let value = command_args.get(arg_def.name);

			match Self::parse_arg_value(&arg_def, value) {
				Ok(Some(val)) => {
					args.insert(arg_def.name.to_string(), val);
				}
				Ok(None) => {
					if arg_def.required {
						return ValidateResult::MissingRequired(arg_def.name.to_string());
					}
				}
				Err(_) => {
					return ValidateResult::TypeError {
						name: arg_def.name.to_string(),
						expected: arg_def.arg_type,
					};
				}
			}
		}

		ValidateResult::Ok(args)
	}

	/// 解析单个参数值
	fn parse_arg_value(arg_def: &Arg, value: Option<&String>) -> Result<Option<ArgValue>, String> {
		match value {
			Some(v) => {
				let parsed = match arg_def.arg_type {
					ArgType::Int => {
						v.parse::<i64>()
							.map(ArgValue::Int)
							.map_err(|_| v.clone())?
					}
					ArgType::Float => {
						v.parse::<f64>()
							.map(ArgValue::Float)
							.map_err(|_| v.clone())?
					}
					ArgType::Bool => {
						match v.to_lowercase().as_str() {
							"true" | "1" | "yes" | "" => ArgValue::Bool(true),
							"false" | "0" | "no" => ArgValue::Bool(false),
							_ => return Err(v.clone()),
						}
					}
					ArgType::String => ArgValue::String(v.clone()),
				};
				Ok(Some(parsed))
			}
			None => Ok(arg_def.default.clone()),
		}
	}

	/// 执行插件
	async fn execute_plugins(bot: &BotContext, event: &MessageContext, command_name: &str) {
		let plugins = CommandRegistry::get_plugins(command_name);
		for name in plugins {
			println!("{}", &name);
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
		"message"
	}

	async fn handle(&self, bot: Bot, event: Event) {
		if let Event::Message(message_event) = event {
			let message_type = match message_event.as_ref() {
				MessageEvent::Friend(message) => MessageType::Friend(message.clone()),
				MessageEvent::Group(message) => MessageType::Group(message.clone()),
			};
			Command::handle(bot, message_type).await;
		}
	}
}
