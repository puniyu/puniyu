use puniyu_bot::Bot;
use std::ops::Deref;

/// 机器人上下文
#[derive(Clone, Copy)]
pub struct BotContext<'c> {
	inner: &'c Bot,
}

impl<'c> BotContext<'c> {
	pub fn new(bot: &'c Bot) -> Self {
		Self { inner: bot }
	}
}

impl Deref for BotContext<'_> {
	type Target = Bot;
	fn deref(&self) -> &Self::Target {
		self.inner
	}
}
