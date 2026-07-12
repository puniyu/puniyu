//! # puniyu_sender
//!
//! 统一的消息发送者类型，覆盖好友、群聊、群临时和频道场景。
//!
//! ## 特性
//!
//! - 提供 `FriendSender`、`GroupSender`、`GroupTempSender` 与 `GuildSender`
//! - 提供统一接口 `Sender`
//! - 提供统一枚举 `SenderType`
//! - 提供构建宏 `sender!`、`sender_friend!`、`sender_group!`、`sender_group_temp!` 和 `sender_guild!`
//!
//! ## 示例
//!
//! ```rust
//! use puniyu_sender::{sender, Sender};
//!
//! let friend = sender!(Friend, user_id: "123456", nick: "Alice");
//! let group = sender!(Group, user_id: "789012", nick: "Bob");
//! let group_temp = sender!(GroupTemp, user_id: "789012", nick: "Bob");
//! let guild = sender!(Guild, user_id: "9527", nick: "Carol", card: "频道管理员");
//!
//! assert!(friend.is_friend());
//! assert_eq!(friend.user_id(), "123456");
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
use strum::{Display, IntoStaticStr};

pub use puniyu_core::sender::Sender;

/// 统一发送者枚举
///
/// 统一的发送者类型，可以是好友发送者或群聊发送者。
///
/// # 示例
///
/// ```rust
/// use puniyu_sender::{FriendSender, GroupSender, GroupTempSender, Role, SenderType, Sex};
///
/// // 创建好友发送者
/// let friend = FriendSender::builder()
///     .user_id("123456")
///     .nick("Alice")
///     .sex(Sex::Female)
///     .age(25)
///     .build();
/// let sender = SenderType::Friend(friend);
///
/// // 创建群聊发送者
/// let group = GroupSender::builder()
///     .user_id("789012")
///     .nick("Bob")
///     .sex(Sex::Male)
///     .age(30)
///     .role(Role::Admin)
///     .card("管理员")
///     .level(10)
///     .title("活跃成员")
///     .build();
/// let sender = SenderType::Group(group);
///
/// // 创建群临时发送者（仅必填，其余使用 None）
/// let group_temp = GroupTempSender::builder()
///     .user_id("246810")
///     .nick("Carol")
///     .sex(Sex::Female)
///     .age(22)
///     .role(Role::Member)
///     .build();
/// let sender = SenderType::GroupTemp(group_temp);
/// ```
#[derive(Debug, Clone, PartialEq, Display, IntoStaticStr, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", tag = "type", content = "field0")]
pub enum SenderType {
	/// 好友发送者
	Friend(FriendSender),
	/// 群聊发送者
	Group(GroupSender),
	/// 群临时发送者
	GroupTemp(GroupTempSender),
	/// 频道发送者
	Guild(GuildSender),
}

impl SenderType {
	/// 获取好友发送者引用
	///
	/// 如果当前是好友发送者则返回 [`Some`]，否则返回 [`None`]。
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_sender::{sender_friend, Sender, SenderType};
	///
	/// let sender = SenderType::from(sender_friend!(user_id: "123456", nick: "Alice"));
	///
	/// if let Some(friend) = sender.as_friend() {
	///     assert_eq!(friend.user_id(), "123456");
	/// }
	/// ```
	pub fn as_friend(&self) -> Option<&FriendSender> {
		match self {
			SenderType::Friend(sender) => Some(sender),
			_ => None,
		}
	}

	/// 获取群聊发送者引用
	///
	/// 如果当前是群聊发送者则返回 [`Some`]，否则返回 [`None`]。
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_sender::{sender_group, Role, SenderType};
	///
	/// let sender = SenderType::from(sender_group!(user_id: "789012", role: Role::Admin));
	///
	/// if let Some(group) = sender.as_group() {
	///     assert_eq!(group.role(), Role::Admin);
	/// }
	/// ```
	pub fn as_group(&self) -> Option<&GroupSender> {
		match self {
			SenderType::Group(sender) => Some(sender),
			_ => None,
		}
	}

	/// 获取群临时发送者引用
	pub fn as_group_temp(&self) -> Option<&GroupTempSender> {
		match self {
			SenderType::GroupTemp(sender) => Some(sender),
			_ => None,
		}
	}

	/// 获取频道发送者引用
	pub fn as_guild(&self) -> Option<&GuildSender> {
		match self {
			SenderType::Guild(sender) => Some(sender),
			_ => None,
		}
	}

	/// 判断是否为好友发送者
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_sender::{sender_friend, SenderType};
	///
	/// let sender = SenderType::from(sender_friend!(user_id: "123456"));
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
	/// use puniyu_sender::{sender_group, SenderType};
	///
	/// let sender = SenderType::from(sender_group!(user_id: "789012"));
	/// assert!(sender.is_group());
	/// ```
	pub fn is_group(&self) -> bool {
		matches!(self, SenderType::Group(_))
	}

	/// 判断是否为群临时发送者
	pub fn is_group_temp(&self) -> bool {
		matches!(self, SenderType::GroupTemp(_))
	}

