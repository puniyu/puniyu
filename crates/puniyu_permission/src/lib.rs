use serde_repr::{Deserialize_repr, Serialize_repr};
use strum::{Display, EnumString, IntoStaticStr};

/// 权限级别（参考 Koishi 权限系统，数字越大权限越高）。
///
/// - Level 0: 未注册/默认用户（受限）
/// - Level 1: 已注册用户（基础命令）
/// - Level 2: 高级用户（几乎全部命令）
/// - Level 3: Bot 管理员（可配置 Bot）
/// - Level 4: 超级管理员（管理账号和权限）
#[repr(u8)]
#[derive(
	Debug,
	Default,
	Copy,
	Clone,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	EnumString,
	Display,
	IntoStaticStr,
	Serialize_repr,
	Deserialize_repr,
)]
#[strum(serialize_all = "lowercase")]
pub enum Permission {
	/// 受限用户
	#[default]
	User = 0,
	/// 已注册用户
	Member = 1,
	/// 高级用户
	Trusted = 2,
	/// Bot 管理员
	Admin = 3,
	/// 超级管理员
	SuperAdmin = 4,
}

impl Permission {
	/// 判断当前权限是否满足目标权限
	pub const fn satisfies(self, required: Self) -> bool {
		(self as u8) >= (required as u8)
	}
}
