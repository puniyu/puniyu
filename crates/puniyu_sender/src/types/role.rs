use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

/// 群聊角色枚举
///
/// 表示发送者在群聊中的角色。
///
/// # 变体
///
/// - `Owner` - 群主，拥有最高权限
/// - `Admin` - 管理员，拥有管理权限
/// - `Member` - 普通成员
/// - `Unknown` - 未知角色（默认值）
///
/// # 示例
///
/// ```rust
/// use puniyu_sender::Role;
/// use std::str::FromStr;
///
/// let role = Role::Admin;
/// assert!(role.is_admin());
///
/// let role = Role::from_str("owner").unwrap();
/// assert!(role.is_owner());
/// ```
///
/// # 序列化
///
/// 该枚举实现了 `Serialize` 和 `Deserialize`，可以直接用于 JSON 序列化：
///
/// ```rust
/// use puniyu_sender::Role;
/// use serde_json;
///
/// let role = Role::Admin;
/// let json = serde_json::to_string(&role).unwrap();
/// assert!(json.contains("admin"));
/// ```
#[derive(
	Debug,
	Default,
	Clone,
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
pub enum Role {
	/// 群主
	#[strum(serialize = "owner")]
	Owner,
	/// 管理员
	#[strum(serialize = "admin")]
	Admin,
	/// 普通成员
	#[strum(serialize = "member")]
	Member,
	/// 未知角色
	#[strum(serialize = "unknow")]
	#[default]
	Unknown,
}

impl Role {
	/// 判断是否为群主
	///
	/// # 返回值
	///
	/// 如果角色为群主则返回 `true`，否则返回 `false`。
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_sender::Role;
	///
	/// let role = Role::Owner;
	/// assert!(role.is_owner());
	///
	/// let role = Role::Admin;
	/// assert!(!role.is_owner());
	/// ```
	pub fn is_owner(&self) -> bool {
		matches!(self, Self::Owner)
	}

	/// 判断是否为管理员
	///
	/// # 返回值
	///
	/// 如果角色为管理员则返回 `true`，否则返回 `false`。
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_sender::Role;
	///
	/// let role = Role::Admin;
	/// assert!(role.is_admin());
	///
	/// let role = Role::Member;
	/// assert!(!role.is_admin());
	/// ```
	pub fn is_admin(&self) -> bool {
		matches!(self, Self::Admin)
	}

	/// 判断是否为普通成员
	///
	/// # 返回值
	///
	/// 如果角色为普通成员则返回 `true`，否则返回 `false`。
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_sender::Role;
	///
	/// let role = Role::Member;
	/// assert!(role.is_member());
	///
	/// let role = Role::Admin;
	/// assert!(!role.is_member());
	/// ```
	pub fn is_member(&self) -> bool {
		matches!(self, Self::Member)
	}

	/// 判断是否为未知角色
	///
	/// # 返回值
	///
	/// 如果角色为未知则返回 `true`，否则返回 `false`。
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_sender::Role;
	///
	/// let role = Role::Unknown;
	/// assert!(role.is_unknown());
	///
	/// let role = Role::Member;
	/// assert!(!role.is_unknown());
	/// ```
	pub fn is_unknown(&self) -> bool {
		matches!(self, Self::Unknown)
	}
}
