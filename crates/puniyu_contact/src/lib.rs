//! # puniyu_contact
//!
//! 统一的联系人类型，覆盖好友、群聊、群临时与频道场景。
//!
//! ## 特性
//!
//! - 提供 `FriendContact`、`GroupContact`、`GroupTempContact` 与 `GuildContact`
//! - 提供统一接口 `Contact`
//! - 提供统一枚举 `ContactType`
//! - 提供构建宏 `contact!`、`contact_friend!`、`contact_group!`、`contact_group_temp!` 与 `contact_guild!`
//!
//! ## 示例
//!
//! ```rust
//! use puniyu_contact::{contact, Contact};
//!
//! let friend = contact!(Friend, peer: "123456", name: "Alice");
//! let group = contact!(Group, peer: "789012", name: "Dev Team");
//! let group_temp = contact!(GroupTemp, peer: "789012", name: "Temp Team");
//! let guild = contact!(Guild, peer: "123", name: "Guild Channel", sub_name: "General");
//!
//! assert!(friend.is_friend());
//! assert_eq!(friend.peer(), "123456");
//!
//! assert!(group.is_group());
//! assert!(group_temp.is_group_temp());
//! assert!(guild.is_guild());
//! ```

mod friend;
#[doc(inline)]
pub use friend::*;
mod group;
#[doc(inline)]
pub use group::*;
mod guild;
#[doc(inline)]
pub use guild::*;
mod types;
#[doc(inline)]
pub use types::*;

use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use strum::{Display, IntoStaticStr};

pub use puniyu_core::contact::Contact;

/// 联系人类型枚举
///
/// 统一的联系人类型，可以是好友或群聊。
///
/// # 示例
///
/// ```rust
/// use puniyu_contact::{ContactType, FriendContact, GroupContact, GroupTempContact};
///
/// // 创建好友联系人
/// let friend = FriendContact::builder().peer("123456").name("Alice").build();
/// let contact = ContactType::Friend(friend);
///
/// // 创建群聊联系人
/// let group = GroupContact::builder().peer("789012").name("Dev Team").build();
/// let contact = ContactType::Group(group);
///
/// // 创建群临时联系人
/// let group_temp = GroupTempContact::builder().peer("246810").name("Temp Team").build();
/// let contact = ContactType::GroupTemp(group_temp);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Display, IntoStaticStr, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", tag = "type", content = "field0")]
#[strum(serialize_all = "lowercase")]
pub enum ContactType {
	/// 好友联系人
	Friend(FriendContact),
	/// 群聊联系人
	Group(GroupContact),
	/// 群临时联系人
	GroupTemp(GroupTempContact),
	/// 频道联系人
	Guild(GuildContact),
}

impl ContactType {
	/// 创建联系人类型
	///
	/// 根据场景类型自动创建对应的联系人。
	///
	/// # 参数
	///
	/// * `scene` - 场景类型（好友或群聊）
	/// * `peer` - 联系人 ID
	/// * `name` - 联系人名称（可选）
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_contact::{ContactType, SceneType};
	///
	/// // 创建好友联系人
	/// let friend = ContactType::new(SceneType::Friend, "123456", Some("Alice"));
	///
	/// // 创建群聊联系人
	/// let group = ContactType::new(SceneType::Group, "789012", Some("Dev Team"));
	/// ```
	pub fn new(
		scene: SceneType,
		peer: impl Into<SmolStr>,
		name: Option<impl Into<SmolStr>>,
	) -> Self {
		let peer = peer.into();
		let name = name.map(Into::into);

		match scene {
			SceneType::Friend => match name {
				Some(name) => {
					ContactType::Friend(FriendContact::builder().peer(peer).name(name).build())
				}
				None => ContactType::Friend(FriendContact::builder().peer(peer).build()),
			},
			SceneType::Group => match name {
				Some(name) => {
					ContactType::Group(GroupContact::builder().peer(peer).name(name).build())
				}
				None => ContactType::Group(GroupContact::builder().peer(peer).build()),
			},
			SceneType::GroupTemp => match name {
				Some(name) => ContactType::GroupTemp(
					GroupTempContact::builder().peer(peer).name(name).build(),
				),
				None => ContactType::GroupTemp(GroupTempContact::builder().peer(peer).build()),
			},
			SceneType::Guild => match name {
				Some(name) => {
					ContactType::Guild(GuildContact::builder().peer(peer).name(name).build())
				}
				None => ContactType::Guild(GuildContact::builder().peer(peer).build()),
			},
		}
	}

	/// 尝试获取好友联系人的引用
	pub fn as_friend(&self) -> Option<&FriendContact> {
		match self {
			ContactType::Friend(f) => Some(f),
			_ => None,
		}
	}

	/// 尝试获取群聊联系人的引用
	pub fn as_group(&self) -> Option<&GroupContact> {
		match self {
			ContactType::Group(g) => Some(g),
			_ => None,
		}
	}

	/// 尝试获取群临时联系人的引用
	pub fn as_group_temp(&self) -> Option<&GroupTempContact> {
		match self {
			ContactType::GroupTemp(g) => Some(g),
			_ => None,
		}
	}

	/// 尝试获取频道联系人的引用
	pub fn as_guild(&self) -> Option<&GuildContact> {
		match self {
			ContactType::Guild(g) => Some(g),
			_ => None,
		}
	}

	pub fn is_friend(&self) -> bool {
		matches!(self, ContactType::Friend(_))
	}

	pub fn is_group(&self) -> bool {
		matches!(self, ContactType::Group(_))
	}

	pub fn is_group_temp(&self) -> bool {
		matches!(self, ContactType::GroupTemp(_))
	}

