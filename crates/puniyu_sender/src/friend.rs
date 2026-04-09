//! 好友发送者模块
//!
//! 提供好友发送者的类型定义和构建宏。

use crate::Sender;
use crate::Sex;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// 好友发送者信息。
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize, Builder)]
#[serde(bound(deserialize = "'de: 's"))]
#[builder(setter(into), pattern = "owned")]
pub struct FriendSender<'s> {
	/// 发送者id
	#[serde(borrow)]
	user_id: Cow<'s, str>,
	/// 用户昵称
	#[builder(default, setter(into, strip_option))]
	#[serde(borrow)]
	nick: Option<Cow<'s, str>>,
	/// 性别
	#[builder(default)]
	sex: Sex,
	/// 年龄
	#[builder(default)]
	age: Option<u32>,
}

impl<'s> FriendSender<'s> {
	pub fn new<U, N>(user_id: U, nick: Option<N>, sex: Sex, age: Option<u32>) -> Self
	where
		U: Into<Cow<'s, str>>,
		N: Into<Cow<'s, str>>,
	{
		Self { user_id: user_id.into(), nick: nick.map(Into::into), sex, age }
	}

	pub fn builder() -> FriendSenderBuilder<'s> {
		FriendSenderBuilder::default()
	}
}

impl<'s> Sender for FriendSender<'s> {
	fn user_id(&self) -> &str {
		self.user_id.as_ref()
	}
	fn name(&self) -> Option<&str> {
		self.nick.as_deref()
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
