//! 好友联系人模块
//!
//! 提供好友联系人的类型定义和构建宏。

use bon::Builder;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use puniyu_core::contact::Contact;

use crate::SceneType;

/// 好友联系人
///
/// 表示一个好友的联系信息。
///
/// # 示例
///
/// ```rust
/// use puniyu_contact::{Contact, FriendContact};
///
/// let friend = FriendContact::builder().peer("123456").name("Alice").build();
/// assert_eq!(friend.peer(), "123456");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, Builder)]
#[builder(on(SmolStr, into))]
pub struct FriendContact {
	/// 好友ID
	peer: SmolStr,
	/// 好友名称
	name: Option<SmolStr>,
}

impl FriendContact {
	pub fn new<N>(peer: N, name: Option<N>) -> Self
	where
		N: Into<SmolStr>,
	{
		Self { peer: peer.into(), name: name.map(Into::into) }
	}
}

impl Contact for FriendContact {
	type Scene = SceneType;
	fn scene(&self) -> Self::Scene {
		SceneType::Friend
	}

	fn peer(&self) -> &str {
		self.peer.as_ref()
	}

	fn name(&self) -> Option<&str> {
		self.name.as_deref()
	}
}

/// 构建好友联系人。
///
/// # 用法
///
/// ## 仅指定 peer
///
/// ```rust
/// use puniyu_contact::contact_friend;
///
/// let friend = contact_friend!("123456");
/// ```
///
/// ## 指定 peer 和 name
///
/// ```rust
/// use puniyu_contact::contact_friend;
///
/// let friend = contact_friend!("123456", "Alice");
/// ```
///
/// ## 使用命名字段
///
/// ```rust
/// use puniyu_contact::contact_friend;
///
/// let friend = contact_friend!(
///     peer: "123456",
///     name: "Alice",
/// );
/// ```
#[macro_export]
macro_rules! contact_friend {
    ( $( $key:ident : $value:expr ),+ $(,)? ) => {{
        $crate::FriendContact::builder()
            $(
                .$key($value)
            )*
            .build()
    }};

    ($peer:expr, $name:expr) => {{
        $crate::FriendContact::builder().peer($peer).name($name).build()
    }};

    ($peer:expr) => {{
        $crate::FriendContact::builder().peer($peer).build()
    }};
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::Contact;

	#[test]
	fn test_new_basic() {
		let friend = FriendContact::new("123456", Some("Alice"));

		assert_eq!(friend.peer(), "123456");
		assert_eq!(friend.name(), Some("Alice"));
		assert_eq!(friend.scene(), SceneType::Friend);
	}

	#[test]
	fn test_new_none_name() {
		let friend = FriendContact::new("u1", None::<&str>);

		assert_eq!(friend.peer(), "u1");
		assert_eq!(friend.name(), None);
		assert_eq!(friend.scene(), SceneType::Friend);
	}

	#[test]
	fn test_new_owned_string() {
		let friend = FriendContact::new(String::from("123456"), Some(String::from("Alice")));

		assert_eq!(friend.peer(), "123456");
		assert_eq!(friend.name(), Some("Alice"));
	}

	#[test]
	fn test_new_builder_equivalence() {
		let a = FriendContact::new("123456", Some("Alice"));
		let b = FriendContact::builder().peer("123456").name("Alice").build();

		assert_eq!(a, b);
	}

	#[test]
	fn test_new_owned_string_creation() {
		let friend = FriendContact::builder()
			.peer(String::from("123456"))
			.name(String::from("Alice"))
			.build();

		assert_eq!(friend.peer(), "123456");
		assert_eq!(friend.name(), Some("Alice"));
	}

	#[test]
	fn test_serde_roundtrip() {
		let friend = FriendContact::builder().peer("123456").name("Alice").build();

		let json = serde_json::to_string(&friend).expect("serialize");
		let restored: FriendContact = serde_json::from_str(&json).expect("deserialize");

		assert_eq!(friend, restored);
	}
}
