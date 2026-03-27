//! # puniyu_adapter_api
//!
//! 统一的 puniyu 适配器 API 库，覆盖消息、群组、好友与账户操作场景。
//!
//! ## 特性
//!
//! - 提供 [`AdapterApi`] 聚合消息、群组、好友和账户接口
//! - 提供 [`MessageApi`] 统一消息发送、撤回与历史查询接口
//! - 提供 [`GroupApi`]、[`FriendApi`]、[`AccountApi`]
//! - 支持 `AdapterApiBuilder` 自定义各子 API 实现
//!
//! ## 示例
//!
//! ```rust,ignore
//! use std::sync::Arc;
//! use puniyu_adapter_api::{AdapterApiBuilder, MessageApi};
//!
//! struct MyMessageApi;
//! impl MessageApi for MyMessageApi {}
//!
//! let api = AdapterApiBuilder::default()
//!     .message_api(Arc::new(MyMessageApi))
//!     .build()
//!     .unwrap();
//!
//! let _ = api.message();
//! ```

#![allow(unused_variables)]

mod group;
#[doc(inline)]
pub use group::GroupApi;
mod friend;
#[doc(inline)]
pub use friend::FriendApi;
mod account;
#[doc(inline)]
pub use account::AccountApi;
mod message;
#[doc(inline)]
pub use message::MessageApi;
mod inner;

use derive_builder::Builder;
use std::sync::Arc;

/// 适配器 API 聚合入口。
#[derive(Clone, Builder)]
#[builder(pattern = "owned")]
pub struct AdapterApi {
	#[builder(default = "Arc::new(inner::DefaultGroupApi)")]
	group_api: Arc<dyn GroupApi>,
	#[builder(default = "Arc::new(inner::DefaultFriendApi)")]
	friend_api: Arc<dyn FriendApi>,
	#[builder(default = "Arc::new(inner::DefaultAccountApi)")]
	account_api: Arc<dyn AccountApi>,
	#[builder(default = "Arc::new(inner::DefaultMessageApi)")]
	message_api: Arc<dyn MessageApi>,
}

impl Default for AdapterApi {
	#[inline]
	fn default() -> Self {
		Self {
			group_api: Arc::new(inner::DefaultGroupApi),
			friend_api: Arc::new(inner::DefaultFriendApi),
			account_api: Arc::new(inner::DefaultAccountApi),
			message_api: Arc::new(inner::DefaultMessageApi),
		}
	}
}

impl PartialEq for AdapterApi {
	fn eq(&self, other: &Self) -> bool {
		Arc::ptr_eq(&self.group_api, &other.group_api)
			&& Arc::ptr_eq(&self.friend_api, &other.friend_api)
			&& Arc::ptr_eq(&self.account_api, &other.account_api)
			&& Arc::ptr_eq(&self.message_api, &other.message_api)
	}
}

impl AdapterApi {
	/// 使用指定子 API 创建实例。
	pub fn new(
		group_api: Arc<dyn GroupApi>,
		friend_api: Arc<dyn FriendApi>,
		account_api: Arc<dyn AccountApi>,
		message_api: Arc<dyn MessageApi>,
	) -> Self {
		Self { group_api, friend_api, account_api, message_api }
	}

	/// 获取群组 API。
	pub fn group(&self) -> &Arc<dyn GroupApi> {
		&self.group_api
	}

	/// 获取好友 API。
	pub fn friend(&self) -> &Arc<dyn FriendApi> {
		&self.friend_api
	}

	/// 获取账户 API。
	pub fn account(&self) -> &Arc<dyn AccountApi> {
		&self.account_api
	}

	/// 获取消息 API。
	pub fn message(&self) -> &Arc<dyn MessageApi> {
		&self.message_api
	}
}
