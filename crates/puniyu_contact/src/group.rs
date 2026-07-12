//! 群聊联系人模块
//!
//! 提供群聊联系人与群临时联系人的类型定义和构建宏。

mod temp;
pub use temp::*;

use bon::Builder;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use puniyu_core::contact::Contact;

use crate::SceneType;

/// 群聊联系人
///
/// 表示一个群聊的联系信息。
///
/// # 示例
///
/// ```rust
/// use puniyu_contact::{Contact, GroupContact};
///
/// let group = GroupContact::builder().peer("789012").name("Dev Team").build();
/// assert_eq!(group.peer(), "789012");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, Builder)]
#[builder(on(SmolStr, into))]
pub struct GroupContact {
	/// 群聊id
	peer: SmolStr,
	/// 群名称
	name: Option<SmolStr>,
}

impl GroupContact {
	pub fn new<N>(peer: N, name: Option<N>) -> Self
	where
		N: Into<SmolStr>,
	{
		Self { peer: peer.into(), name: name.map(Into::into) }
	}
}

impl Contact for GroupContact {
	type Scene = SceneType;
	fn scene(&self) -> Self::Scene {
		SceneType::Group
	}

	fn peer(&self) -> &str {
		self.peer.as_ref()
	}

	fn name(&self) -> Option<&str> {
		self.name.as_deref()
	}
}

/// 构建群聊联系人。
///
/// # 用法
///
/// ## 仅指定 peer
///
/// ```rust
/// use puniyu_contact::contact_group;
///
/// let group = contact_group!("789012");
/// ```
///
/// ## 指定 peer 和 name
///
/// ```rust
/// use puniyu_contact::contact_group;
///
/// let group = contact_group!("789012", "Dev Team");
/// ```
///
/// ## 使用命名字段
///
/// ```rust
/// use puniyu_contact::contact_group;
///
/// let group = contact_group!(
///     peer: "789012",
///     name: "Dev Team",
/// );
/// ```
#[macro_export]
macro_rules! contact_group {
    ( $( $key:ident : $value:expr ),+ $(,)? ) => {{
        $crate::GroupContact::builder()
            $(
                .$key($value)
            )*
            .build()
    }};

    ($peer:expr, $name:expr) => {{
        $crate::GroupContact::builder().peer($peer).name($name).build()
    }};

    ($peer:expr) => {{
        $crate::GroupContact::builder().peer($peer).build()
    }};
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::Contact;

	#[test]
	fn test_new_basic() {
		let group = GroupContact::new("789012", Some("Dev Team"));

		assert_eq!(group.peer(), "789012");
		assert_eq!(group.name(), Some("Dev Team"));
		assert_eq!(group.scene(), SceneType::Group);
	}

	#[test]
	fn test_new_none_name() {
		let group = GroupContact::new("g1", None::<&str>);

		assert_eq!(group.peer(), "g1");
		assert_eq!(group.name(), None);
		assert_eq!(group.scene(), SceneType::Group);
	}

	#[test]
	fn test_new_owned_string_creation() {
		let group = GroupContact::new(String::from("789012"), Some(String::from("Dev Team")));

		assert_eq!(group.peer(), "789012");
		assert_eq!(group.name(), Some("Dev Team"));
	}

	#[test]
	fn test_new_builder_equivalence() {
		let a = GroupContact::new("789012", Some("Dev Team"));
		let b = GroupContact::builder().peer("789012").name("Dev Team").build();

		assert_eq!(a, b);
	}

	#[test]
	fn test_serde_roundtrip() {
		let group = GroupContact::builder().peer("789012").name("Dev Team").build();

		let json = serde_json::to_string(&group).expect("serialize");
		let restored: GroupContact = serde_json::from_str(&json).expect("deserialize");

		assert_eq!(group, restored);
	}
}
