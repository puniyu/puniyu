//! # puniyu_bot
//!
//! 统一的机器人实例类型，封装适配器信息、适配器 API 与账户信息，并提供全局注册表。
//!
//! ## 特性
//!
//! - 提供 `Bot`
//! - 提供 `BotRegistry` 与 `BotId`
//! - 提供便捷函数 `get_bot`、`get_bot_count` 与 `get_all_bot`
//! - 提供宏 `register_bot!` 与 `unregister_bot!`
//!
//! ## 示例
//!
//! ```rust
//! use puniyu_account::AccountInfo;
//! use puniyu_adapter_api::AdapterApi;
//! use puniyu_adapter_types::{AdapterInfo, AdapterPlatform, AdapterProtocol};
//! use puniyu_bot::{Bot, BotRegistry};
//!
//! let mut adapter = AdapterInfo::default();
//! adapter.name = "console".to_string();
//! adapter.platform = AdapterPlatform::QQ;
//! adapter.protocol = AdapterProtocol::Console;
//!
//! let account = AccountInfo {
//!     uin: "123456789".to_string(),
//!     name: "Puniyu".to_string(),
//!     avatar: Default::default(),
//! };
//!
//! let bot = Bot::new(adapter, AdapterApi::default(), account);
//! let index = BotRegistry::register(bot.clone()).unwrap();
//!
//! assert_eq!(BotRegistry::get(index), Some(bot));
//! BotRegistry::unregister(index).unwrap();
//! ```

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
use std::fmt::{Debug, Formatter};

/// 机器人实例。
///
/// 统一封装适配器信息、适配器 API 与账户信息。
///
/// # 示例
///
/// ```rust
/// use puniyu_account::AccountInfo;
/// use puniyu_adapter_api::AdapterApi;
/// use puniyu_adapter_types::{AdapterInfo, AdapterPlatform, AdapterProtocol};
/// use puniyu_bot::Bot;
///
/// let mut adapter = AdapterInfo::default();
/// adapter.name = "console".to_string();
/// adapter.platform = AdapterPlatform::QQ;
/// adapter.protocol = AdapterProtocol::Console;
///
/// let account = AccountInfo {
///     uin: "123456789".to_string(),
///     name: "Puniyu".to_string(),
///     avatar: Default::default(),
/// };
///
/// let bot = Bot::new(adapter, AdapterApi::default(), account);
///
/// assert_eq!(bot.account().uin, "123456789");
/// assert_eq!(bot.adapter().name, "console");
/// ```
#[derive(Clone, PartialEq)]
pub struct Bot {
	adapter: AdapterInfo,
	api: AdapterApi,
	account: AccountInfo,
}

impl Debug for Bot {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_tuple("Bot").field(&self.adapter).field(&self.account).finish()
	}
}

impl Bot {
	/// 使用适配器信息、适配器 API 和账户信息创建机器人实例。
	pub fn new(adapter: AdapterInfo, api: AdapterApi, account: AccountInfo) -> Self {
		Self { adapter, api, account }
	}

	/// 返回当前机器人的适配器信息。
	pub fn adapter(&self) -> &AdapterInfo {
		&self.adapter
	}

	/// 返回当前机器人的适配器 API。
	pub fn api(&self) -> &AdapterApi {
		&self.api
	}

	/// 返回当前机器人的账户信息。
	pub fn account(&self) -> &AccountInfo {
		&self.account
	}
}

/// 按索引或 UIN 查询已注册的机器人。
pub fn get_bot<'a>(bot_id: impl Into<BotId<'a>>) -> Option<Bot> {
	BotRegistry::get(bot_id)
}

/// 返回当前已注册的机器人数量。
pub fn get_bot_count() -> usize {
	BotRegistry::all().len()
}

/// 返回所有已注册的机器人副本。
pub fn get_all_bot() -> Vec<Bot> {
	BotRegistry::all()
}
