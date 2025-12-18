mod friend;
mod group;

pub use friend::FriendSender;
pub use group::GroupSender;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

#[derive(Debug, Clone, PartialEq, EnumString, Display, IntoStaticStr, Deserialize, Serialize)]
/// 性别
pub enum Sex {
	/// 男性
	#[strum(serialize = "male")]
	Male,
	/// 女性
	#[strum(serialize = "female")]
	Female,
	/// 未知
	#[strum(serialize = "unknow")]
	Unknown,
}

impl Sex {
	/// 是否为男性
	pub fn is_male(&self) -> bool {
		matches!(self, Sex::Male)
	}
	/// 是否为女性
	pub fn is_female(&self) -> bool {
		matches!(self, Sex::Female)
	}
	/// 是否为未知性别
	pub fn is_unknown(&self) -> bool {
		matches!(self, Sex::Unknown)
	}
}

#[derive(Debug, Clone, PartialEq, EnumString, Display, IntoStaticStr, Deserialize, Serialize)]
/// 事件发送者角色
pub enum Role {
	/// 群主
	#[strum(serialize = "owner")]
	Owner,
	/// 管理员
	#[strum(serialize = "admin")]
	Admin,
	/// 成员
	#[strum(serialize = "member")]
	Member,
	/// 未知
	#[strum(serialize = "unknow")]
	Unknown,
}

impl Role {
	/// 是否为群主
	pub fn is_owner(&self) -> bool {
		matches!(self, Self::Owner)
	}

	/// 是否为管理员
	pub fn is_admin(&self) -> bool {
		matches!(self, Self::Admin)
	}

	/// 是否为群成员
	pub fn is_member(&self) -> bool {
		matches!(self, Self::Member)
	}

	/// 是否为未知角色
	pub fn is_unknown(&self) -> bool {
		matches!(self, Self::Unknown)
	}
}

pub trait Sender: Send + Sync {
	/// 发送者id
	fn user_id(&self) -> &str;
	/// 发送者昵称
	fn name(&self) -> Option<&str>;
	/// 发送者性别
	fn sex(&self) -> Sex;
	/// 发送者年龄
	fn age(&self) -> u8;
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", tag = "type", content = "field0")]
pub enum SenderType {
	Friend(FriendSender),
	Group(GroupSender),
}


impl Sender for SenderType {
	fn user_id(&self) -> &str {
		match self {
			SenderType::Friend(sender) => sender.user_id(),
			SenderType::Group(sender) => sender.user_id(),
		}
	}
	fn name(&self) -> Option<&str> {
		match self {
			SenderType::Friend(sender) => sender.name(),
			SenderType::Group(sender) => sender.name(),
		}
	}
	fn sex(&self) -> Sex {
		match self {
			SenderType::Friend(sender) => sender.sex(),
			SenderType::Group(sender) => sender.sex(),
		}
	}
	fn age(&self) -> u8 {
		match self {
			SenderType::Friend(sender) => sender.age(),
			SenderType::Group(sender) => sender.age(),
		}
	}
}
