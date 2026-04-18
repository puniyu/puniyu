use std::sync::Arc;

include!(concat!(env!("OUT_DIR"), "/puniyu.bot.rs"));

impl From<&puniyu_bot::Bot> for BotInfo {
	fn from(bot: &puniyu_bot::Bot) -> Self {
		Self {
			adapter: Some(bot.adapter_info().clone().into()),
			account: Some(bot.account_info().clone().into()),
		}
	}
}

impl From<&Arc<puniyu_bot::Bot>> for BotInfo {
	fn from(bot: &Arc<puniyu_bot::Bot>) -> Self {
		Self::from(bot.as_ref())
	}
}

impl From<Arc<puniyu_bot::Bot>> for BotInfo {
	fn from(bot: Arc<puniyu_bot::Bot>) -> Self {
		Self::from(bot.as_ref())
	}
}

