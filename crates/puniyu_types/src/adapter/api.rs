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
use crate::contact::ContactType;
use crate::element::Message;
use async_trait::async_trait;
use bytes::Bytes;
use std::sync::Arc;
use std::time::Duration;

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

macro_rules! impl_api{
    (
        $impl_type:ty,
        $field:ident,
        $trait_name:ident,
        $($method_name:ident( $($arg_name:ident: $arg_type:ty),* $(,)? ) -> $return_type:ty),* $(,)?
    ) => {
        #[async_trait]
        impl $trait_name for $impl_type {
            $(
                async fn $method_name(&self, $($arg_name: $arg_type),*) -> $return_type {
                    self.$field.$method_name($($arg_name),*).await
                }
            )*
        }
    };
}

impl_api!(
	AdapterApi,
	group_api,
	GroupApi,
	get_group_avatar(group_id: &str, size: Option<AvatarSize>) -> Result<Avatar>,
	get_group_info(group_id: &str) -> Result<GroupInfo>,
	get_group_list() -> Result<Vec<GroupInfo>>,
	get_group_member_list(group_id: &str) -> Result<Vec<UserInfo>>,
	get_group_mute_list(group_id: &str) -> Result<Vec<GroupMuteInfo>>,
	set_group_name(group_id: &str, name: &str) -> Result<()>,
	set_kick_group_member(
		group_id: &str,
		target_id: &str,
		reject_add_request: Option<bool>,
		reason: Option<&str>
	) -> Result<()>,
	set_group_mute(group_id: &str, target_id: &str, duration: Duration) -> Result<()>,
	set_group_all_mute(group_id: &str, action: MuteType) -> Result<()>,
	set_group_admin(group_id: &str, target_id: &str, action: SetAdminType) -> Result<()>,
	set_group_quit(group_id: &str) -> Result<()>,
	set_group_invited_join(group_id: &str, action: SetGroupApplyType) -> Result<()>
);

impl_api!(
	AdapterApi,
	friend_api,
	FriendApi,
	get_user_avatar(target_id: &str, size: Option<AvatarSize>) -> Result<Avatar>,
	get_friend_list() -> Result<Vec<UserInfo>>,
	set_friend_apply(action: SetFriendApplyType) -> Result<()>
);

impl_api!(
	AdapterApi,
	account_api,
	AccountApi,
	set_avatar(avatar: Bytes) -> Result<bool>
);

impl_api!(
	AdapterApi,
	message_api,
	MessageApi,
	send_msg(contact: ContactType, message: Message) -> Result<SendMsgType>,
	recall_msg(message_id: &str) -> Result<()>,
	get_msg(message_id: &str) -> Result<MessageType>,
	get_history_msg(contact: ContactType, message: MessageType, count: u8) -> Result<Vec<MessageInfo>>
);
