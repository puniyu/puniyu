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
use std::borrow::Cow;
use std::fmt::Debug;
use strum::{Display, IntoStaticStr};

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
/// let friend = FriendContact {
///     peer: "123456".into(),
///     name: Some("Alice".into()),
/// };
/// let contact = ContactType::Friend(friend);
///
/// // 创建群聊联系人
/// let group = GroupContact {
///     peer: "789012".into(),
///     name: Some("Dev Team".into()),
/// };
/// let contact = ContactType::Group(group);
///
/// // 创建群临时联系人
/// let group_temp = GroupTempContact {
///     peer: "246810".into(),
///     name: Some("Temp Team".into()),
/// };
/// let contact = ContactType::GroupTemp(group_temp);
/// ```
#[derive(Debug, Clone, PartialEq, Display, IntoStaticStr, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", tag = "type", content = "field0", bound(deserialize = "'de: 'c"))]
#[strum(serialize_all = "lowercase")]
pub enum ContactType<'c> {
	/// 好友联系人
	Friend(FriendContact<'c>),
	/// 群聊联系人
	Group(GroupContact<'c>),
	/// 群临时联系人
	GroupTemp(GroupTempContact<'c>),
	/// 频道联系人
	Guild(GuildContact<'c>),
}

impl<'c> ContactType<'c> {
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
	pub fn new<P, N>(scene: SceneType, peer: P, name: Option<N>) -> Self
	where
		P: Into<Cow<'c, str>>,
		N: Into<Cow<'c, str>>,
	{
		let peer = peer.into();
		let name = name.map(Into::into);

		match scene {
			SceneType::Friend => match name {
				Some(name) => ContactType::Friend(
					FriendContactBuilder::default()
						.peer(peer)
						.name(name)
						.build()
						.expect("Failed to build FriendContact"),
				),
				None => ContactType::Friend(
					FriendContactBuilder::default()
						.peer(peer)
						.build()
						.expect("Failed to build FriendContact"),
				),
			},
			SceneType::Group => match name {
				Some(name) => ContactType::Group(
					GroupContactBuilder::default()
						.peer(peer)
						.name(name)
						.build()
						.expect("Failed to build GroupContact"),
				),
				None => ContactType::Group(
					GroupContactBuilder::default()
						.peer(peer)
						.build()
						.expect("Failed to build GroupContact"),
				),
			},
			SceneType::GroupTemp => match name {
				Some(name) => ContactType::GroupTemp(
					GroupTempContactBuilder::default()
						.peer(peer)
						.name(name)
						.build()
						.expect("Failed to build GroupTempContact"),
				),
				None => ContactType::GroupTemp(
					GroupTempContactBuilder::default()
						.peer(peer)
						.build()
						.expect("Failed to build GroupTempContact"),
				),
			},
			SceneType::Guild => match name {
				Some(name) => ContactType::Guild(
					GuildContact::builder()
						.peer(peer)
						.name(name)
						.build()
						.expect("Failed to build GuildContact"),
				),
				None => ContactType::Guild(
					GuildContact::builder()
						.peer(peer)
						.build()
						.expect("Failed to build GuildContact"),
				),
			},
		}
	}

	/// 尝试获取好友联系人的引用
	///
	/// 如果是好友联系人则返回 [`Some`]，否则返回 [`None`]。
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_contact::{ContactType, FriendContact, Contact};
	///
	/// let friend = FriendContact {
	///     peer: "123456".into(),
	///     name: Some("Alice".into()),
	/// };
	/// let contact = ContactType::Friend(friend);
	///
	/// if let Some(f) = contact.as_friend() {
	///     println!("Friend: {}", f.peer());
	/// }
	/// ```
	pub fn as_friend(&self) -> Option<&FriendContact<'c>> {
		match self {
			ContactType::Friend(f) => Some(f),
			_ => None,
		}
	}

	/// 尝试获取群聊联系人的引用
	///
	/// 如果是群聊联系人则返回 [`Some`]，否则返回 [`None`]。
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_contact::{ContactType, GroupContact, Contact};
	///
	/// let group = GroupContact {
	///     peer: "789012".into(),
	///     name: Some("Dev Team".into()),
	/// };
	/// let contact = ContactType::Group(group);
	///
	/// if let Some(g) = contact.as_group() {
	///     println!("Group: {}", g.peer());
	/// }
	/// ```
	pub fn as_group(&self) -> Option<&GroupContact<'c>> {
		match self {
			ContactType::Group(g) => Some(g),
			_ => None,
		}
	}

	/// 尝试获取群临时联系人的引用
	pub fn as_group_temp(&self) -> Option<&GroupTempContact<'c>> {
		match self {
			ContactType::GroupTemp(g) => Some(g),
			_ => None,
		}
	}

	/// 尝试获取频道联系人的引用
	pub fn as_guild(&self) -> Option<&GuildContact<'c>> {
		match self {
			ContactType::Guild(g) => Some(g),
			_ => None,
		}
	}
}

