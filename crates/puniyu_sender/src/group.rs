//! 群聊发送者模块
//!
//! 提供群聊发送者的类型定义和构建宏。

use crate::{Role, Sender, Sex};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// 群聊发送者信息。
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize, Builder)]
#[builder(setter(into), pattern = "owned")]
pub struct GroupSender<'s> {
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
	/// 角色
	#[builder(default)]
	pub role: Role,
	/// 群名片
	#[builder(default)]
	pub card: Option<&'s str>,
	/// 等级
	#[builder(default)]
	pub level: Option<u32>,
	/// 专属头衔
	#[builder(default)]
	pub title: Option<&'s str>,
}

impl<'s> GroupSender<'s> {
	/// 获取群角色。
	pub fn role(&self) -> &Role {
		&self.role
	}

	/// 获取群名片。
	pub fn card(&self) -> Option<&str> {
		self.card
	}

	/// 获取等级。
	pub fn level(&self) -> Option<u32> {
		self.level
	}

	/// 获取专属头衔。
	pub fn title(&self) -> Option<&str> {
		self.title
	}
}

impl<'s> Sender for GroupSender<'s> {
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
