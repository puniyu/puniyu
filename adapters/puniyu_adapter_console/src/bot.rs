use std::sync::Arc;

use crate::runtime::ConsoleBotRuntime;

#[derive(Debug)]
pub(crate) struct ConsoleBot {
	runtime: Arc<ConsoleBotRuntime>,
}

impl ConsoleBot {
	pub(crate) fn new(runtime: Arc<ConsoleBotRuntime>) -> Self {
		Self { runtime }
	}
}

impl puniyu_adapter::bot::Bot for ConsoleBot {
	fn runtime(&self) -> &dyn puniyu_adapter::runtime::BotRuntime {
		self.runtime.as_ref()
	}
}
