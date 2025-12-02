use async_trait::async_trait;
use clap::builder::ValueParser;
use puniyu_logger::{info, owo_colors::OwoColorize};
use puniyu_registry::command::CommandRegistry;
use puniyu_types::bot::Bot;
use puniyu_types::command::{Arg, ArgMode, ArgType, ArgValue, HandlerAction};
use puniyu_types::context::{BotContext, MessageContext};
use puniyu_types::event::{Event, EventBase, message::MessageEvent};
use puniyu_types::handler::Handler;
use puniyu_matcher_command::MatchResult;
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
			ArgType::Bool => clap_arg
				.value_parser(clap::value_parser!(bool))
				.num_args(0..=1)
				.default_missing_value("true"),
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

struct ArgParser;

impl ArgParser {
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

		let get_arg_name = |e: &clap::Error| {
			e.get(ContextKind::InvalidArg)
				.map(|v| v.to_string())
				.unwrap_or_default()
				.trim_matches(|c| c == '<' || c == '>' || c == '-')
				.to_string()
		};

		let get_type_name = |arg_name: &str| {
			args.iter()
				.find(|a| a.name == arg_name)
				.map(|a| match a.arg_type {
					ArgType::String => "字符串",
					ArgType::Int => "整数",
					ArgType::Float => "浮点数",
					ArgType::Bool => "布尔值",
				})
				.unwrap_or("有效值")
		};

		match e.kind() {
			ErrorKind::InvalidValue | ErrorKind::ValueValidation => {
				let arg_name = get_arg_name(&e);
				format!(
					"参数 {} 输入无效，请提供一个{}",
					arg_name,
					get_type_name(&arg_name)
				)
			}
			ErrorKind::UnknownArgument => {
				format!("未知参数: {}", get_arg_name(&e))
			}
			ErrorKind::TooManyValues => {
				format!("参数 {} 提供了过多的值", get_arg_name(&e))
			}
			ErrorKind::TooFewValues => {
				format!("参数 {} 提供的值不足", get_arg_name(&e))
			}
			ErrorKind::MissingRequiredArgument => {
				format!("缺少必需参数: {}", get_arg_name(&e))
			}
			_ => e.to_string(),
		}
	}

	fn parse(
		command_name: &str,
		args_text: &str,
		arg_defs: &[Arg],
	) -> Result<HashMap<String, ArgValue>, String> {
		let tokens: Vec<&str> = args_text.split_whitespace().collect();
		let cmd = Self::to_clap_command(command_name, arg_defs);

		let matches = cmd
			.try_get_matches_from(&tokens)
			.map_err(|e| Self::format_error(e, arg_defs))?;

		let mut result = HashMap::new();
		for arg_def in arg_defs {
			if let Some(value) = Self::get_value(&matches, arg_def) {
				result.insert(arg_def.name.to_string(), value);
			}
		}

		Ok(result)
	}

	fn get_value(matches: &clap::ArgMatches, arg_def: &Arg) -> Option<ArgValue> {
		match arg_def.arg_type {
			ArgType::String => matches
				.get_one::<String>(arg_def.name)
				.map(|s| ArgValue::String(s.clone())),
			ArgType::Int => matches
				.get_one::<i64>(arg_def.name)
				.map(|i| ArgValue::Int(*i)),
			ArgType::Float => matches
				.get_one::<f64>(arg_def.name)
				.map(|f| ArgValue::Float(*f)),
			ArgType::Bool => matches
				.get_one::<bool>(arg_def.name)
				.map(|b| ArgValue::Bool(*b)),
		}
	}
}

pub struct CommandHandler;

impl CommandHandler {
	pub async fn handle_matched(
		bot: Bot,
		message_event: &MessageEvent,
		match_result: &MatchResult,
	) {
		let command_name = &match_result.command_name;
		let args_text = &match_result.args_text;

		let bot_ctx = match message_event {
			MessageEvent::Friend(msg) => create_context_bot!(bot.clone(), msg.contact().into()),
			MessageEvent::Group(msg) => create_context_bot!(bot.clone(), msg.contact().into()),
		};

		let Some(command) = CommandRegistry::get_with_name(command_name) else {
			return;
		};
		let arg_defs = command.builder.args();

		let parsed_args = match ArgParser::parse(command_name, args_text, &arg_defs) {
			Ok(args) => args,
			Err(e) => {
				let _ = bot_ctx.reply(e.into()).await;
				return;
			}
		};

		let message_ctx = match message_event {
			MessageEvent::Friend(msg) => {
				create_message_event_context!(MessageEvent::Friend(msg.clone()), parsed_args)
			}
			MessageEvent::Group(msg) => {
				create_message_event_context!(MessageEvent::Group(msg.clone()), parsed_args)
			}
		};

		Self::execute_plugins(&bot_ctx, &message_ctx, command_name).await;
	}

	async fn execute_plugins(bot: &BotContext, event: &MessageContext, command_name: &str) {
		let plugins = CommandRegistry::get_plugins(command_name);
		for name in plugins {
			let Some(command) = CommandRegistry::get_with_plugin(&name, command_name) else {
				continue;
			};

			let start_time = std::time::Instant::now();
			info!(
				"[{}] 开始执行",
				format!("command:{}:{}", &name, command_name).yellow()
			);

			let result = command.builder.run(bot, event).await;

			info!(
				"[{}] 执行完毕, 耗时{}ms",
				format!("command:{}:{}", &name, command_name).yellow(),
				start_time.elapsed().as_millis()
			);

			match result {
				Ok(HandlerAction::Done) => break,
				Ok(HandlerAction::Continue) | Err(_) => continue,
			}
		}
	}
}

#[async_trait]
impl Handler for CommandHandler {
	type MatchResult = MatchResult;

	fn name(&self) -> &str {
		"command"
	}

	async fn handle(&self, bot: Bot, event: Event, match_result: Option<Self::MatchResult>) {
		let Some(match_result) = match_result else {
			return;
		};

		if let Event::Message(message_event) = &event {
			Self::handle_matched(bot, message_event.as_ref(), &match_result).await;
		}
	}
}
