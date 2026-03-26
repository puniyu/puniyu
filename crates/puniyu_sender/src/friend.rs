//! 好友发送者模块
//!
//! 提供好友发送者的类型定义和构建宏。

use crate::Sender;
use crate::Sex;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// 好友发送者信息。
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

/// 构建好友发送者。
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
