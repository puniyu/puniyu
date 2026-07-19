use crate::CommandRegistry;
use crate::executor::{CommandOutcome, execute};
use crate::invocation::{ParseOutcome, parse};
use async_trait::async_trait;
use log::error;
use puniyu_logger::owo_colors::OwoColorize;
use puniyu_middleware::{Middleware, MiddlewareContext};
use puniyu_session::MessageSession;
use std::collections::HashMap;

pub(crate) struct CommandMiddleware {
	commands: CommandRegistry,
}

impl CommandMiddleware {
	pub(crate) const fn new(commands: CommandRegistry) -> Self {
		Self { commands }
	}
}

#[async_trait]
impl Middleware for CommandMiddleware {
	fn name(&self) -> &'static str {
		"command"
	}

	async fn handle(&self, mut chain: MiddlewareContext<'_>) {
		let Some(message) = chain.as_message() else {
			chain.next().await;
			return;
		};

		let commands = match self.commands.all() {
			Ok(commands) => commands,
			Err(error) => {
				error!("[{}] 读取命令失败: {error}", "command".yellow());
				return;
			}
		};
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
