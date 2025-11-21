mod friend;
mod group;

pub use friend::FriendContact;
pub use group::GroupContact;

use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use strum::{Display, EnumString, IntoStaticStr};

#[derive(Debug, Clone, PartialEq, EnumString, Display, IntoStaticStr, Deserialize, Serialize)]
/// 场景
pub enum Scene {
	#[strum(serialize = "group")]
	/// 群聊场景
	Group,
	#[strum(serialize = "friend")]
	/// 好友场景
	Friend,
}

/// 联系人
pub trait Contact: Send + Sync + Debug + Clone {
	/// 事件来源
	fn scene(&self) -> Scene;
	/// 联系人ID
	fn peer(&self) -> &str;
	/// 联系人名称
	fn name(&self) -> Option<&str>;
	/// 判断是否为好友
	fn is_friend(&self) -> bool;
	/// 判断是否为群聊
	fn is_group(&self) -> bool;
}

/// 联系人枚举
#[derive(Debug, Clone, PartialEq, Display, IntoStaticStr, Deserialize, Serialize)]
pub enum ContactType {
	/// 好友联系人
	#[strum(serialize = "friend")]
	Friend(FriendContact),
	/// 群聊联系人
	#[strum(serialize = "group")]
	Group(GroupContact),
}

impl Contact for ContactType {
	fn scene(&self) -> Scene {
		match self {
			ContactType::Friend(f) => f.scene(),
			ContactType::Group(g) => g.scene(),
		}
	}

	fn peer(&self) -> &str {
		match self {
			ContactType::Friend(f) => f.peer(),
			ContactType::Group(g) => g.peer(),
		}
	}

	fn name(&self) -> Option<&str> {
		match self {
			ContactType::Friend(f) => f.name(),
			ContactType::Group(g) => g.name(),
		}
	}

	fn is_friend(&self) -> bool {
		matches!(self, ContactType::Friend(_))
	}

	fn is_group(&self) -> bool {
		matches!(self, ContactType::Group(_))
	}
}

impl From<FriendContact> for ContactType {
	fn from(contact: FriendContact) -> Self {
		ContactType::Friend(contact)
	}
}


impl From<GroupContact> for ContactType {
	fn from(contact: GroupContact) -> Self {
		ContactType::Group(contact)
	}
}
