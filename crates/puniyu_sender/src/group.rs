//! 群聊发送者模块
//!
//! 提供群聊发送者的类型定义和构建宏。
mod temp;
#[doc(inline)]
pub use temp::*;

use crate::{Role, Sender, Sex};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// 群聊发送者信息。
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize, Builder)]
#[serde(bound(deserialize = "'de: 's"))]
#[builder(setter(into), pattern = "owned")]
pub struct GroupSender<'s> {
	/// 发送者id
	#[serde(borrow)]
	user_id: Cow<'s, str>,
	/// 用户昵称
	#[builder(default, setter(strip_option))]
	#[serde(borrow)]
	nick: Option<Cow<'s, str>>,
	/// 性别
	#[builder(default)]
	sex: Sex,
	/// 年龄
	#[builder(default)]
	age: Option<u32>,
	/// 角色
	#[builder(default)]
	role: Role,
	/// 群名片
	#[builder(default, setter(strip_option))]
	#[serde(borrow)]
	card: Option<Cow<'s, str>>,
	/// 等级
	#[builder(default)]
	level: Option<u32>,
	/// 专属头衔
	#[builder(default, setter(strip_option))]
	#[serde(borrow)]
	title: Option<Cow<'s, str>>,
}

impl<'s> GroupSender<'s> {
	#[allow(clippy::too_many_arguments)]
	pub fn new<U, N, C, T>(
		user_id: U,
		nick: Option<N>,
		sex: Sex,
		age: Option<u32>,
		role: Role,
		card: Option<C>,
		level: Option<u32>,
		title: Option<T>,
	) -> Self
	where
		U: Into<Cow<'s, str>>,
		N: Into<Cow<'s, str>>,
		C: Into<Cow<'s, str>>,
		T: Into<Cow<'s, str>>,
	{
		Self {
			user_id: user_id.into(),
			nick: nick.map(Into::into),
			sex,
			age,
			role,
			card: card.map(Into::into),
			level,
			title: title.map(Into::into),
		}
	}

	pub fn builder() -> GroupSenderBuilder<'s> {
		GroupSenderBuilder::default()
	}

	/// 获取群角色。
	pub fn role(&self) -> &Role {
		&self.role
	}

	/// 获取群名片。
	pub fn card(&self) -> Option<&str> {
		self.card.as_deref()
	}

	/// 获取等级。
	pub fn level(&self) -> Option<u32> {
		self.level
	}

	/// 获取专属头衔。
	pub fn title(&self) -> Option<&str> {
		self.title.as_deref()
	}
}

impl<'s> Sender for GroupSender<'s> {
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

/// 构建群聊发送者。
///
/// ```rust
/// use puniyu_sender::{sender_group, Sex, Role};
///
/// let sender = sender_group!(
///     user_id: "123456",
///     nick: "Alice",
///     sex: Sex::Female,
///     age: 25u32,
///     role: Role::Admin,
///     card: "Group Admin",
/// );
/// ```
#[macro_export]
macro_rules! sender_group {
    ( $( $key:ident : $value:expr ),+ $(,)? ) => {{
        $crate::GroupSenderBuilder::default()
        $(
            .$key($value)
        )*
        .build()
		.expect("Failed to build GroupSender")
    }};
	($user_id:expr) => {{
		$crate::GroupSenderBuilder::default()
			.user_id($user_id)
			.build()
			.expect("Failed to build GroupSender")
	}};
}
