use crate::tools::{cooldown, get_permission};
use crate::{message, tools};
use puniyu_core::async_trait::async_trait;
use itertools::Itertools as _;
use log::info;
use puniyu_core::command::{CommandAction, CommandRegistry, Permission, parser::CommandParser};
use crate::tools::has_permission;
use puniyu_core::config::{app_config, bot_config};
use puniyu_core::context::MessageContext;
use puniyu_core::event::{Event, EventBase, message::MessageBase};
use puniyu_core::handler::Handler;
use puniyu_logger::owo_colors::OwoColorize;

use std::collections::HashMap;
use puniyu_core::message::Message;

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
		let config = app_config();
		let global = config.prefix();
		let mut prefixes: Vec<String> = global.iter().map(|s| s.to_string()).collect();

		prefixes.extend(
			CommandRegistry::all()
				.into_iter()
				.filter_map(|c| c.handle.get().prefix().map(|p| p.to_string()))
				.dedup()
				.map(|pp| {
					global.map(|g| format!("{}{}", g, pp)).unwrap_or(pp)
				}),
		);

		prefixes
	}

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
		let aliases = bot_config().bot(bot_id).alias().into_iter().map(str::to_string).collect::<Vec<String>>();
		let mode = bot_config().bot(bot_id).mode();

		let has_alias =
			aliases.iter().any(|alias| !alias.is_empty() && original_text.starts_with(alias));

		if !tools::check_mode(event, &mode, has_alias) {
			return false;
		}

		let parser = CommandParser::builder()
			.aliases(aliases)
			.prefix(Self::build_prefix())
			.build()
			.parse(&original_text)
			.ok();

		if parser.is_some() {
			let _ = cooldown::set_cooldown(event);
			true
		} else {
			false
		}
	}

	async fn handle_command(&self, event: &MessageContext<'_>) {
		let original_text = Self::get_text(event);
		let bot_id = event.self_id();
		let aliases = 	bot_config().bot(bot_id).alias().into_iter().map(str::to_string).collect::<Vec<String>>();

		let parser = match CommandParser::builder()
			.aliases(aliases)
			.prefix(Self::build_prefix())
			.build()
			.parse(&original_text)
		{
			Ok(p) => p,
			Err(e) => {
				let error_msg = e.to_string();
				let msg = Message::from(error_msg.as_str());
				let _ = event.reply(msg).await;
				return;
			}
		};

		let command_name = parser.command_name().to_string();
		let parsed_args = parser.into_inner();
		let message_ctx = MessageContext::new(event.event(), parsed_args);

		Self::execute_command(&message_ctx, &command_name).await;
	}

	/// 执行命令
	async fn execute_command(ctx: &MessageContext<'_>, command_name: &str) {
		let commands = CommandRegistry::all()
			.into_iter()
			.map(|c| c.handle.get())
			.filter(|cmd| cmd.name() == command_name)
			.sorted_by_key(|cmd| cmd.priority());

		let perm = get_permission(ctx);

		for command in commands {
			if !has_permission!(&perm, command.permission()) {
				let msg = match command.permission() {
					Permission::Master => "暂无权限, 只有主人才能操作",
					Permission::Owner => "暂无权限, 只有群主或频道主才能操作",
					Permission::Admin => "暂无权限, 只有管理员才能操作!",
					Permission::All => unreachable!(),
				};
				let _ = ctx.reply(Message::from(msg)).await;
				continue;
			}

			let start_time = std::time::Instant::now();
			info!("[{}] 开始执行", format!("command:{}", command_name).yellow());

			let result = command.execute(ctx).await;

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

	fn priority(&self) -> u32 {
		2
	}

	#[inline]
	async fn handle(&self, event: &Event) -> puniyu_core::result::Result {
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
