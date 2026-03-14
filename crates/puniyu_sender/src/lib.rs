//! # puniyu_sender
//!
//! 消息发送者类型定义。
//!
//! ## 使用
//!
//! ```rust
//! use puniyu_sender::{sender_friend, sender_group, Role, Sex};
//!
//! // 创建好友发送者
//! let friend = sender_friend!(
//!     user_id: "123456",
//!     nick: "Alice",
//!     sex: Sex::Female,
//! );
//!
//! // 创建群聊发送者
//! let group = sender_group!(
//!     user_id: "789012",
//!     nick: "Bob",
//!     role: Role::Admin,
//! );
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
use strum::{Display, IntoStaticStr};

/// 发送者类型枚举
///
/// 统一的发送者类型，可以是好友或群聊发送者。
///
/// # 示例
///
/// ```rust
/// use puniyu_sender::{SenderType, FriendSender, Sex};
///
/// // 创建好友发送者
/// let friend = FriendSender {
///     user_id: "123456",
///     nick: Some("Alice"),
///     sex: Sex::Female,
///     age: Some(25),
/// };
/// let sender = SenderType::Friend(friend);
/// ```
#[derive(Debug, Clone, PartialEq, Display, IntoStaticStr, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", tag = "type", content = "field0", bound(deserialize = "'de: 's"))]
pub enum SenderType<'s> {
	/// 好友发送者
	Friend(FriendSender<'s>),
	/// 群聊发送者
	Group(GroupSender<'s>),
}

impl SenderType<'_> {
	/// 尝试获取好友发送者的引用
	///
	/// 如果是好友发送者则返回 [`Some`]，否则返回 [`None`]。
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_sender::{SenderType, FriendSender, Sender, Sex};
	///
	/// let friend = FriendSender {
	///     user_id: "123456",
	///     nick: Some("Alice"),
	///     sex: Sex::Female,
	///     age: Some(25),
	/// };
	/// let sender = SenderType::Friend(friend);
	///
	/// if let Some(f) = sender.as_friend() {
	///     println!("Friend: {}", f.user_id());
	/// }
	/// ```
	pub fn as_friend(&self) -> Option<&FriendSender<'_>> {
		match self {
			SenderType::Friend(sender) => Some(sender),
			_ => None,
		}
	}

	/// 尝试获取群聊发送者的引用
	///
	/// 如果是群聊发送者则返回 [`Some`]，否则返回 [`None`]。
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_sender::{SenderType, GroupSender, Sender, Sex, Role};
	///
	/// let group = GroupSender {
	///     user_id: "123456",
	///     nick: Some("Alice"),
	///     sex: Sex::Female,
	///     age: Some(25),
	///     role: Role::Member,
	///     card: None,
	///     level: None,
	///     title: None,
	/// };
	/// let sender = SenderType::Group(group);
	///
	/// if let Some(g) = sender.as_group() {
	///     println!("Group member: {}", g.user_id());
	/// }
	/// ```
	pub fn as_group(&self) -> Option<&GroupSender<'_>> {
		match self {
			SenderType::Group(sender) => Some(sender),
			_ => None,
		}
	}

	/// 判断是否为好友发送者
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_sender::{SenderType, FriendSender, Sex};
	///
	/// let friend = FriendSender {
	///     user_id: "123456",
	///     nick: Some("Alice"),
	///     sex: Sex::Female,
	///     age: Some(25),
	/// };
	/// let sender = SenderType::Friend(friend);
	///
	/// assert!(sender.is_friend());
	/// ```
	pub fn is_friend(&self) -> bool {
		matches!(self, SenderType::Friend(_))
	}

	/// 判断是否为群聊发送者
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_sender::{SenderType, GroupSender, Sex, Role};
	///
	/// let group = GroupSender {
	///     user_id: "123456",
	///     nick: Some("Alice"),
	///     sex: Sex::Female,
	///     age: Some(25),
	///     role: Role::Member,
	///     card: None,
	///     level: None,
	///     title: None,
	/// };
	/// let sender = SenderType::Group(group);
	///
	/// assert!(sender.is_group());
	/// ```
	pub fn is_group(&self) -> bool {
		matches!(self, SenderType::Group(_))
	}
}

impl<'s> Sender for SenderType<'s> {
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
	fn sex(&self) -> &Sex {
		match &self {
			SenderType::Friend(sender) => sender.sex(),
			SenderType::Group(sender) => sender.sex(),
		}
	}
	fn age(&self) -> Option<u32> {
		match self {
			SenderType::Friend(sender) => sender.age(),
			SenderType::Group(sender) => sender.age(),
		}
	}
}

impl<'s> From<FriendSender<'s>> for SenderType<'s> {
	fn from(sender: FriendSender<'s>) -> Self {
		Self::Friend(sender)
	}
}

impl<'s> From<GroupSender<'s>> for SenderType<'s> {
	fn from(sender: GroupSender<'s>) -> Self {
		Self::Group(sender)
	}
}

