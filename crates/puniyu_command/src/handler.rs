use crate::matcher::MatchResult;
use async_trait::async_trait;
use puniyu_logger::{info, owo_colors::OwoColorize};
use puniyu_registry::command::CommandRegistry;
use puniyu_types::bot::Bot;
use puniyu_types::command::{HandlerAction, Permission};
use puniyu_types::context::{BotContext, MessageContext};
use puniyu_types::event::{Event, EventBase, message::MessageEvent};
use puniyu_types::handler::Handler;

mod parse;
use parse::ArgParser;

pub struct CommandHandler;

impl CommandHandler {
	pub async fn handle_matched(
		bot: Bot,
		message_event: &MessageEvent,
		match_result: &MatchResult,
	) {
		let command_name = &match_result.command_name;
		let args = &match_result.args;

		let bot_ctx = match message_event {
			MessageEvent::Friend(msg) => BotContext::new(bot.clone(), msg.contact().into()),
			MessageEvent::Group(msg) => BotContext::new(bot.clone(), msg.contact().into()),
		};

		let Some(command) = CommandRegistry::get_with_name(command_name) else {
			return;
		};

		let permission = command.builder.permission();
		if permission == Permission::Master && !message_event.is_master() {
			let _ = bot_ctx.reply("呜喵～这是只有主人才能用的命令哦!".into()).await;
			return;
		}

		let arg_defs = command.builder.args();

		let parsed_args = match ArgParser::parse(command_name, args, &arg_defs) {
			Ok(args) => args,
			Err(e) => {
				let _ = bot_ctx.reply(e.into()).await;
				return;
			}
		};

		let message_ctx = match message_event {
			MessageEvent::Friend(msg) => {
				MessageContext::new(MessageEvent::Friend(msg.clone()), parsed_args)
			}
			MessageEvent::Group(msg) => {
				MessageContext::new(MessageEvent::Group(msg.clone()), parsed_args)
			}
		};

		Self::execute_command(&bot_ctx, &message_ctx, command_name).await;
	}

	async fn execute_command(bot: &BotContext, event: &MessageContext, command_name: &str) {
		let plugins = CommandRegistry::get_plugins(command_name);
		for name in plugins {
			let Some(command) = CommandRegistry::get_with_plugin(&name, command_name) else {
				continue;
			};

			let start_time = std::time::Instant::now();
			info!("[{}] 开始执行", format!("command:{}:{}", &name, command_name).yellow());

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
