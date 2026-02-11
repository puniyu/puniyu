//! # puniyu_bot
//!
//! 机器人实例库，提供机器人的统一类型定义。
//!
//! ## 概述
//!
//! `puniyu_bot` 提供了机器人实例的类型定义，封装了适配器信息、API 接口和账户信息。
//!
//! ## 使用方式
//!
//! ### 创建机器人实例
//!
//! ```rust
//! use puniyu_bot::Bot;
//! use puniyu_account::AccountInfo;
//! use puniyu_adapter_core::api::AdapterApi;
//! use puniyu_adapter_core::adapter_info;
//! use puniyu_adapter_core::types::info::{AdapterPlatform, AdapterProtocol};
//!
//! let adapter = adapter_info!(
//!     "my_adapter",
//!     AdapterPlatform::QQ,
//!     AdapterProtocol::Console
//! );
//!
//! let api = AdapterApi::default();
//!
//! let account = AccountInfo {
//!     uin: "123456".to_string(),
//!     name: "MyBot".to_string(),
//!     avatar: "".to_string(),
//! };
//!
//! let bot = Bot::new(adapter, api, account);
//! ```
//!
//! ### 访问机器人信息
//!
//! ```rust,ignore
//! use puniyu_bot::Bot;
//!
//! // 获取适配器信息
//! let adapter = bot.adapter();
//! println!("适配器: {}", adapter.name);
//!
//! // 获取 API
//! let api = bot.api();
//! api.message().send_msg(&contact, &message).await?;
//!
//! // 获取账户信息
//! let account = bot.account();
//! println!("机器人 UIN: {}", account.uin);
//! ```

#[cfg(feature = "registry")]
mod registry;
#[cfg(feature = "registry")]
pub use registry::BotRegistry;
pub mod types;

use puniyu_account::AccountInfo;
use puniyu_adapter_core::api::AdapterApi;
use puniyu_adapter_core::types::info::AdapterInfo;
use std::fmt::{Debug, Formatter};

/// 机器人实例
///
/// 封装了适配器信息、API 接口和账户信息的机器人实例。
///
/// # 字段
///
/// - `adapter` - 适配器信息
/// - `api` - 适配器 API 接口
/// - `account` - 机器人账户信息
///
/// # 示例
///
/// ```rust
/// use puniyu_bot::Bot;
/// use puniyu_account::AccountInfo;
/// use puniyu_adapter_core::api::AdapterApi;
/// use puniyu_adapter_core::adapter_info;
/// use puniyu_adapter_core::types::info::{AdapterPlatform, AdapterProtocol};
///
/// let adapter = adapter_info!(
///     "test_adapter",
///     AdapterPlatform::QQ,
///     AdapterProtocol::Console
/// );
///
/// let api = AdapterApi::default();
///
/// let account = AccountInfo {
///     uin: "123456".to_string(),
///     name: "TestBot".to_string(),
///     avatar: "".to_string(),
/// };
///
/// let bot = Bot::new(adapter, api, account);
///
/// assert_eq!(bot.account().uin, "123456");
/// ```
#[derive(Clone)]
pub struct Bot {
	/// 适配器信息
	adapter: AdapterInfo,
	/// 适配器API
	api: AdapterApi,
	/// 账户信息
	account: AccountInfo,
}

impl Debug for Bot {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_tuple("Bot").field(&self.adapter).field(&self.account).finish()
	}
}

impl PartialEq for Bot {
	fn eq(&self, other: &Self) -> bool {
		self.adapter == other.adapter && self.account == other.account
	}
}

impl Bot {
	/// 创建新的机器人实例
	///
	/// # 参数
	///
	/// - `adapter` - 适配器信息
	/// - `api` - 适配器 API 接口
	/// - `account` - 机器人账户信息
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_bot::Bot;
	/// use puniyu_account::AccountInfo;
	/// use puniyu_adapter_core::api::AdapterApi;
	/// use puniyu_adapter_core::adapter_info;
	/// use puniyu_adapter_core::types::info::{AdapterPlatform, AdapterProtocol};
	///
	/// let adapter = adapter_info!(
	///     "my_adapter",
	///     AdapterPlatform::QQ,
	///     AdapterProtocol::Console
	/// );
	///
	/// let api = AdapterApi::default();
	///
	/// let account = AccountInfo {
	///     uin: "123456".to_string(),
	///     name: "MyBot".to_string(),
	///     avatar: "".to_string(),
	/// };
	///
	/// let bot = Bot::new(adapter, api, account);
	/// ```
	pub fn new(adapter: AdapterInfo, api: AdapterApi, account: AccountInfo) -> Self {
		Self { adapter, api, account }
	}

	/// 获取适配器信息
	///
	/// # 返回值
	///
	/// 返回适配器信息的引用
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let adapter = bot.adapter();
	/// println!("适配器名称: {}", adapter.name);
	/// println!("平台: {}", adapter.platform);
	/// ```
	pub fn adapter(&self) -> &AdapterInfo {
		&self.adapter
	}

	/// 获取适配器 API
	///
	/// # 返回值
	///
	/// 返回适配器 API 的引用
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let api = bot.api();
	///
	/// // 发送消息
	/// api.message().send_msg(&contact, &message).await?;
	///
	/// // 获取群列表
	/// let groups = api.group().get_group_list().await?;
	/// ```
	pub fn api(&self) -> &AdapterApi {
		&self.api
	}

	/// 获取账户信息
	///
	/// # 返回值
	///
	/// 返回机器人账户信息的引用
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let account = bot.account();
	/// println!("机器人 UIN: {}", account.uin);
	/// println!("机器人昵称: {}", account.name);
	/// ```
	pub fn account(&self) -> &AccountInfo {
		&self.account
	}
}
