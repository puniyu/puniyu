//! # puniyu_sender
//!
//! 统一的消息发送者类型，覆盖好友和群聊场景。
//!
//! ## 特性
//!
//! - 提供 `FriendSender` 与 `GroupSender`
//! - 提供统一接口 `Sender`
//! - 提供统一枚举 `SenderType`
//! - 提供构建宏 `sender_friend!` 和 `sender_group!`
//!
//! ## 示例
//!
//! ```rust
//! use puniyu_sender::{sender_friend, Sender, SenderType};
//!
//! let friend = sender_friend!(user_id: "123456", nick: "Alice");
//! let sender = SenderType::from(friend);
//! assert_eq!(sender.user_id(), "123456");
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

/// 统一发送者枚举。
#[derive(Debug, Clone, PartialEq, Display, IntoStaticStr, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", tag = "type", content = "field0", bound(deserialize = "'de: 's"))]
pub enum SenderType<'s> {
	/// 好友发送者
	Friend(FriendSender<'s>),
	/// 群聊发送者
	Group(GroupSender<'s>),
}

impl SenderType<'_> {
	/// 获取好友发送者引用；若当前不是好友发送者则返回 `None`。
	pub fn as_friend(&self) -> Option<&FriendSender<'_>> {
		match self {
			SenderType::Friend(sender) => Some(sender),
			_ => None,
		}
	}

	/// 获取群聊发送者引用；若当前不是群聊发送者则返回 `None`。
	pub fn as_group(&self) -> Option<&GroupSender<'_>> {
		match self {
			SenderType::Group(sender) => Some(sender),
			_ => None,
		}
	}

	/// 判断当前是否为好友发送者。
	pub fn is_friend(&self) -> bool {
		matches!(self, SenderType::Friend(_))
	}

	/// 判断当前是否为群聊发送者。
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
