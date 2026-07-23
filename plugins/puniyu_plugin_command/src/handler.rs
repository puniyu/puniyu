use puniyu_command::CommandRegistry;
use crate::executor::{CommandOutcome, execute};
use crate::invocation::{ParseOutcome, parse};
use async_trait::async_trait;
use log::error;
use puniyu_handler::{Handler, HandlerContext};
use puniyu_logger::owo_colors::OwoColorize;
use puniyu_session::MessageSession;
use std::collections::HashMap;

pub(crate) struct CommandHandler {
	commands: CommandRegistry,
}

impl CommandHandler {
	pub(crate) const fn new(commands: CommandRegistry) -> Self {
		Self { commands }
	}
}

#[async_trait]
impl Handler for CommandHandler {
	fn name(&self) -> &'static str {
		"command"
	}

	async fn handle(&self, mut ctx: HandlerContext<'_>) {
		let Some(message) = ctx.as_message() else {
			ctx.next().await;
			return;
		};

		let commands = self.commands.values();
		match parse(message, commands) {
			ParseOutcome::NotMatched => ctx.next().await,
			ParseOutcome::Invalid(e) => {
				let session = MessageSession::new(message, HashMap::new());
				if let Err(reply_error) = session.reply(e.to_string(), None).await {
					error!("[{}] 回复失败: {reply_error}", "command".yellow());
				}
			}
			ParseOutcome::Matched(invocation) => match execute(invocation).await {
				CommandOutcome::Handled => {}
				CommandOutcome::Continue => ctx.next().await,
			},
		}
	}
}
