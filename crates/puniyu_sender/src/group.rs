mod temp;
#[doc(inline)]
pub use temp::*;

use bon::Builder;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use puniyu_core::sender::Sender;

use crate::{Role, Sex};

/// 群聊发送者信息。
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, Builder)]
#[builder(on(SmolStr, into))]
pub struct GroupSender {
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
	/// 群名片
	card: Option<SmolStr>,
	/// 等级
	level: Option<u32>,
	/// 专属头衔
	title: Option<SmolStr>,
}

impl GroupSender {
	/// 获取群角色。
	pub fn role(&self) -> Role {
		self.role
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

impl Sender for GroupSender {
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
        $crate::GroupSender::builder()
        $(
            .$key($value)
        )*
        .build()
    }};
	($user_id:expr) => {{
		$crate::GroupSender::builder().user_id($user_id).build()
	}};
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::Sender;

	#[test]
	fn test_new_basic() {
		let sender = GroupSender::new(
			"123456",
			Some("Alice"),
			Sex::Female,
			Some(25),
			Role::Admin,
			Some("管理员"),
			Some(10),
			Some("活跃成员"),
		);

		assert_eq!(sender.user_id(), "123456");
		assert_eq!(sender.name(), Some("Alice"));
		assert_eq!(sender.sex(), Sex::Female);
		assert_eq!(sender.age(), Some(25));
		assert_eq!(sender.role(), Role::Admin);
		assert_eq!(sender.card(), Some("管理员"));
		assert_eq!(sender.level(), Some(10));
		assert_eq!(sender.title(), Some("活跃成员"));
	}

	#[test]
	fn test_new_none_values() {
		let sender =
			GroupSender::new("u1", None, Sex::Unknown, None, Role::Member, None, None, None);

		assert_eq!(sender.user_id(), "u1");
		assert_eq!(sender.name(), None);
		assert_eq!(sender.sex(), Sex::Unknown);
		assert_eq!(sender.age(), None);
		assert_eq!(sender.role(), Role::Member);
		assert_eq!(sender.card(), None);
		assert_eq!(sender.level(), None);
		assert_eq!(sender.title(), None);
	}

	#[test]
	fn test_default() {
		let sender = GroupSender::default();

		assert_eq!(sender.user_id(), "");
		assert_eq!(sender.name(), None);
		assert_eq!(sender.sex(), Sex::Unknown);
		assert_eq!(sender.age(), None);
		assert_eq!(sender.role(), Role::Unknown);
		assert_eq!(sender.card(), None);
		assert_eq!(sender.level(), None);
		assert_eq!(sender.title(), None);
	}

	#[test]
	fn test_new_owned_string_creation() {
		let sender = GroupSender::new(
			String::from("123456"),
			Some(String::from("Alice")),
			Sex::Female,
			Some(25),
			Role::Admin,
			Some(String::from("管理员")),
			Some(10),
			Some(String::from("活跃成员")),
		);

		assert_eq!(sender.user_id(), "123456");
		assert_eq!(sender.name(), Some("Alice"));
		assert_eq!(sender.card(), Some("管理员"));
		assert_eq!(sender.title(), Some("活跃成员"));
	}

	#[test]
	fn test_new_builder_equivalence() {
		let a = GroupSender::new(
			"123456",
			Some("Alice"),
			Sex::Female,
			Some(25),
			Role::Admin,
			Some("管理员"),
			Some(10),
			Some("活跃成员"),
		);
		let b = GroupSender::builder()
			.user_id("123456")
			.nick("Alice")
			.sex(Sex::Female)
			.age(25)
			.role(Role::Admin)
			.card("管理员")
			.level(10)
			.title("活跃成员")
			.build();

		assert_eq!(a, b);
	}

	#[test]
	fn test_different_roles() {
		let member = GroupSender::builder().user_id("111").role(Role::Member).build();
		let admin = GroupSender::builder().user_id("222").role(Role::Admin).build();
		let owner = GroupSender::builder().user_id("333").role(Role::Owner).build();

		assert_eq!(member.role(), Role::Member);
		assert_eq!(admin.role(), Role::Admin);
		assert_eq!(owner.role(), Role::Owner);
	}

	#[test]
	fn test_serde_roundtrip() {
		let sender = GroupSender::new(
			"123456",
			Some("Alice"),
			Sex::Female,
			Some(25),
			Role::Admin,
			Some("管理员"),
			Some(10),
			Some("活跃成员"),
		);

		let json = serde_json::to_string(&sender).expect("serialize");
		let restored: GroupSender = serde_json::from_str(&json).expect("deserialize");

		assert_eq!(sender, restored);
	}
}
