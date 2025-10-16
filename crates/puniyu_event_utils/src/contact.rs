mod friend;
mod group;

pub use friend::FriendContact;
pub use group::GroupContact;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

#[derive(Debug, Clone, PartialEq, EnumString, Display, IntoStaticStr, Deserialize, Serialize)]
#[strum(serialize_all = "snake_case")]
/// 场景
pub enum Scene {
	#[strum(serialize = "group")]
	Group,
	#[strum(serialize = "friend")]
	Friend,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
/// 联系人
pub enum Contact {
	/// 好友
	Friend(FriendContact),
	/// 群聊
	Group(GroupContact),
}
