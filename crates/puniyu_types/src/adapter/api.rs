mod group;
pub use group::*;
mod friend;
pub use friend::*;
mod account;
pub use account::*;
mod message;
pub use message::*;
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
}

impl GroupApi for AdapterApi {}
impl FriendApi for AdapterApi {}
impl AccountApi for AdapterApi {}
impl MessageApi for AdapterApi {}
