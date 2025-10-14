mod friend;
mod group;

pub use friend::FriendContact;
pub use group::GroupContact;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

#[derive(Debug, Clone, PartialEq, EnumString, Display, IntoStaticStr, Deserialize, Serialize)]
#[strum(serialize_all = "snake_case")]
pub enum Scene {
	#[strum(serialize = "group")]
	Group,
	#[strum(serialize = "friend")]
	Friend,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Contact {
	/// 好友
	Friend(FriendContact),
	/// 群聊
	Group(GroupContact),
}
