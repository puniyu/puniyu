use puniyu_utils::adapter::{AccountInfo, AdapterInfo};

pub mod registry;

#[derive(Clone)]
pub enum BotId {
	Index(u64),
	SelfId(String),
}

impl From<u64> for BotId {
	fn from(index: u64) -> Self {
		BotId::Index(index)
	}
}

impl From<&str> for BotId {
	fn from(self_id: &str) -> Self {
		BotId::SelfId(self_id.to_string())
	}
}

impl From<String> for BotId {
	fn from(self_id: String) -> Self {
		BotId::SelfId(self_id)
	}
}

#[derive(Debug, Clone)]
pub struct Bot {
	pub index: u64,
	pub adapter: AdapterInfo,
	pub account: AccountInfo,
}
