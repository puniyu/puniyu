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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Sender {
	Friend(FriendSender),
	Group(GroupSender),
}

impl Sender {
	pub fn as_friend(&self) -> Option<FriendSender> {
		match self {
			Sender::Friend(sender) => Some(sender.clone()),
			_ => None,
		}
	}
	pub fn as_group(&self) -> Option<GroupSender> {
		match self {
			Sender::Group(sender) => Some(sender.clone()),
			_ => None,
		}
	}
}

impl From<FriendSender> for Sender {
	fn from(sender: FriendSender) -> Self {
		Sender::Friend(sender)
	}
}

impl From<GroupSender> for Sender {
	fn from(sender: GroupSender) -> Self {
		Sender::Group(sender)
	}
}
