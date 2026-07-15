use crate::executor::{CommandOutcome, execute};
use crate::invocation::{ParseOutcome, parse};
use async_trait::async_trait;
use log::error;
use puniyu_command::CommandRegistry;
use puniyu_session::MessageSession;
use puniyu_handler::{HandleContext, Handler};
use puniyu_logger::owo_colors::OwoColorize;
use std::collections::HashMap;

/// 命令匹配、权限、冷却与执行分发处理器。
#[derive(Debug, Default, Clone, Copy)]
pub struct CommandHandler;

impl CommandHandler {
	/// 创建命令处理器。
	pub const fn new() -> Self {
		Self
	}
}

#[async_trait]
impl Handler for CommandHandler {
	fn name(&self) -> &'static str {
		"command"
	}

	async fn handle(&self, mut chain: HandleContext<'_, '_>) {
		let Some(message) = chain.as_message() else {
			chain.next().await;
			return;
		};

		let commands = CommandRegistry::all();
		match parse(message, commands) {
			ParseOutcome::NotMatched => chain.next().await,
			ParseOutcome::Invalid(error) => {
				let context = MessageSession::new(message, HashMap::new());
				if let Err(reply_error) = context.reply(error.to_string(), None).await {
					error!("[{}] 回复失败: {reply_error}", "command".yellow());
				}
			}
			ParseOutcome::Matched(invocation) => match execute(invocation).await {
				CommandOutcome::Handled => {}
				CommandOutcome::Continue => chain.next().await,
			},
		}
	}
}
