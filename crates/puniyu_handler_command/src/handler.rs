use crate::tools::cooldown;
use crate::{config, message, tools};
use async_trait::async_trait;
use itertools::Itertools as _;
use puniyu_command::{CommandAction, CommandRegistry, Permission};
use puniyu_command_parser::CommandParser;
use puniyu_config::app_config;
use puniyu_context::MessageContext;
use puniyu_event::{Event, EventBase, message::MessageBase};
use puniyu_handler::Handler;
use puniyu_logger::info;
use puniyu_logger::owo_colors::OwoColorize;
use puniyu_plugin_core::PluginRegistry;
use std::collections::HashMap;

/// 命令处理器。
#[derive(Default)]
pub struct CommandHandler;

impl CommandHandler {
	/// 获取消息文本内容
	fn get_text(event: &MessageContext) -> String {
		event.elements().iter().filter_map(|e| e.as_text()).collect::<Vec<_>>().join(" ")
	}

	/// 构建前缀列表
	fn build_prefix() -> Vec<String> {
		let global = app_config().prefix();
		let mut prefixes: Vec<String> = global.iter().cloned().collect();

		prefixes.extend(PluginRegistry::all().iter().filter_map(|p| p.prefix()).flat_map(|pp| {
			global.as_ref().map(|g| format!("{}{}", g, pp)).or(Some(pp.to_string()))
		}));

		prefixes
	}

	/// 检查消息是否匹配命令
	fn matches_command(event: &MessageContext) -> bool {
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

		let has_alias =
			aliases.iter().any(|alias| !alias.is_empty() && original_text.starts_with(alias));

		if !tools::check_mode(event, &mode, has_alias) {
			return false;
		}

		let parser = CommandParser::builder()
			.aliases(aliases)
			.prefix(Self::build_prefix())
			.parse(&original_text)
			.ok();

		if parser.is_some() {
			let _ = cooldown::set_cooldown(event);
			true
		} else {
			false
		}
	}

	/// 处理命令执行
	async fn handle_command(&self, event: &MessageContext<'_>) {
		let original_text = Self::get_text(event);
		let aliases = config::get_bot_alias(event.self_id());

		let parser = match CommandParser::builder()
			.aliases(aliases)
			.prefix(Self::build_prefix())
			.parse(&original_text)
		{
			Ok(p) => p,
			Err(e) => {
				let error_msg = e.to_string();
				let msg = puniyu_message::Message::from(error_msg.as_str());
				let _ = event.reply(msg).await;
				return;
			}
		};

		let command_name = parser.command_name().to_string();
		let parsed_args = parser.into_inner();

		// 检查权限
		let commands = CommandRegistry::get_with_command_name(&command_name);
		if let Some(cmd) = commands.first()
			&& cmd.builder.permission() == Permission::Master
			&& !event.is_master()
		{
			let msg = puniyu_message::Message::from("呜喵～这是只有主人才能用的命令哦!");
			let _ = event.reply(msg).await;
			return;
		}

		let message_ctx = MessageContext::new(event.event(), parsed_args);

		Self::execute_command(&message_ctx, &command_name).await;
	}

	/// 执行命令
	async fn execute_command(ctx: &MessageContext<'_>, command_name: &str) {
		let commands = CommandRegistry::all()
			.into_iter()
			.filter(|cmd| cmd.builder.name() == command_name)
			.sorted_by_key(|cmd| cmd.builder.priority());

		for command in commands {
			let start_time = std::time::Instant::now();
			info!("[{}] 开始执行", format!("command:{}", command_name).yellow());

			let result = command.builder.run(ctx).await;

			info!(
				"[{}] 执行完毕, 耗时{}ms",
				format!("command:{}", command_name).yellow(),
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
	fn name(&self) -> &'static str {
		"command"
	}

	#[inline]
	async fn handle(&self, event: &Event) -> puniyu_error::Result {
		let Some(message_event) = event.as_message() else {
			return Ok(());
		};

		let ctx = MessageContext::new(message_event, HashMap::new());

		if !Self::matches_command(&ctx) {
			return Ok(());
		}

		self.handle_command(&ctx).await;
		Ok(())
	}
}
