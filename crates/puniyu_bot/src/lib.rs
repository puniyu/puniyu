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

use puniyu_account::AccountInfo;
use puniyu_adapter_api::AdapterApi;
use puniyu_adapter_types::AdapterInfo;
use std::{
	fmt::{Debug, Formatter},
	sync::Arc,
};

/// 机器人实例。
#[derive(Clone, PartialEq)]
pub struct Bot {
	adapter: Arc<AdapterInfo>,
	api: Arc<AdapterApi>,
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

impl Bot {
	/// 使用适配器信息、适配器 API 与账户信息创建机器人实例。
	pub fn new(
		adapter: impl Into<Arc<AdapterInfo>>,
		api: impl Into<Arc<AdapterApi>>,
		account: impl Into<Arc<AccountInfo>>,
	) -> Self {
		Self { adapter: adapter.into(), api: api.into(), account: account.into() }
	}

	/// 返回适配器信息引用。
	pub fn adapter(&self) -> &AdapterInfo {
		&self.adapter
	}

	/// 返回适配器 API 引用。
	pub fn api(&self) -> &AdapterApi {
		&self.api
	}

	/// 返回账户信息引用。
	pub fn account(&self) -> &AccountInfo {
		&self.account
	}
}
