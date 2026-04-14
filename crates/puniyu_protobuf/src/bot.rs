use std::sync::Arc;

include!(concat!(env!("OUT_DIR"), "/puniyu.bot.rs"));

impl From<&dyn puniyu_bot::Bot> for BotInfo {
	fn from(bot: &dyn puniyu_bot::Bot) -> Self {
		Self {
			adapter: Some(bot.adapter_info().clone().into()),
			account: Some(bot.account().clone().into()),
		}
	}
}

impl From<&Arc<dyn puniyu_bot::Bot>> for BotInfo {
	fn from(bot: &Arc<dyn puniyu_bot::Bot>) -> Self {
		Self::from(bot.as_ref())
	}
}

impl From<Arc<dyn puniyu_bot::Bot>> for BotInfo {
	fn from(bot: Arc<dyn puniyu_bot::Bot>) -> Self {
		Self::from(bot.as_ref())
	}
}

