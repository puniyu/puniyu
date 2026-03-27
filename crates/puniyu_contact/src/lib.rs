//! # puniyu_contact
//!
//! 统一的联系人类型，覆盖好友与群聊场景。
//!
//! ## 特性
//!
//! - 提供 `FriendContact` 与 `GroupContact`
//! - 提供统一接口 `Contact`
//! - 提供统一枚举 `ContactType`
//! - 提供构建宏 `contact!`、`contact_friend!` 与 `contact_group!`
//!
//! ## 示例
//!
//! ```rust
//! use puniyu_contact::{contact, Contact, ContactType};
//!
//! let friend = contact!(Friend, peer: "123456", name: "Alice");
//! let group = contact!(Group, peer: "789012", name: "Dev Team");
//!
//! let contact = ContactType::from(friend);
//! assert!(contact.is_friend());
//! assert_eq!(contact.peer(), "123456");
//!
//! let contact = ContactType::from(group);
//! assert!(contact.is_group());
//! ```

mod friend;
#[doc(inline)]
pub use friend::*;
mod group;
#[doc(inline)]
pub use group::*;
mod types;
#[doc(inline)]
pub use types::*;

use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use strum::{Display, IntoStaticStr};

/// 联系人类型枚举
///
/// 统一的联系人类型，可以是好友或群聊。
///
/// # 示例
///
/// ```rust
/// use puniyu_contact::{ContactType, FriendContact, GroupContact};
///
/// // 创建好友联系人
/// let friend = FriendContact {
///     peer: "123456",
///     name: Some("Alice"),
/// };
/// let contact = ContactType::Friend(friend);
///
/// // 创建群聊联系人
/// let group = GroupContact {
///     peer: "789012",
///     name: Some("Dev Team"),
/// };
/// let contact = ContactType::Group(group);
/// ```
#[derive(Debug, Clone, PartialEq, Display, IntoStaticStr, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", tag = "type", content = "field0", bound(deserialize = "'de: 'c"))]
#[strum(serialize_all = "lowercase")]
pub enum ContactType<'c> {
	/// 好友联系人
	Friend(FriendContact<'c>),
	/// 群聊联系人
	Group(GroupContact<'c>),
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
	pub fn new(scene: SceneType, peer: &'c str, name: Option<&'c str>) -> Self {
		match scene {
			SceneType::Friend => ContactType::Friend(contact_friend!(peer, name)),
			SceneType::Group => ContactType::Group(contact_group!(peer, name)),
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
	///     peer: "123456",
	///     name: Some("Alice"),
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
	///     peer: "789012",
	///     name: Some("Dev Team"),
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

	/// 判断是否为好友联系人
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_contact::{ContactType, FriendContact};
	///
	/// let friend = FriendContact {
	///     peer: "123456",
	///     name: Some("Alice"),
	/// };
	/// let contact = ContactType::Friend(friend);
	///
	/// assert!(contact.is_friend());
	/// ```
	pub fn is_friend(&self) -> bool {
		matches!(self, ContactType::Friend(_))
	}

	/// 判断是否为群聊联系人
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_contact::{ContactType, GroupContact};
	///
	/// let group = GroupContact {
	///     peer: "789012",
	///     name: Some("Dev Team"),
	/// };
	/// let contact = ContactType::Group(group);
	///
	/// assert!(contact.is_group());
	/// ```
	pub fn is_group(&self) -> bool {
		matches!(self, ContactType::Group(_))
	}
}

impl<'c> Contact for ContactType<'c> {
	fn scene(&self) -> &SceneType {
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

/// 统一的联系人构建宏
///
/// 根据联系人类型（Friend 或 Group）创建相应的联系人对象。
/// 这是一个便捷宏，内部会调用 [`contact_friend!`] 或 [`contact_group!`]。
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
/// use puniyu_contact::{contact, contact_friend, contact_group};
///
/// // 使用统一宏
/// let friend = contact!(Friend, peer: "123456", name: "Alice");
/// let group = contact!(Group, peer: "789012", name: "Dev Team");
///
/// // 使用专用宏（效果相同）
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
        $crate::contact_friend!( $( $key : $value ),+ )
    };

    (Group, $( $key:ident : $value:expr ),+ $(,)?) => {
        $crate::contact_group!( $( $key : $value ),+ )
    };
}