	/// 判断是否为频道发送者
	pub fn is_guild(&self) -> bool {
		matches!(self, SenderType::Guild(_))
	}
}

impl Sender for SenderType {
	type Sex = Sex;
	fn user_id(&self) -> &str {
		match self {
			Self::Friend(sender) => sender.user_id(),
			Self::Group(sender) => sender.user_id(),
			Self::GroupTemp(sender) => sender.user_id(),
			Self::Guild(sender) => sender.user_id(),
		}
	}
	fn name(&self) -> Option<&str> {
		match self {
			Self::Friend(sender) => sender.name(),
			Self::Group(sender) => sender.name(),
			Self::GroupTemp(sender) => sender.name(),
			Self::Guild(sender) => sender.name(),
		}
	}
	fn sex(&self) -> Sex {
		match &self {
			Self::Friend(sender) => sender.sex(),
			Self::Group(sender) => sender.sex(),
			Self::GroupTemp(sender) => sender.sex(),
			Self::Guild(sender) => sender.sex(),
		}
	}
	fn age(&self) -> Option<u32> {
		match self {
			Self::Friend(sender) => sender.age(),
			Self::Group(sender) => sender.age(),
			Self::GroupTemp(sender) => sender.age(),
			Self::Guild(sender) => sender.age(),
		}
	}
}

impl From<FriendSender> for SenderType {
	#[inline]
	fn from(sender: FriendSender) -> Self {
		Self::Friend(sender)
	}
}

impl From<GroupSender> for SenderType {
	#[inline]
	fn from(sender: GroupSender) -> Self {
		Self::Group(sender)
	}
}

impl From<GroupTempSender> for SenderType {
	#[inline]
	fn from(sender: GroupTempSender) -> Self {
		Self::GroupTemp(sender)
	}
}

impl From<GuildSender> for SenderType {
	#[inline]
	fn from(sender: GuildSender) -> Self {
		Self::Guild(sender)
	}
}

/// 统一的发送者构建宏
///
/// 根据发送者类型（Friend 或 Group）创建相应的发送者对象，
/// 并返回统一类型 [`SenderType`]。
///
/// # 语法
///
/// ```text
/// sender!(Friend, field: value, ...)
/// sender!(Group, field: value, ...)
/// ```
///
/// # 参数
///
/// - 第一个参数：发送者类型，必须是 `Friend` 或 `Group`
/// - 后续参数：字段名和值的键值对
///   - `user_id`: 发送者 ID（必需）
///   - `nick`: 发送者昵称（可选）
///   - `sex`: 性别（可选）
///   - `age`: 年龄（可选）
/// - 当类型为 `Group` 时，还支持：
///   - `role`: 群角色（可选）
///   - `card`: 群名片（可选）
///   - `level`: 等级（可选）
///   - `title`: 专属头衔（可选）
///
/// # 示例
///
/// ## 创建好友发送者
///
/// ```rust
/// use puniyu_sender::sender;
///
/// let sender = sender!(Friend, user_id: "123456", nick: "Alice");
/// assert!(sender.is_friend());
/// ```
///
/// ## 创建群聊发送者
///
/// ```rust
/// use puniyu_sender::{sender, Role};
///
/// let sender = sender!(Group, user_id: "789012", role: Role::Admin, card: "管理员");
/// assert!(sender.is_group());
/// ```
///
/// ## 与专用宏的对比
///
/// ```rust
/// use puniyu_sender::{sender, sender_friend, sender_group, Role, SenderType};
///
/// // 使用统一宏
/// let friend = sender!(Friend, user_id: "123456", nick: "Alice");
/// let group = sender!(Group, user_id: "789012", role: Role::Admin);
/// assert!(matches!(friend, SenderType::Friend(_)));
/// assert!(matches!(group, SenderType::Group(_)));
///
/// // 使用专用宏（返回具体发送者类型）
/// let friend = sender_friend!(user_id: "123456", nick: "Alice");
/// let group = sender_group!(user_id: "789012", role: Role::Admin);
/// ```
///
/// # 注意
///
/// - 此宏只支持命名字段语法，不支持位置参数
/// - 如果需要使用位置参数（如 `sender_friend!("123456")`），请直接使用专用宏
#[macro_export]
macro_rules! sender {
    (Friend, $( $key:ident : $value:expr ),+ $(,)?) => {
        $crate::SenderType::Friend($crate::sender_friend!( $( $key : $value ),+ ))
    };

    (Group, $( $key:ident : $value:expr ),+ $(,)?) => {
        $crate::SenderType::Group($crate::sender_group!( $( $key : $value ),+ ))
    };

    (GroupTemp, $( $key:ident : $value:expr ),+ $(,)?) => {
        $crate::SenderType::GroupTemp($crate::sender_group_temp!( $( $key : $value ),+ ))
    };

    (Guild, $( $key:ident : $value:expr ),+ $(,)?) => {
        $crate::SenderType::Guild($crate::sender_guild!( $( $key : $value ),+ ))
    };
}
