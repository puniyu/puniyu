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

use std::sync::Arc;

#[derive(Clone)]
pub struct AdapterApi {
	group_api: Arc<dyn GroupApi>,
	friend_api: Arc<dyn FriendApi>,
	account_api: Arc<dyn AccountApi>,
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


impl AdapterApi
{
	pub fn new(
		group_api: Arc<dyn GroupApi>,
		friend_api: Arc<dyn FriendApi>,
		account_api: Arc<dyn AccountApi>,
		message_api: Arc<dyn MessageApi>,
	) -> Self {
		Self { group_api, friend_api, account_api, message_api }
	}
	pub fn group(&self) -> &Arc<dyn GroupApi> {
		&self.group_api
	}
	
	pub fn friend(&self) -> &Arc<dyn FriendApi> {
		&self.friend_api
	}
	
	pub fn account(&self) -> &Arc<dyn AccountApi> {
		&self.account_api
	}
	
	pub fn message(&self) -> &Arc<dyn MessageApi> {
		&self.message_api
	}
}
