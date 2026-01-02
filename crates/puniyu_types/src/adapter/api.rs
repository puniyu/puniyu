mod group;
pub use group::GroupApi;
mod friend;
pub use friend::FriendApi;
mod account;
pub use account::AccountApi;
mod message;
pub use message::MessageApi;
mod inner;

use super::{Error, Result, types::*};
use std::sync::Arc;

#[derive(Clone)]
pub struct AdapterApi {
	group_api: Arc<dyn GroupApi>,
	friend_api: Arc<dyn FriendApi>,
	account_api: Arc<dyn AccountApi>,
	message_api: Arc<dyn MessageApi>,
}

impl Default for AdapterApi {
	fn default() -> Self {
		Self {
			group_api: Arc::new(inner::DefaultGroupApi),
			friend_api: Arc::new(inner::DefaultFriendApi),
			account_api: Arc::new(inner::DefaultAccountApi),
			message_api: Arc::new(inner::DefaultMessageApi),
		}
	}
}

impl AdapterApi {
	pub fn new(
		group_api: Arc<dyn GroupApi>,
		friend_api: Arc<dyn FriendApi>,
		account_api: Arc<dyn AccountApi>,
		message_api: Arc<dyn MessageApi>,
	) -> Self {
		Self { group_api, friend_api, account_api, message_api }
	}
	
	/// 获取群实例
	pub fn group(&self) -> &dyn GroupApi {
		self.group_api.as_ref()
	}
	
	/// 获取好友实例
	pub fn friend(&self) -> &dyn FriendApi {
		self.friend_api.as_ref()
	}
	
	/// 获取账号实例
	pub fn account(&self) -> &dyn AccountApi {
		self.account_api.as_ref()
	}
	
	/// 获取消息实例
	pub fn message(&self) -> &dyn MessageApi {
		self.message_api.as_ref()
	}
}

