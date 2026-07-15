use puniyu_bot::Bot;
use std::ops::Deref;

/// 机器人会话
#[derive(Clone, Copy)]
pub struct BotSession<'c> {
	inner: &'c Bot,
}

impl<'c> BotSession<'c> {
	pub fn new(bot: &'c Bot) -> Self {
		Self { inner: bot }
	}
}

impl Deref for BotSession<'_> {
	type Target = Bot;
	fn deref(&self) -> &Self::Target {
		self.inner
	}
}
