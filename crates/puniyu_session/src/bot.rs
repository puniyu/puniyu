use puniyu_bot::Bot;
use std::ops::Deref;

/// 机器人会话
#[derive(Clone)]
pub struct BotSession {
	inner: Bot,
}

impl BotSession {
	pub fn new(bot: &Bot) -> Self {
		Self { inner: bot.clone() }
	}

	pub fn bot(&self) -> &Bot {
		&self.inner
	}
}

impl Deref for BotSession {
	type Target = Bot;
	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}
