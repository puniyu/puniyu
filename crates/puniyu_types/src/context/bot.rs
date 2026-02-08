use crate::adapter::AdapterApi;
use crate::bot::Bot;
use crate::account::AccountInfo;

#[derive(Clone)]
pub struct BotContext<'c> {
	bot: &'c Bot,
}

impl<'c> BotContext<'c> {
	pub fn new(bot: &'c Bot) -> Self {
		Self { bot }
	}
	pub fn api(&self) -> AdapterApi {
		self.bot.api()
	}

	pub fn account(&self) -> AccountInfo{
		self.bot.account()
	}
}
