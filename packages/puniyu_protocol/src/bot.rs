use puniyu_types::bot::Bot as puniyu_bot;
use puniyu_types::adapter::AdapterApi;
include!(concat!(env!("OUT_DIR"), "/puniyu.bot.rs"));

impl From<BotInfo> for puniyu_bot {
	fn from(bot: BotInfo) -> Self {
		Self {
			adapter: bot.adapter.unwrap().into(),
			api: AdapterApi::default(),
			account: bot.account.unwrap().into()
		}
	}
}

impl From<puniyu_bot> for BotInfo {
	fn from(bot: puniyu_bot) -> Self {
		Self { adapter: Some(bot.adapter.into()), account: Some(bot.account.into()) }
	}
}
