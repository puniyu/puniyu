use puniyu_bot::Bot;
use std::{ops::Deref, sync::Arc};

/// bot会话
#[derive(Clone)]
pub struct BotSession {
	inner: Arc<dyn Bot>,
}

impl BotSession {
	pub fn new(bot: impl Into<Arc<dyn Bot>>) -> Self {
		Self { inner: bot.into() }
	}
}

impl Deref for BotSession {
	type Target = Arc<dyn Bot>;
	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}
