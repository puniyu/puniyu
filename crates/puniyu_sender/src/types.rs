use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

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
	pub fn is_owner(&self) -> bool {
		matches!(self, Self::Owner)
	}

	/// 是否为管理员。
	pub fn is_admin(&self) -> bool {
		matches!(self, Self::Admin)
	}

	/// 是否为普通成员。
	pub fn is_member(&self) -> bool {
		matches!(self, Self::Member)
	}

	/// 是否为未知角色。
	pub fn is_unknown(&self) -> bool {
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
	pub fn is_male(&self) -> bool {
		matches!(self, Sex::Male)
	}

	/// 是否为女性。
	pub fn is_female(&self) -> bool {
		matches!(self, Sex::Female)
	}

	/// 是否为未知性别。
	pub fn is_unknown(&self) -> bool {
		matches!(self, Sex::Unknown)
	}
}
