mod arg;

use crate::{config, cooldown, message, tools};
use arg::ArgParser;
use async_trait::async_trait;
use itertools::Itertools;
use puniyu_config::Config;
use puniyu_logger::info;
use puniyu_logger::owo_colors::OwoColorize;
use puniyu_registry::command::{Command, CommandRegistry};
use puniyu_types::adapter::MessageApi;
use puniyu_types::command::CommandAction;
use puniyu_types::context::{BotContext, MessageContext};
use puniyu_types::event::message::MessageEvent;
use puniyu_types::event::{Event, Permission};
use puniyu_types::handler::{Handler, HandlerResult};
use std::sync::Arc;

#[derive(Default)]
pub struct CommandHandler;

impl CommandHandler {
	fn get_text(event: &MessageEvent) -> String {
		event.elements().iter().filter_map(|e| e.as_text()).collect::<Vec<_>>().join(" ")
	}

	fn get_commands() -> Vec<Arc<Command>> {
		CommandRegistry::all().into_iter().sorted_by_key(|cmd| cmd.builder.rank()).collect()
	}

	fn matches_command(event: &MessageEvent) -> bool {
		if !tools::check_perm(event) {
			return false;
		}

		if cooldown::is_cooling_down(event) {
			return false;
		}

		message::log(event);

		let original_text = Self::get_text(event);
		if original_text.is_empty() {
			return false;
		}

		let bot_id = event.self_id();
		let aliases = config::get_bot_alias(bot_id);
		let mode = config::get_bot_reactive_mode(bot_id);

		let text = tools::strip_bot_alias(original_text.as_str(), aliases.as_ref());
		let has_alias = original_text != text;

		if !tools::check_mode(event, &mode, has_alias) {
			return false;
		}

		let config = Config::app();
		let global_prefix = config.prefix();
		let input_text = text.strip_prefix(global_prefix).unwrap_or(&text);
		let command_name = input_text.split_whitespace().next().unwrap_or("").trim();
		if command_name.is_empty() {
			return false;
		}

		let commands = Self::get_commands();
		if commands.iter().any(|cmd| {
			cmd.builder.name() == command_name || cmd.builder.alias().contains(&command_name)
		}) {
			cooldown::set_cooldown(event);
			true
		} else {
			false
		}
	}

	fn get_command_name(event: &MessageEvent) -> String {
		let config = Config::app();
		let global_prefix = config.prefix();
		let aliases = config::get_bot_alias(event.self_id());
		let original_text = Self::get_text(event);
		let text = tools::strip_bot_alias(original_text.as_str(), aliases.as_ref());
		let input_text = text.strip_prefix(global_prefix).unwrap_or(text.as_str());
		input_text.split_whitespace().next().unwrap_or_default().trim().to_string()
	}

	fn get_args(event: &MessageEvent) -> Vec<String> {
		let aliases = config::get_bot_alias(event.self_id());
		let original_text = Self::get_text(event);
		let text = tools::strip_bot_alias(original_text.as_str(), aliases.as_ref());
		let config = Config::app();
		let global_prefix = config.prefix();

		let input_text = if global_prefix.is_empty() {
			text.as_str()
		} else {
			text.strip_prefix(global_prefix).unwrap_or(&text)
		};

		let mut parts = input_text.splitn(2, char::is_whitespace);
		parts.next();
		let args_str = parts.next().unwrap_or("");
		args_str.split_whitespace().map(String::from).collect()
	}

	async fn handle_command(&self, event: &MessageEvent) {
		let input_name = Self::get_command_name(event);
		let args = Self::get_args(event);

		let commands = CommandRegistry::all();
		let Some(command) = commands.iter().find(|cmd| {
			cmd.builder.name() == input_name.as_str()
				|| cmd.builder.alias().contains(&input_name.as_str())
		}) else {
			return;
		};

		let bot_ctx = { BotContext::new(Arc::new(event.bot().clone())) };
		let permission = command.builder.permission();
		if permission == Permission::Master && !event.is_master() {
			let _ = bot_ctx
				.api()
				.send_msg(event.contact(), "呜喵～这是只有主人才能用的命令哦!".into())
				.await;
			return;
		}
		let builder_args = command.builder.args();
		let parsed_args = match ArgParser::parse(command.builder.name(), &args, &builder_args) {
			Ok(args) => args,
			Err(e) => {
				let _ = bot_ctx.api().send_msg(event.contact(), e.into()).await;
				return;
			}
		};
		let message_ctx = MessageContext::new(bot_ctx, event.clone(), parsed_args);

		Self::execute_command(&message_ctx, command.builder.name()).await;
	}

	async fn execute_command(ctx: &MessageContext, command_name: &str) {
		let commands =
			Self::get_commands().into_iter().filter(|cmd| cmd.builder.name() == command_name);

		for command in commands {
			let plugin_name = &command.plugin_name;
			let start_time = std::time::Instant::now();
			info!("[{}] 开始执行", format!("command:{}:{}", plugin_name, command_name).yellow());

			let result = command.builder.run(ctx).await;

			info!(
				"[{}] 执行完毕, 耗时{}ms",
				format!("command:{}:{}", plugin_name, command_name).yellow(),
				start_time.elapsed().as_millis()
			);

			match result {
				Ok(CommandAction::Done) => break,
				Ok(CommandAction::Continue) | Err(_) => continue,
			}
		}
	}
}

#[async_trait]
impl Handler for CommandHandler {
	fn name(&self) -> &str {
		"command"
	}

	async fn handle(&self, event: &Event) -> HandlerResult {
		let Event::Message(message_event) = event else {
			return Ok(());
		};

		if !Self::matches_command(message_event) {
			return Ok(());
		}

		self.handle_command(message_event.as_ref()).await;
		Ok(())
	}
}
