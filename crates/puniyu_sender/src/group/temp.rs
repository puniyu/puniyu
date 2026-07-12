use bon::Builder;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use puniyu_core::sender::Sender;

use crate::{Role, Sex};

/// 群临时发送者信息。
///
/// 表示群临时会话中的发送者资料。
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, Builder)]
#[builder(on(SmolStr, into))]
pub struct GroupTempSender {
	/// 发送者 ID
	user_id: SmolStr,
	/// 用户昵称
	nick: Option<SmolStr>,
	/// 性别
	#[builder(default)]
	sex: Sex,
	/// 年龄
	age: Option<u32>,
	/// 角色
	#[builder(default)]
	role: Role,
	/// 群名片
	card: Option<SmolStr>,
	/// 等级
	level: Option<u32>,
	/// 专属头衔
	title: Option<SmolStr>,
}

impl GroupTempSender {
	pub fn role(&self) -> &Role {
		&self.role
	}

	pub fn card(&self) -> Option<&str> {
		self.card.as_deref()
	}

	pub fn level(&self) -> Option<u32> {
		self.level
	}

	pub fn title(&self) -> Option<&str> {
		self.title.as_deref()
	}

	#[allow(clippy::too_many_arguments)]
	pub fn new<N>(
		user_id: N,
		nick: Option<N>,
		sex: Sex,
		age: Option<u32>,
		role: Role,
		card: Option<N>,
		level: Option<u32>,
		title: Option<N>,
	) -> Self
	where
		N: Into<SmolStr>,
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
}

impl Sender for GroupTempSender {
	type Sex = Sex;
	fn user_id(&self) -> &str {
		self.user_id.as_ref()
	}
	fn name(&self) -> Option<&str> {
		self.nick.as_deref()
	}
	fn sex(&self) -> Sex {
		self.sex
	}
	fn age(&self) -> Option<u32> {
		self.age
	}
}

#[macro_export]
macro_rules! sender_group_temp {
    ( $( $key:ident : $value:expr ),+ $(,)? ) => {{
        $crate::GroupTempSender::builder()
        $(
            .$key($value)
        )*
        .build()
    }};
	($user_id:expr) => {{
		$crate::GroupTempSender::builder().user_id($user_id).build()
	}};
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::Sender;

	#[test]
	fn test_new_basic() {
		let sender = GroupTempSender::new(
			"123456",
			Some("Carol"),
			Sex::Female,
			Some(22),
			Role::Member,
			Some("临时群员"),
			Some(5),
			Some("新人"),
		);

		assert_eq!(sender.user_id(), "123456");
		assert_eq!(sender.name(), Some("Carol"));
		assert_eq!(sender.sex(), Sex::Female);
		assert_eq!(sender.age(), Some(22));
		assert_eq!(sender.role(), &Role::Member);
		assert_eq!(sender.card(), Some("临时群员"));
		assert_eq!(sender.level(), Some(5));
		assert_eq!(sender.title(), Some("新人"));
	}

	#[test]
	fn test_new_none_values() {
		let sender = GroupTempSender::new(
			"u1",
			None::<&str>,
			Sex::Unknown,
			None,
			Role::Member,
			None::<&str>,
			None,
			None::<&str>,
		);

		assert_eq!(sender.user_id(), "u1");
		assert_eq!(sender.name(), None);
		assert_eq!(sender.sex(), Sex::Unknown);
		assert_eq!(sender.age(), None);
		assert_eq!(sender.role(), &Role::Member);
		assert_eq!(sender.card(), None);
		assert_eq!(sender.level(), None);
		assert_eq!(sender.title(), None);
	}

	#[test]
	fn test_default() {
		let sender = GroupTempSender::default();

		assert_eq!(sender.user_id(), "");
		assert_eq!(sender.name(), None);
		assert_eq!(sender.sex(), Sex::Unknown);
		assert_eq!(sender.age(), None);
		assert_eq!(sender.role(), &Role::Unknown);
		assert_eq!(sender.card(), None);
		assert_eq!(sender.level(), None);
		assert_eq!(sender.title(), None);
	}

	#[test]
	fn test_new_owned_string_creation() {
		let sender = GroupTempSender::new(
			String::from("123456"),
			Some(String::from("Carol")),
			Sex::Female,
			Some(22),
			Role::Admin,
			Some(String::from("临时管理员")),
			Some(7),
			Some(String::from("活跃成员")),
		);

		assert_eq!(sender.user_id(), "123456");
		assert_eq!(sender.name(), Some("Carol"));
		assert_eq!(sender.card(), Some("临时管理员"));
		assert_eq!(sender.title(), Some("活跃成员"));
	}

	#[test]
	fn test_new_builder_equivalence() {
		let a = GroupTempSender::new(
			"123456",
			Some("Carol"),
			Sex::Female,
			Some(22),
			Role::Member,
			Some("临时群员"),
			Some(5),
			Some("新人"),
		);
		let b = GroupTempSender::builder()
			.user_id("123456")
			.nick("Carol")
			.sex(Sex::Female)
			.age(22)
			.role(Role::Member)
			.card("临时群员")
			.level(5)
			.title("新人")
			.build();

		assert_eq!(a, b);
	}

	#[test]
	fn test_serde_roundtrip() {
		let sender = GroupTempSender::new(
			"123456",
			Some("Carol"),
			Sex::Female,
			Some(22),
			Role::Member,
			Some("临时群员"),
			Some(5),
			Some("新人"),
		);

		let json = serde_json::to_string(&sender).expect("serialize");
		let restored: GroupTempSender = serde_json::from_str(&json).expect("deserialize");

		assert_eq!(sender, restored);
	}
}