impl<'c> Contact for ContactType<'c> {
	fn scene(&self) -> &SceneType {
		match self {
			ContactType::Friend(_) => &SceneType::Friend,
			ContactType::Group(_) => &SceneType::Group,
			ContactType::GroupTemp(_) => &SceneType::GroupTemp,
			ContactType::Guild(_) => &SceneType::Guild,
		}
	}

	fn peer(&self) -> &str {
		match self {
			ContactType::Friend(f) => f.peer(),
			ContactType::Group(g) => g.peer(),
			ContactType::GroupTemp(g) => g.peer(),
			ContactType::Guild(g) => g.peer(),
		}
	}

	fn name(&self) -> Option<&str> {
		match self {
			ContactType::Friend(f) => f.name(),
			ContactType::Group(g) => g.name(),
			ContactType::GroupTemp(g) => g.name(),
			ContactType::Guild(g) => g.name(),
		}
	}
}

impl<'c> From<FriendContact<'c>> for ContactType<'c> {
	fn from(contact: FriendContact<'c>) -> Self {
		Self::Friend(contact)
	}
}

impl<'c> From<GroupContact<'c>> for ContactType<'c> {
	fn from(contact: GroupContact<'c>) -> Self {
		Self::Group(contact)
	}
}

impl<'c> From<GroupTempContact<'c>> for ContactType<'c> {
	fn from(contact: GroupTempContact<'c>) -> Self {
		Self::GroupTemp(contact)
	}
}

impl<'c> From<GuildContact<'c>> for ContactType<'c> {
	fn from(contact: GuildContact<'c>) -> Self {
		Self::Guild(contact)
	}
}

/// 统一的联系人构建宏
///
/// 根据联系人类型（Friend 或 Group）创建相应的联系人对象。
/// 这是一个便捷宏，内部会调用 [`contact_friend!`] 或 [`contact_group!`]，
/// 并返回统一类型 [`ContactType`]。
///
/// # 语法
///
/// ```text
/// contact!(Friend, field: value, ...)
/// contact!(Group, field: value, ...)
/// ```
///
/// # 参数
///
/// - 第一个参数：联系人类型，必须是 `Friend` 或 `Group`
/// - 后续参数：字段名和值的键值对
///   - `peer`: 联系人 ID（必需）
///   - `name`: 联系人名称（可选）
///
/// # 示例
///
/// ## 创建好友联系人
///
/// ```rust
/// use puniyu_contact::contact;
///
/// // 仅指定 peer
/// let friend = contact!(Friend, peer: "123456");
///
/// // 指定 peer 和 name
/// let friend = contact!(Friend, peer: "123456", name: "Alice");
/// ```
///
/// ## 创建群聊联系人
///
/// ```rust
/// use puniyu_contact::contact;
///
/// // 仅指定 peer
/// let group = contact!(Group, peer: "789012");
///
/// // 指定 peer 和 name
/// let group = contact!(Group, peer: "789012", name: "Dev Team");
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
/// - 如果需要使用位置参数（如 `contact_friend!("123456", "Alice")`），请直接使用专用宏
#[macro_export]
macro_rules! contact {
    (Friend, $( $key:ident : $value:expr ),+ $(,)?) => {
        $crate::ContactType::Friend($crate::contact_friend!( $( $key : $value ),+ ))
    };

    (Group, $( $key:ident : $value:expr ),+ $(,)?) => {
        $crate::ContactType::Group($crate::contact_group!( $( $key : $value ),+ ))
    };

    (GroupTemp, $( $key:ident : $value:expr ),+ $(,)?) => {
        $crate::ContactType::GroupTemp($crate::contact_group_temp!( $( $key : $value ),+ ))
    };

    (Guild, $( $key:ident : $value:expr ),+ $(,)?) => {
        $crate::ContactType::Guild($crate::contact_guild!( $( $key : $value ),+ ))
    };
}
