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
use puniyu_adapter_types::AdapterApi;
use puniyu_adapter_core::types::info::AdapterInfo;

pub trait Bot: Send + Sync + 'static {
	fn adapter(&self) -> AdapterInfo;
	fn api(&self) -> &AdapterApi;
	fn account(&self) -> &AccountInfo;
}

impl PartialEq for dyn Bot {
	fn eq(&self, other: &Self) -> bool {
		self.adapter() == other.adapter() && self.account() == other.account()
	}
}
