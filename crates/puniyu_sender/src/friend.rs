//! 好友发送者模块
//!
//! 提供好友发送者的类型定义和构建宏。

use derive_builder::Builder;
use crate::Sex;
use serde::{Deserialize, Serialize};
use crate::Sender;

/// 好友发送者
///
/// 表示一对一聊天中的消息发送者信息。
///
/// # 字段
///
/// - `user_id` - 发送者 ID
/// - `nick` - 用户昵称（可选）
/// - `sex` - 性别，类型为 [`Sex`]
/// - `age` - 年龄（可选）
///
/// # 示例
///
/// ```rust
/// use puniyu_sender::{FriendSender, Sex};
///
/// let sender = FriendSender {
///     user_id: "123456",
///     nick: Some("Alice"),
///     sex: Sex::Female,
///     age: Some(25),
/// };
/// ```
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize, Builder)]
#[builder(setter(into), pattern = "owned")]
pub struct FriendSender<'s> {
	/// 发送者id
	pub user_id: &'s str,
	/// 用户昵称
	#[builder(default)]
	pub nick: Option<&'s str>,
	/// 性别
	#[builder(default)]
	pub sex: Sex,
	/// 年龄
	#[builder(default)]
	pub age: Option<u32>,
}

impl<'s> Sender for FriendSender<'s> {
	fn user_id(&self) -> &str {
		self.user_id
	}
	fn name(&self) -> Option<&str> {
		self.nick
	}
	fn sex(&self) -> &Sex {
		&self.sex
	}
	fn age(&self) -> Option<u32> {
		self.age
	}
}

/// 构建好友发送者宏
///
/// 提供便捷的方式创建好友发送者。
///
/// # 用法
///
/// ## 使用命名字段
///
/// ```rust
/// use puniyu_sender::{sender_friend, Sex};
///
/// let sender = sender_friend!(
///     user_id: "123456",
///     nick: "Alice",
///     sex: Sex::Female,
///     age: 25u32,
/// );
/// ```
///
/// ## 仅指定必需字段
///
/// ```rust
/// use puniyu_sender::sender_friend;
///
/// let sender = sender_friend!(user_id: "123456");
/// ```
#[macro_export]
macro_rules! sender_friend {
    ( $( $key:ident : $value:expr ),+ $(,)? ) => {{
        $crate::FriendSenderBuilder::default()
            $(
                .$key($value)
            )*
            .build()
            .expect("Failed to build FriendSender")
    }};
	($user_id:expr) => {{
		$crate::FriendSenderBuilder::default()
			.user_id($user_id)
			.build()
			.expect("Failed to build FriendSender")
	}};
}
