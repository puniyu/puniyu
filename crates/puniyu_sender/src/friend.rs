use bon::Builder;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use puniyu_core::sender::Sender;

use crate::Sex;

/// 好友发送者信息
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, Builder)]
#[builder(on(SmolStr, into))]
pub struct FriendSender {
	/// 发送者id
	user_id: SmolStr,
	/// 用户昵称
	nick: Option<SmolStr>,
	/// 性别
	#[builder(default)]
	sex: Sex,
	/// 年龄
	age: Option<u32>,
}

impl FriendSender {
	pub fn new<N>(user_id: N, nick: Option<N>, sex: Sex, age: Option<u32>) -> Self
	where
		N: Into<SmolStr>,
	{
		Self { user_id: user_id.into(), nick: nick.map(Into::into), sex, age }
	}
}

impl Sender for FriendSender {
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
        $crate::FriendSender::builder()
            $(
                .$key($value)
            )*
            .build()
    }};
	($user_id:expr) => {{
		$crate::FriendSender::builder().user_id($user_id).build()
	}};
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::Sender;

	#[test]
	fn test_new_basic() {
		let sender = FriendSender::new("123456", Some("Alice"), Sex::Female, Some(25));

		assert_eq!(sender.user_id(), "123456");
		assert_eq!(sender.name(), Some("Alice"));
		assert_eq!(sender.sex(), Sex::Female);
		assert_eq!(sender.age(), Some(25));
	}

	#[test]
	fn test_new_none_values() {
		let sender = FriendSender::new("u1", None, Sex::Unknown, None);

		assert_eq!(sender.user_id(), "u1");
		assert_eq!(sender.name(), None);
		assert_eq!(sender.sex(), Sex::Unknown);
		assert_eq!(sender.age(), None);
	}

	#[test]
	fn test_default() {
		let sender = FriendSender::default();

		assert_eq!(sender.user_id(), "");
		assert_eq!(sender.name(), None);
		assert_eq!(sender.sex(), Sex::Unknown);
		assert_eq!(sender.age(), None);
	}

	#[test]
	fn test_new_owned_string_creation() {
		let sender = FriendSender::new(
			String::from("123456"),
			Some(String::from("Alice")),
			Sex::Female,
			Some(25),
		);

		assert_eq!(sender.user_id(), "123456");
		assert_eq!(sender.name(), Some("Alice"));
		assert_eq!(sender.sex(), Sex::Female);
		assert_eq!(sender.age(), Some(25));
	}

	#[test]
	fn test_new_builder_equivalence() {
		let a = FriendSender::new("123456", Some("Alice"), Sex::Female, Some(25));
		let b = FriendSender::builder()
			.user_id("123456")
			.nick("Alice")
			.sex(Sex::Female)
			.age(25)
			.build();

		assert_eq!(a, b);
	}

	#[test]
	fn test_serde_roundtrip() {
		let sender = FriendSender::new("123456", Some("Alice"), Sex::Female, Some(25));

		let json = serde_json::to_string(&sender).expect("serialize");
		let restored: FriendSender = serde_json::from_str(&json).expect("deserialize");

		assert_eq!(sender, restored);
	}
}