	pub fn is_guild(&self) -> bool {
		matches!(self, ContactType::Guild(_))
	}
}

impl Contact for ContactType {
	type Scene = SceneType;
	fn scene(&self) -> Self::Scene {
		match self {
			Self::Friend(contact) => contact.scene(),
			Self::Group(contact) => contact.scene(),
			Self::GroupTemp(contact) => contact.scene(),
			Self::Guild(contact) => contact.scene(),
		}
	}

	fn peer(&self) -> &str {
		match self {
			Self::Friend(contact) => contact.peer(),
			Self::Group(contact) => contact.peer(),
			Self::GroupTemp(contact) => contact.peer(),
			Self::Guild(contact) => contact.peer(),
		}
	}

	fn name(&self) -> Option<&str> {
		match self {
			Self::Friend(contact) => contact.name(),
			Self::Group(contact) => contact.name(),
			Self::GroupTemp(contact) => contact.name(),
			Self::Guild(contact) => contact.name(),
		}
	}
}

impl From<FriendContact> for ContactType {
	#[inline]
	fn from(value: FriendContact) -> Self {
		Self::Friend(value)
	}
}

impl From<GroupContact> for ContactType {
	#[inline]
	fn from(value: GroupContact) -> Self {
		Self::Group(value)
	}
}

impl From<GroupTempContact> for ContactType {
	#[inline]
	fn from(value: GroupTempContact) -> Self {
		Self::GroupTemp(value)
	}
}

impl From<GuildContact> for ContactType {
	#[inline]
	fn from(value: GuildContact) -> Self {
		Self::Guild(value)
	}
}

impl From<&FriendContact> for ContactType {
	#[inline]
	fn from(value: &FriendContact) -> Self {
		Self::Friend(FriendContact::builder().peer(value.peer()).maybe_name(value.name()).build())
	}
}

impl From<&GroupContact> for ContactType {
	#[inline]
	fn from(value: &GroupContact) -> Self {
		Self::Group(GroupContact::builder().peer(value.peer()).maybe_name(value.name()).build())
	}
}

impl From<&GroupTempContact> for ContactType {
	#[inline]
	fn from(value: &GroupTempContact) -> Self {
		Self::GroupTemp(
			GroupTempContact::builder().peer(value.peer()).maybe_name(value.name()).build(),
		)
	}
}

impl From<&GuildContact> for ContactType {
	#[inline]
	fn from(value: &GuildContact) -> Self {
		Self::Guild(
			GuildContact::builder()
				.peer(value.peer())
				.maybe_name(value.name())
				.maybe_sub_name(value.sub_name())
				.build(),
		)
	}
}

/// 统一的联系人构建宏
///
/// 根据联系人类型（Friend、Group、GroupTemp 或 Guild）创建相应的联系人对象，
/// 并返回统一类型 [`ContactType`]。
///
/// # 语法
///
/// ```text
/// contact!(Friend, field: value, ...)
/// contact!(Group, field: value, ...)
/// contact!(GroupTemp, field: value, ...)
/// contact!(Guild, field: value, ...)
/// ```
///
/// # 参数
///
/// - 第一个参数：联系人类型，必须是 `Friend`、`Group`、`GroupTemp` 或 `Guild`
/// - 后续参数：字段名和值的键值对
///   - `peer`: 联系人 ID（必需）
///   - `name`: 联系人名称（可选）
/// - 当类型为 `Guild` 时，还支持：
///   - `sub_name`: 子频道名称（可选）
///
/// # 示例
///
/// ## 创建好友联系人
///
/// ```rust
/// use puniyu_contact::contact;
///
/// let contact = contact!(Friend, peer: "123456", name: "Alice");
/// assert!(contact.is_friend());
/// ```
///
/// ## 创建群聊联系人
///
/// ```rust
/// use puniyu_contact::contact;
///
/// let contact = contact!(Group, peer: "789012", name: "Dev Team");
/// assert!(contact.is_group());
/// ```
///
/// ## 与专用宏的对比
///
/// ```rust
/// use puniyu_contact::{contact, contact_friend, contact_group, ContactType};
///
/// // 使用统一宏
/// let friend = contact!(Friend, peer: "123456", name: "Alice");
/// let group = contact!(Group, peer: "789012", name: "Dev Team");
/// assert!(matches!(friend, ContactType::Friend(_)));
/// assert!(matches!(group, ContactType::Group(_)));
///
/// // 使用专用宏（返回具体联系人类型）
/// let friend = contact_friend!(peer: "123456", name: "Alice");
/// let group = contact_group!(peer: "789012", name: "Dev Team");
/// ```
///
/// # 注意
///
/// - 此宏只支持命名字段语法，不支持位置参数
/// - 如果需要使用位置参数（如 `contact_friend!("123456")`），请直接使用专用宏
#[macro_export]
macro_rules! contact {
	(Friend, $( $key:ident : $value:expr ),+ $(,)? ) => {
		$crate::ContactType::Friend($crate::contact_friend!($( $key : $value ),+))
	};
	(Group, $( $key:ident : $value:expr ),+ $(,)? ) => {
		$crate::ContactType::Group($crate::contact_group!($( $key : $value ),+))
	};
	(GroupTemp, $( $key:ident : $value:expr ),+ $(,)? ) => {
		$crate::ContactType::GroupTemp($crate::contact_group_temp!($( $key : $value ),+))
	};
	(Guild, $( $key:ident : $value:expr ),+ $(,)? ) => {
		$crate::ContactType::Guild($crate::contact_guild!($( $key : $value ),+))
	};
}
