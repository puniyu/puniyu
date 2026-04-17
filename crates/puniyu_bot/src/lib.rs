//! # puniyu_bot
//!
//! 统一的机器人门面与全局注册表。
//!
//! ## 特性
//!
//! - 提供 `Bot` trait
//! - 提供 `BotRegistry` 与 `BotId`
//! - 提供便捷函数 `get_bot`、`get_bot_count` 与 `get_all_bot`
//! - 提供宏 `register_bot!` 与 `unregister_bot!`

mod registry;
use log::debug;
#[doc(inline)]
pub use registry::BotRegistry;
mod macros;
mod types;
#[doc(inline)]
pub use types::*;

use puniyu_adapter_types::AdapterInfo;
use puniyu_contact::{Contact, ContactType};
use puniyu_logger::owo_colors::OwoColorize;
use puniyu_message::Message;
use puniyu_runtime::BotRuntime;
use std::fmt::Debug;

pub trait Bot: Debug + Send + Sync {
	/// 获取Bot Runtime
	fn runtime(&self) -> &dyn BotRuntime;
}

impl dyn Bot + '_ {
	/// 返回适配器信息。
	pub fn adapter_info(&self) -> &AdapterInfo {
		self.runtime().adapter().adapter_info()
	}

	/// 返回账户信息。
	pub fn account_info(&self) -> &puniyu_account::AccountInfo {
		self.runtime().account_info()
	}

	/// 发送消息。
	pub async fn send_message(
		&self,
		contact: &ContactType<'_>,
		message: &Message,
	) -> puniyu_error::Result<puniyu_adapter_types::SendMsgType> {
		let (msg_type, user_id) = match contact {
			ContactType::Friend(friend) => ("PrivateMessage", &friend.peer()),
			ContactType::Group(group) => ("GroupMssage", &group.peer()),
			ContactType::GroupTemp(group) => ("Group TempMessage", &group.peer()),
			ContactType::Guild(guild) => ("GuildMessage", &guild.peer()),
		};
		debug!("[{}:{}]\n{:#?}", format!("Send {}", msg_type).yellow(), user_id.green(), message);
		self.runtime().adapter().send_message(contact, message).await
	}
}

impl PartialEq for dyn Bot {
	fn eq(&self, other: &Self) -> bool {
		self.adapter_info() == other.adapter_info() && self.account_info() == other.account_info()
	}
}
