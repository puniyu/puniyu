//! 群聊发送者模块
//!
//! 提供群聊发送者的类型定义和构建宏。

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
	pub user_id: Cow<'s, str>,
	/// 用户昵称
	#[builder(default, setter(into, strip_option))]
	#[serde(borrow)]
	pub nick: Option<Cow<'s, str>>,
	/// 性别
	#[builder(default)]
	pub sex: Sex,
	/// 年龄
	#[builder(default)]
	pub age: Option<u32>,
	/// 角色
	#[builder(default)]
	pub role: Role,
	/// 群名片
	#[builder(default, setter(into, strip_option))]
	#[serde(borrow)]
	pub card: Option<Cow<'s, str>>,
	/// 等级
	#[builder(default)]
	pub level: Option<u32>,
	/// 专属头衔
	#[builder(default, setter(into, strip_option))]
	#[serde(borrow)]
	pub title: Option<Cow<'s, str>>,
}

impl<'s> GroupSender<'s> {
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
