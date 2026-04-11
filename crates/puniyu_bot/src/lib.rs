//! # puniyu_bot
//!
//! 统一的机器人实例类型，封装适配器与账户信息，并提供全局注册表。
//!
//! ## 特性
//!
//! - 提供 `Bot`
//! - 提供 `BotRegistry` 与 `BotId`
//! - 提供便捷函数 `get_bot`、`get_bot_count` 与 `get_all_bot`
//! - 提供宏 `register_bot!` 与 `unregister_bot!`

mod registry;
#[doc(inline)]
pub use registry::BotRegistry;
mod macros;
mod types;
#[doc(inline)]
pub use types::*;
#[doc(hidden)]
pub use puniyu_runtime::FrameworkRuntime;
use puniyu_account::AccountInfo;
use puniyu_runtime::Runtime;
use puniyu_adapter_types::{AdapterInfo, SendMsgType};
use puniyu_contact::ContactType;
use puniyu_message::Message;
use std::{
	fmt::{Debug, Formatter},
	sync::Arc,
};

/// 机器人实例。
#[derive(Clone)]
pub struct Bot {
	adapter: Arc<AdapterInfo>,
	runtime: Arc<dyn FrameworkRuntime>,
	account: Arc<AccountInfo>,
}

impl Debug for Bot {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Bot")
			.field("adapter", &self.adapter)
			.field("account", &self.account)
			.finish()
	}
}

impl PartialEq for Bot {
	fn eq(&self, other: &Self) -> bool {
		self.adapter == other.adapter
			&& Arc::ptr_eq(&self.runtime, &other.runtime)
			&& self.account == other.account
	}
}

impl Bot {
	/// 使用适配器信息、适配器运行时与账户信息创建机器人实例。
	pub fn new(
		adapter: impl Into<Arc<AdapterInfo>>,
		runtime: impl Into<Arc<dyn FrameworkRuntime>>,
		account: impl Into<Arc<AccountInfo>>,
	) -> Self {
		Self { adapter: adapter.into(), runtime: runtime.into(), account: account.into() }
	}

	/// 返回适配器信息引用。
	pub fn adapter(&self) -> &AdapterInfo {
		&self.adapter
	}

	/// 返回适配器运行时引用。
	pub fn runtime(&self) -> &dyn Runtime {
		self.runtime.as_ref()
	}

	pub async fn send_message(
		&self,
		contact: &ContactType<'_>,
		message: &Message,
	) -> puniyu_error::Result<SendMsgType> {
		self.runtime.send_message(contact, message).await
	}

	/// 返回账户信息引用。
	pub fn account(&self) -> &AccountInfo {
		&self.account
	}
}
