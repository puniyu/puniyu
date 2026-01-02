mod arg;
mod matcher;

use arg::ArgParser;
use async_trait::async_trait;
use matcher::CommandMatcher;
use puniyu_logger::info;
use puniyu_logger::owo_colors::OwoColorize;
use puniyu_registry::command::CommandRegistry;
use puniyu_types::command::HandlerAction;
use puniyu_types::context::{BotContext, MessageContext};
use puniyu_types::event::message::MessageEvent;
use puniyu_types::event::{Event, EventBase, Permission};
use puniyu_types::handler::{Handler, HandlerResult, Matcher};
use std::sync::Arc;

#[derive(Default)]
pub struct CommandHandler;

impl CommandHandler {
	fn command_name(&self, event: &MessageEvent) -> String {
		let matcher = CommandMatcher::new(event);
		matcher.command_name()
	}

	async fn handle_command(&self, event: &MessageEvent) {
		let command = CommandMatcher::new(event);
		let command_name = self.command_name(event);
		let args = command.args();
		let Some(command) = CommandRegistry::get_with_name(&command_name) else {
			return;
		};

		let bot_ctx = match event {
			MessageEvent::Friend(msg) => {
				let bot = event.bot();
				BotContext::new(Arc::new(bot.clone()), msg.contact().into())
			}
			MessageEvent::Group(msg) => {
				let bot = event.bot();
				BotContext::new(Arc::new(bot.clone()), msg.contact().into())
			}
		};
		let permission = command.builder.permission();
		if permission == Permission::Master && !event.is_master() {
			let _ = bot_ctx.reply("呜喵～这是只有主人才能用的命令哦!".into()).await;
			return;
		}
		let builder_args = command.builder.args();

		let parsed_args = match ArgParser::parse(&command_name, &args, &builder_args) {
			Ok(args) => args,
			Err(e) => {
				let _ = bot_ctx.reply(e.into()).await;
				return;
			}
		};
		let message_ctx = match event {
			MessageEvent::Friend(msg) => {
				MessageContext::new(MessageEvent::Friend(msg.clone()), parsed_args)
			}
			MessageEvent::Group(msg) => {
				MessageContext::new(MessageEvent::Group(msg.clone()), parsed_args)
			}
		};
		Self::execute_command(&bot_ctx, &message_ctx, &command_name).await;
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

impl Matcher for CommandHandler {
	fn matches(&self, event: &Event) -> bool {
		if let Event::Message(message) = event {
			let command = CommandMatcher::new(message);
			command.matcher()
		} else {
			false
		}
	}
}

#[async_trait]
impl Handler for CommandHandler {
	fn name(&self) -> &str {
		"command"
	}
	async fn handle(&self, event: &Event) -> HandlerResult {
		if let Event::Message(message_event) = &event {
			self.handle_command(message_event.as_ref()).await;
		}
		Ok(())
	}
}
