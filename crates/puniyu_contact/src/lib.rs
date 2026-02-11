//! # puniyu_contact
//!
//! 联系人类型定义库，提供好友和群聊联系人的类型系统。
//!
//! ## 概述
//!
//! `puniyu_contact` 提供了统一的联系人类型定义，用于处理聊天机器人中的好友和群聊联系人信息。
//! 该库将联系人分为两类：
//!
//! - **好友联系人（FriendContact）** - 一对一聊天的好友信息
//! - **群聊联系人（GroupContact）** - 群组聊天的群信息
//!
//! ## 使用方式
//!
//! ### 创建好友联系人
//!
//! ```rust
//! use puniyu_contact::{FriendContact, SceneType, contact_friend};
//!
//! // 手动创建
//! let friend = FriendContact {
//!     scene: SceneType::Friend,
//!     peer: "123456",
//!     name: Some("Alice"),
//! };
//!
//! // 使用宏创建（仅包含 ID）
//! let friend = contact_friend!("123456");
//!
//! // 使用宏创建（包含 ID 和名称）
//! let friend = contact_friend!("123456", "Alice");
//! ```
//!
//! ### 创建群聊联系人
//!
//! ```rust
//! use puniyu_contact::{GroupContact, SceneType, contact_group};
//!
//! // 手动创建
//! let group = GroupContact {
//!     scene: SceneType::Group,
//!     peer: "789012",
//!     name: Some("Dev Team"),
//! };
//!
//! // 使用宏创建（仅包含 ID）
//! let group = contact_group!("789012");
//!
//! // 使用宏创建（包含 ID 和名称）
//! let group = contact_group!("789012", "Dev Team");
//! ```
//!
//! ### 使用统一的联系人类型
//!
//! ```rust
//! use puniyu_contact::{ContactType, FriendContact, GroupContact, Contact, SceneType};
//!
//! // 从好友创建
//! let friend = FriendContact {
//!     scene: SceneType::Friend,
//!     peer: "123456",
//!     name: Some("Alice"),
//! };
//! let contact = ContactType::from(friend);
//!
//! // 从群聊创建
//! let group = GroupContact {
//!     scene: SceneType::Group,
//!     peer: "789012",
//!     name: Some("Dev Team"),
//! };
//! let contact = ContactType::from(group);
//!
//! // 使用 Contact trait 方法
//! println!("Peer ID: {}", contact.peer());
//! if let Some(name) = contact.name() {
//!     println!("Name: {}", name);
//! }
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
/// use puniyu_contact::{ContactType, FriendContact, GroupContact, SceneType};
///
/// // 创建好友联系人
/// let friend = FriendContact {
///     scene: SceneType::Friend,
///     peer: "123456",
///     name: Some("Alice"),
/// };
/// let contact = ContactType::Friend(friend);
///
/// // 创建群聊联系人
/// let group = GroupContact {
///     scene: SceneType::Group,
///     peer: "789012",
///     name: Some("Dev Team"),
/// };
/// let contact = ContactType::Group(group);
/// ```
#[derive(Debug, Clone, PartialEq, Display, IntoStaticStr, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", tag = "type", content = "field0", bound(deserialize = "'de: 'c"))]
pub enum ContactType<'c> {
	/// 好友联系人
	#[strum(serialize = "friend")]
	Friend(FriendContact<'c>),
	/// 群聊联系人
	#[strum(serialize = "group")]
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
	/// 如果是好友联系人则返回 `Some`，否则返回 `None`。
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_contact::{ContactType, FriendContact, SceneType, Contact};
	///
	/// let friend = FriendContact {
	///     scene: SceneType::Friend,
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
	/// 如果是群聊联系人则返回 `Some`，否则返回 `None`。
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_contact::{ContactType, GroupContact, SceneType, Contact};
	///
	/// let group = GroupContact {
	///     scene: SceneType::Group,
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
	/// use puniyu_contact::{ContactType, FriendContact, SceneType};
	///
	/// let friend = FriendContact {
	///     scene: SceneType::Friend,
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
	/// use puniyu_contact::{ContactType, GroupContact, SceneType};
	///
	/// let group = GroupContact {
	///     scene: SceneType::Group,
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
