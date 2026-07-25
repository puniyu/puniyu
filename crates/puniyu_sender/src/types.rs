use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};


pub trait Sender: Send + Sync {
	/// 获取发送者id
	fn user_id(&self) -> &str;
	/// 获取发送者昵称
	fn name(&self) -> Option<&str>;
	/// 获取发送者性别
	fn sex(&self) -> Option<Sex>;
	/// 获取发送者年龄
	fn age(&self) -> Option<u32>;
}

impl<T: Sender + ?Sized> Sender for &T {
	fn user_id(&self) -> &str {
		(**self).user_id()
	}
	fn name(&self) -> Option<&str> {
		(**self).name()
	}
	fn sex(&self) -> Option<Sex> {
		(**self).sex()
	}
	fn age(&self) -> Option<u32> {
		(**self).age()
	}
}

impl PartialEq for dyn Sender {
	fn eq(&self, other: &Self) -> bool {
		self.user_id() == other.user_id()
			&& self.name() == other.name()
			&& self.sex() == other.sex()
			&& self.age() == other.age()
	}
}

impl Eq for dyn Sender {}

/// 角色
#[derive(
	Debug,
	Default,
	Clone,
	Hash,
	Copy,
	PartialEq,
	Eq,
	EnumString,
	Display,
	IntoStaticStr,
	Deserialize,
	Serialize,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Role {
	/// 群主/频道主
	Owner,
	/// 管理员
	Admin,
	/// 群成员、频道成员
	Member,
	/// 未知角色
	#[default]
	Unknown,
}

impl Role {
	/// 是否为群主。
	pub const fn is_owner(&self) -> bool {
		matches!(self, Self::Owner)
	}

	/// 是否为管理员。
	pub const fn is_admin(&self) -> bool {
		matches!(self, Self::Admin)
	}

	/// 是否为普通成员。
	pub const fn is_member(&self) -> bool {
		matches!(self, Self::Member)
	}

	/// 是否为未知角色。
	pub const fn is_unknown(&self) -> bool {
		matches!(self, Self::Unknown)
	}
}

/// 性别
#[derive(
	Debug,
	Default,
	Clone,
	Copy,
	Hash,
	PartialEq,
	Eq,
	EnumString,
	Display,
	IntoStaticStr,
	Deserialize,
	Serialize,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Sex {
	/// 男性
	Male,
	/// 女性
	Female,
	/// 未知性别
	#[default]
	Unknown,
}

impl Sex {
	/// 是否为男性。
	pub const fn is_male(&self) -> bool {
		matches!(self, Self::Male)
	}

	/// 是否为女性。
	pub const fn is_female(&self) -> bool {
		matches!(self, Self::Female)
	}

	/// 是否为未知性别。
	pub const fn is_unknown(&self) -> bool {
		matches!(self, Self::Unknown)
	}
}
