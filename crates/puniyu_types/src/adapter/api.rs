mod group;
pub use group::GroupApi;
mod friend;
pub use friend::FriendApi;
mod account;
pub use account::AccountApi;
mod message;
pub use message::MessageApi;
mod inner;

use std::sync::Arc;
use super::{Error, Result, types::*};

#[derive(Clone)]
pub struct AdapterApi {
	pub friend_api: Arc<dyn FriendApi>,
	pub account_api: Arc<dyn AccountApi>,
	pub message_api: Arc<dyn MessageApi>,
}

impl Default for AdapterApi {
	fn default() -> Self {
		Self {
			friend_api: Arc::new(inner::EmptyFriendApi),
			account_api: Arc::new(inner::EmptyAccountApi),
			message_api: Arc::new(inner::EmptyMessageApi),
		}
	}
}

impl AdapterApi { 
	pub fn new(
		friend_api: Arc<dyn FriendApi>,
		account_api: Arc<dyn AccountApi>,
		message_api: Arc<dyn MessageApi>,
	) -> Self {
		Self {
			friend_api,
			account_api,
			message_api,
		}
	}
	
	pub fn friend(&self) -> &dyn FriendApi {
		self.friend_api.as_ref()
	}
	
	pub fn account(&self) -> &dyn AccountApi {
		self.account_api.as_ref()
	}
	
	pub fn message(&self) -> &dyn MessageApi {
		self.message_api.as_ref()
	}
}
