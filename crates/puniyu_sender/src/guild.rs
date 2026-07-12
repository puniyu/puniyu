use bon::Builder;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use puniyu_core::sender::Sender;

use crate::{Role, Sex};

/// 频道发送者信息。
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, Builder)]
#[builder(on(SmolStr, into))]
pub struct GuildSender {
	/// 发送者id
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
	/// 频道名片
	card: Option<SmolStr>,
	/// 等级
	level: Option<u32>,
	/// 专属头衔
	title: Option<SmolStr>,
}

impl GuildSender {
	/// 获取频道角色。
	pub fn role(&self) -> &Role {
		&self.role
	}

	/// 获取频道名片。
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

impl Sender for GuildSender {
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
macro_rules! sender_guild {
    ( $( $key:ident : $value:expr ),+ $(,)? ) => {{
        $crate::GuildSender::builder()
        $(
            .$key($value)
        )*
        .build()
    }};
	($user_id:expr) => {{
		$crate::GuildSender::builder().user_id($user_id).build()
	}};
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::Sender;

	#[test]
	fn test_new_basic() {
		let sender = GuildSender::new(
			"123456",
			Some("Alice"),
			Sex::Female,
			Some(25),
			Role::Admin,
			Some("频道管理员"),
			Some(10),
			Some("频道成员"),
		);

		assert_eq!(sender.user_id(), "123456");
		assert_eq!(sender.name(), Some("Alice"));
		assert_eq!(sender.sex(), Sex::Female);
		assert_eq!(sender.age(), Some(25));
		assert_eq!(sender.role(), &Role::Admin);
		assert_eq!(sender.card(), Some("频道管理员"));
		assert_eq!(sender.level(), Some(10));
		assert_eq!(sender.title(), Some("频道成员"));
	}

	#[test]
	fn test_new_none_values() {
		let sender = GuildSender::new(
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
		let sender = GuildSender::default();

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
		let sender = GuildSender::new(
			String::from("123456"),
			Some(String::from("Alice")),
			Sex::Female,
			Some(25),
			Role::Admin,
			Some(String::from("频道管理员")),
			Some(10),
			Some(String::from("频道成员")),
		);

		assert_eq!(sender.user_id(), "123456");
		assert_eq!(sender.name(), Some("Alice"));
		assert_eq!(sender.card(), Some("频道管理员"));
		assert_eq!(sender.title(), Some("频道成员"));
	}

	#[test]
	fn test_new_builder_equivalence() {
		let a = GuildSender::new(
			"123456",
			Some("Alice"),
			Sex::Female,
			Some(25),
			Role::Admin,
			Some("频道管理员"),
			Some(10),
			Some("频道成员"),
		);
		let b = GuildSender::builder()
			.user_id("123456")
			.nick("Alice")
			.sex(Sex::Female)
			.age(25)
			.role(Role::Admin)
			.card("频道管理员")
			.level(10)
			.title("频道成员")
			.build();

		assert_eq!(a, b);
	}

	#[test]
	fn test_serde_roundtrip() {
		let sender = GuildSender::new(
			"123456",
			Some("Alice"),
			Sex::Female,
			Some(25),
			Role::Admin,
			Some("频道管理员"),
			Some(10),
			Some("频道成员"),
		);

		let json = serde_json::to_string(&sender).expect("serialize");
		let restored: GuildSender = serde_json::from_str(&json).expect("deserialize");

		assert_eq!(sender, restored);
	}
}
