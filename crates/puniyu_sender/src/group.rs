//! 群聊发送者模块
//!
//! 提供群聊发送者的类型定义和构建宏。

use crate::{Role, Sender, Sex};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// 群聊发送者
///
/// 表示群组聊天中的消息发送者信息。
///
/// # 字段
///
/// - `user_id` - 发送者 ID
/// - `nick` - 用户昵称（可选）
/// - `sex` - 性别，类型为 [`Sex`]
/// - `age` - 年龄（可选）
/// - `role` - 群角色，类型为 [`Role`]
/// - `card` - 群名片（可选）
/// - `level` - 等级（可选）
/// - `title` - 专属头衔（可选）
///
/// # 示例
///
/// ```rust
/// use puniyu_sender::{GroupSender, Sex, Role};
///
/// let sender = GroupSender {
///     user_id: "123456",
///     nick: Some("Alice"),
///     sex: Sex::Female,
///     age: Some(25),
///     role: Role::Admin,
///     card: Some("Group Admin"),
///     level: Some(10),
///     title: Some("Active Member"),
/// };
/// ```
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
	/// 获取群角色
	///
	/// # 返回值
	///
	/// 返回发送者在群中的角色 &[`Role`]。
	pub fn role(&self) -> &Role {
		&self.role
	}

	/// 获取群名片
	///
	/// # 返回值
	///
	/// 返回发送者的群名片 [`Option<&str>`]，如果未设置则返回 [`None`]。
	pub fn card(&self) -> Option<&str> {
		self.card
	}

	/// 获取等级
	///
	/// # 返回值
	///
	/// 返回发送者在群中的等级 [`Option<u32>`]，如果未设置则返回 [`None`]。
	pub fn level(&self) -> Option<u32> {
		self.level
	}

	/// 获取专属头衔
	///
	/// # 返回值
	///
	/// 返回发送者的专属头衔 [`Option<&str>`]，如果未设置则返回 [`None`]。
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

/// 构建群聊发送者宏
///
/// 提供便捷的方式创建群聊发送者。
///
/// # 用法
///
/// ## 使用命名字段
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
///
/// ## 仅指定必需字段
///
/// ```rust
/// use puniyu_sender::sender_group;
///
/// let sender = sender_group!(user_id: "123456");
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
