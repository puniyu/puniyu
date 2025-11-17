mod registry;

use puniyu_adapter_api::{AdapterApi, Result};
use puniyu_adapter_api::types::SendMsgType;
use puniyu_builder::account::AccountInfo;
use puniyu_builder::adapter::AdapterInfo;
use puniyu_contact::ContactType;
use puniyu_element::Message;
pub use registry::BotRegistry;

#[derive(Clone)]
pub enum BotId {
	Index(u64),
	SelfId(String),
}

impl From<u64> for BotId {
	fn from(index: u64) -> Self {
		Self::Index(index)
	}
}

impl From<&str> for BotId {
	fn from(self_id: &str) -> Self {
		Self::SelfId(self_id.to_string())
	}
}

impl From<String> for BotId {
	fn from(self_id: String) -> Self {
		Self::SelfId(self_id)
	}
}

#[derive(Clone)]
pub struct Bot {
	/// 适配器信息
	pub adapter: AdapterInfo,
	/// 适配器API
	pub api: &'static dyn AdapterApi,
	/// 账户信息
	pub account: AccountInfo,
}

#[derive(Debug, Clone)]
pub struct BotInfo {
	pub adapter: AdapterInfo,
	/// 账户信息
	pub account: AccountInfo,
}

impl Bot {
	pub async fn send_msg(&self, contact: ContactType, message: Message) -> Result<SendMsgType> {
		self.api.send_msg(contact, message).await
	}
}
