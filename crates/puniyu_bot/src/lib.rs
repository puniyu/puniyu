//! # puniyu_bot
//!
//! 机器人实例库，提供机器人的统一类型定义和注册表管理。
//!
//! ## 概述
//!
//! `puniyu_bot` 提供了机器人实例的类型定义，封装了适配器信息、API 接口和账户信息。
//! 该库还提供了全局注册表功能，用于管理多个机器人实例。
//!
//! ## 特性
//!
//! - 🎯 **统一封装** - 将适配器、API 和账户信息封装在一个类型中
//! - 🔧 **简单易用** - 提供简洁的 API 访问机器人信息
//! - 📦 **类型安全** - 使用 Rust 类型系统确保数据正确性
//! - 🔄 **可克隆** - 支持克隆操作，方便在多处使用
//! - 📋 **注册表管理** - 全局注册表功能，管理多个机器人实例
//! - 🎨 **便捷宏** - 提供 `bot!` 宏快速创建机器人实例
//!
//! ## 使用方式
//!
//! ### 创建机器人实例
//!
//! ```rust
//! use puniyu_bot::Bot;
//! use puniyu_account::AccountInfo;
//! use puniyu_adapter_api::AdapterApi;
//! use puniyu_adapter_types::AdapterInfo;
//! use bytes::Bytes;
//!
//! let adapter = AdapterInfo {
//!     name: "my_adapter".to_string(),
//!     platform: "QQ".to_string(),
//!     protocol: "Console".to_string(),
//! };
//!
//! let api = AdapterApi::default();
//!
//! let account = AccountInfo {
//!     uin: "123456".to_string(),
//!     name: "MyBot".to_string(),
//!     avatar: Bytes::new(),
//! };
//!
//! let bot = Bot::new(adapter, api, account);
//! ```
//!
//! ### 使用宏创建机器人
//!
//! ```rust,ignore
//! use puniyu_bot::bot;
//!
//! let bot = bot!(
//!     adapter: adapter_info,
//!     api: api_instance,
//!     account: account_info,
//! );
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
//!
//! ### 使用注册表
//!
//! ```rust,ignore
//! use puniyu_bot::{Bot, BotRegistry};
//!
//! // 注册机器人
//! let index = BotRegistry::register(bot)?;
//! println!("机器人已注册，索引: {}", index);
//!
//! // 通过索引获取
//! if let Some(bot) = BotRegistry::get_with_index(index) {
//!     println!("找到机器人: {}", bot.account().uin);
//! }
//!
//! // 通过 UIN 获取
//! if let Some(bot) = BotRegistry::get_with_bot_id("123456") {
//!     println!("机器人: {}", bot.account().name);
//! }
//!
//! // 获取所有机器人
//! let all_bots = BotRegistry::all();
//! println!("共有 {} 个机器人", all_bots.len());
//!
//! // 注销机器人
//! BotRegistry::unregister(index)?;
//! ```
//!
//! ### 便捷函数
//!
//! ```rust,ignore
//! use puniyu_bot::{get_bot, get_bot_count, get_all_bot};
//!
//! // 获取机器人
//! if let Some(bot) = get_bot(123u64) {
//!     println!("找到机器人");
//! }
//!
//! // 获取机器人数量
//! let count = get_bot_count();
//! println!("共有 {} 个机器人", count);
//!
//! // 获取所有机器人
//! let all_bots = get_all_bot();
//! ```

mod registry;
pub use registry::BotRegistry;
mod macros;
mod types;

#[doc(inline)]
pub use types::*;

use puniyu_account::AccountInfo;
use puniyu_adapter_api::AdapterApi;
use puniyu_adapter_types::AdapterInfo;
use std::fmt::{Debug, Formatter};

/// 机器人实例
///
/// 封装了适配器信息、API 接口和账户信息的机器人实例。
///
/// # 字段
///
/// - `adapter` - 适配器信息，包含适配器名称、平台和协议
/// - `api` - 适配器 API 接口，用于与平台交互
/// - `account` - 机器人账户信息，包含 UIN、昵称和头像
///
/// # 示例
///
/// ```rust
/// use puniyu_bot::Bot;
/// use puniyu_account::AccountInfo;
/// use puniyu_adapter_api::AdapterApi;
/// use puniyu_adapter_types::AdapterInfo;
/// use bytes::Bytes;
///
/// let adapter = AdapterInfo {
///     name: "my_adapter".to_string(),
///     platform: "QQ".to_string(),
///     protocol: "Console".to_string(),
/// };
///
/// let api = AdapterApi::default();
///
/// let account = AccountInfo {
///     uin: "123456".to_string(),
///     name: "MyBot".to_string(),
///     avatar: Bytes::new(),
/// };
///
/// let bot = Bot::new(adapter, api, account);
///
/// assert_eq!(bot.account().uin, "123456");
/// assert_eq!(bot.adapter().name, "my_adapter");
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

/// 获取机器人实例
///
/// 通过索引或 UIN 获取已注册的机器人实例。
///
/// # 参数
///
/// - `bot_id` - 机器人标识符，可以是索引（`u64`）或 UIN（`&str`）
///
/// # 返回值
///
/// 返回匹配的机器人实例，如果未找到则返回 `None`
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_bot::get_bot;
///
/// // 使用索引查询
/// if let Some(bot) = get_bot(123u64) {
///     println!("找到机器人: {}", bot.account().uin);
/// }
///
/// // 使用 UIN 查询
/// if let Some(bot) = get_bot("123456") {
///     println!("机器人: {}", bot.account().name);
/// }
/// ```
pub fn get_bot<'a>(bot_id: impl Into<BotId<'a>>) -> Option<Bot> {
	BotRegistry::get(bot_id)
}

/// 获取已注册的机器人数量
///
/// # 返回值
///
/// 返回当前已注册的机器人总数
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_bot::get_bot_count;
///
/// let count = get_bot_count();
/// println!("共有 {} 个机器人", count);
/// ```
pub fn get_bot_count() -> usize {
	BotRegistry::all().len()
}

/// 获取所有已注册的机器人
///
/// # 返回值
///
/// 返回包含所有已注册机器人的向量
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_bot::get_all_bot;
///
/// let all_bots = get_all_bot();
/// for bot in all_bots {
///     println!("机器人: {} ({})", bot.account().name, bot.account().uin);
/// }
/// ```
pub fn get_all_bot() -> Vec<Bot> {
	BotRegistry::all()
}
