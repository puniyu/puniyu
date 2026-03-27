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

/// 适配器 API
///
/// 提供统一的 API 接口，包含消息、群组、好友和账户管理功能。
///
/// # 组成部分
///
/// - `MessageApi` - 消息相关操作
/// - `GroupApi` - 群组管理操作
/// - `FriendApi` - 好友管理操作
/// - `AccountApi` - 账户管理操作
///
/// # 示例
///
/// ## 使用默认实现
///
/// ```rust
/// use puniyu_adapter_core::AdapterApi;
///
/// let api = AdapterApi::default();
/// ```
///
/// ## 使用 Builder 模式
///
/// ```rust,ignore
/// use puniyu_adapter_core::{AdapterApi, AdapterApiBuilder};
/// use std::sync::Arc;
///
/// // 全部使用默认实现
/// let api = AdapterApiBuilder::default().build().unwrap();
///
/// // 自定义部分 API
/// let api = AdapterApiBuilder::default()
///     .group_api(Arc::new(MyGroupApi))
///     .build()
///     .unwrap();
/// ```
///
/// ## 使用消息 API
///
/// ```rust,ignore
/// use puniyu_adapter_core::AdapterApi;
/// use puniyu_contact::Contact;
/// use puniyu_message::Message;
///
/// async fn send_message(api: &AdapterApi) {
///     let contact = Contact::friend("123456");
///     let message = Message::from("Hello!");
///
///     let result = api.message().send_msg(&contact, &message).await;
///     match result {
///         Ok(info) => println!("消息已发送: {}", info.message_id),
///         Err(e) => eprintln!("发送失败: {}", e),
///     }
/// }
/// ```
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
	/// 创建新的 AdapterApi 实例
	///
	/// # 参数
	///
	/// - `group_api` - 群组 API 实现
	/// - `friend_api` - 好友 API 实现
	/// - `account_api` - 账户 API 实现
	/// - `message_api` - 消息 API 实现
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_adapter_core::api::AdapterApi;
	/// use std::sync::Arc;
	///
	/// let api = AdapterApi::new(
	///     Arc::new(MyGroupApi),
	///     Arc::new(MyFriendApi),
	///     Arc::new(MyAccountApi),
	///     Arc::new(MyMessageApi),
	/// );
	/// ```
	pub fn new(
		group_api: Arc<dyn GroupApi>,
		friend_api: Arc<dyn FriendApi>,
		account_api: Arc<dyn AccountApi>,
		message_api: Arc<dyn MessageApi>,
	) -> Self {
		Self { group_api, friend_api, account_api, message_api }
	}

	/// 获取群组 API
	///
	/// # 返回值
	///
	/// 返回群组 API 的引用
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let group_api = api.group();
	/// let groups = group_api.get_group_list().await?;
	/// ```
	pub fn group(&self) -> &Arc<dyn GroupApi> {
		&self.group_api
	}

	/// 获取好友 API
	///
	/// # 返回值
	///
	/// 返回好友 API 的引用
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let friend_api = api.friend();
	/// let friends = friend_api.get_friend_list().await?;
	/// ```
	pub fn friend(&self) -> &Arc<dyn FriendApi> {
		&self.friend_api
	}

	/// 获取账户 API
	///
	/// # 返回值
	///
	/// 返回账户 API 的引用
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let account_api = api.account();
	/// account_api.set_avatar(avatar_data).await?;
	/// ```
	pub fn account(&self) -> &Arc<dyn AccountApi> {
		&self.account_api
	}

	/// 获取消息 API
	///
	/// # 返回值
	///
	/// 返回消息 API 的引用
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let message_api = api.message();
	/// message_api.send_msg(&contact, message).await?;
	/// ```
	pub fn message(&self) -> &Arc<dyn MessageApi> {
		&self.message_api
	}
}
