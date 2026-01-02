use crate::account::AccountInfo;
use crate::adapter::{AdapterApi, AdapterInfo, MessageApi, Result, SendMsgType};
use crate::contact::ContactType;
use crate::element;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Bot {
	/// 适配器信息
	pub adapter: AdapterInfo,
	/// 适配器API
	#[serde(skip)]
	pub api: AdapterApi,
	/// 账户信息
	pub account: AccountInfo,
}

impl Eq for Bot {}

impl std::fmt::Debug for Bot {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Bot")
			.field("adapter", &self.adapter)
			.field("account", &self.account)
			.finish_non_exhaustive()
	}
}

impl PartialEq for Bot {
	fn eq(&self, other: &Self) -> bool {
		self.adapter == other.adapter && self.account == other.account
	}
}

impl Bot {
	/// 发送消息
	///
	/// ## 参数
	/// `_contact` - 联系人
	/// `_message` - 消息元素
	///
	pub async fn send_msg(
		&self,
		contact: ContactType,
		message: element::Message,
	) -> Result<SendMsgType> {
		self.api.send_msg(contact, message).await
	}
}
