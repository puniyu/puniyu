use crate::command::CommandRegistry;
use async_trait::async_trait;
use clap::builder::ValueParser;
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

struct ClapArg<'a>(&'a Arg);

impl From<ClapArg<'_>> for clap::Arg {
	fn from(ClapArg(arg): ClapArg<'_>) -> Self {
		let mut clap_arg = clap::Arg::new(arg.name);

		clap_arg = match arg.arg_type {
			ArgType::String => clap_arg.value_parser(ValueParser::string()),
			ArgType::Int => clap_arg.value_parser(clap::value_parser!(i64)),
			ArgType::Float => clap_arg.value_parser(clap::value_parser!(f64)),
			ArgType::Bool => clap_arg.value_parser(clap::value_parser!(bool)).num_args(0..=1).default_missing_value("true"),
		};


		clap_arg = match arg.mode {
			ArgMode::Positional => clap_arg.required(arg.required),
			ArgMode::Named => clap_arg.long(arg.name).required(arg.required),
		};

		if let Some(ref default) = arg.default {
			clap_arg = clap_arg.default_value(match default {
				ArgValue::String(s) => s.clone(),
				ArgValue::Int(i) => i.to_string(),
				ArgValue::Float(f) => f.to_string(),
				ArgValue::Bool(b) => b.to_string(),
			});
		}

		clap_arg
	}
}

pub struct Command;

impl Command {
	fn to_clap_command(name: &str, args: &[Arg]) -> clap::Command {
		let mut cmd = clap::Command::new(name.to_string())
			.no_binary_name(true)
			.disable_help_flag(true)
			.disable_version_flag(true);

		for arg in args {
			cmd = cmd.arg(clap::Arg::from(ClapArg(arg)));
		}
		cmd
	}

	fn format_error(e: clap::Error, args: &[Arg]) -> String {
		use clap::error::{ContextKind, ErrorKind};
		match e.kind() {
			ErrorKind::ValueValidation | ErrorKind::InvalidValue  => {
				let arg_name = e.get(ContextKind::InvalidArg)
					.map(|v| v.to_string())
					.unwrap_or_default()
					.trim_matches(|c| c == '<' || c == '>' || c == '-')
					.to_string();
				let expected = args.iter()
					.find(|a| a.name == arg_name)
					.map(|a| match a.arg_type {
						ArgType::String => "字符串",
						ArgType::Int => "整数",
						ArgType::Float => "浮点数",
						ArgType::Bool => "布尔值",
					})
					.unwrap_or("有效值");
				format!("参数 {} 输入无效，请提供一个{}", arg_name, expected)
			}
			ErrorKind::MissingRequiredArgument => {
				let arg_name = e.get(ContextKind::InvalidArg)
					.map(|v| v.to_string())
					.unwrap_or_default()
					.trim_matches(|c| c == '<' || c == '>' || c == '-')
					.to_string();
				format!("参数 {} 未提供", arg_name)
			}
			ErrorKind::UnknownArgument => {
				let arg_name = e.get(ContextKind::InvalidArg)
					.map(|v| v.to_string())
					.unwrap_or_default()
					.trim_matches(|c| c == '<' || c == '>' || c == '-')
					.to_string();
				format!("未知参数: {}", arg_name)
			}
			_ => e.to_string(),
		}
	}

	fn parse(
		input: &str,
		args: &[Arg],
	) -> Result<HashMap<String, ArgValue>, String> {
		let tokens: Vec<&str> = input.split_whitespace().collect();
		if tokens.is_empty() {
			return Ok(HashMap::new());
		}

		let command_name = tokens[0];
		let cmd = Self::to_clap_command(command_name, args);

		let matches = cmd
			.try_get_matches_from(&tokens[1..])
			.map_err(|e| Self::format_error(e, args))?;

		let mut result = HashMap::new();
		for arg_def in args {
			if let Some(value) = Self::get_value(&matches, arg_def) {
				result.insert(arg_def.name.to_string(), value);
			}
		}

		Ok(result)
	}

	fn get_value(matches: &clap::ArgMatches, arg_def: &Arg) -> Option<ArgValue> {
		match arg_def.arg_type {
			ArgType::String => matches.get_one::<String>(arg_def.name).map(|s| ArgValue::String(s.clone())),
			ArgType::Int => matches.get_one::<i64>(arg_def.name).map(|i| ArgValue::Int(*i)),
			ArgType::Float => matches.get_one::<f64>(arg_def.name).map(|f| ArgValue::Float(*f)),
			ArgType::Bool => matches.get_one::<bool>(arg_def.name).map(|b| ArgValue::Bool(*b)),
		}
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

		let command_name = text_content.split_whitespace().next().unwrap_or_default();

		let args = CommandRegistry::get_with_name(command_name)
			.map(|c| c.builder.args())
			.unwrap_or_default();

		let plugin_args = match Self::parse(&text_content, &args) {
			Ok(args) => args,
			Err(e) => {
				let _ = bot_ctx.reply(e.into()).await;
				return;
			}
		};

		let event = create_message_event_context!(message_event, plugin_args);
		Self::execute_plugins(&bot_ctx, &event, command_name).await;
	}

	fn extract_text(elements: Vec<puniyu_types::element::receive::Elements>) -> String {
		elements.iter().filter_map(|e| e.as_text()).collect::<Vec<_>>().join(" ")
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
