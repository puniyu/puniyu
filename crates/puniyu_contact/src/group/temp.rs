//! 群临时联系人模块
//!
//! 提供群临时联系人的类型定义和构建宏。

use bon::Builder;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use puniyu_core::contact::Contact;

use crate::SceneType;

/// 群临时联系人
///
/// 表示一个群临时会话的联系信息。
///
/// # 示例
///
/// ```rust
/// use puniyu_contact::{Contact, GroupTempContact};
///
/// let group_temp = GroupTempContact::builder().peer("789012").name("Temp Team").build();
/// assert_eq!(group_temp.peer(), "789012");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, Builder)]
#[builder(on(SmolStr, into))]
pub struct GroupTempContact {
	/// 群 ID
	peer: SmolStr,
	/// 群名称
	name: Option<SmolStr>,
}

impl GroupTempContact {
	pub fn new<N>(peer: N, name: Option<N>) -> Self
	where
		N: Into<SmolStr>,
	{
		Self { peer: peer.into(), name: name.map(Into::into) }
	}
}

impl Contact for GroupTempContact {
	type Scene = SceneType;
	fn scene(&self) -> Self::Scene {
		SceneType::GroupTemp
	}

	fn peer(&self) -> &str {
		self.peer.as_ref()
	}

	fn name(&self) -> Option<&str> {
		self.name.as_deref()
	}
}

/// 构建群临时联系人。
///
/// # 用法
///
/// ## 仅指定 peer
///
/// ```rust
/// use puniyu_contact::contact_group_temp;
///
/// let group_temp = contact_group_temp!("789012");
/// ```
///
/// ## 指定 peer 和 name
///
/// ```rust
/// use puniyu_contact::contact_group_temp;
///
/// let group_temp = contact_group_temp!("789012", "Temp Team");
/// ```
///
/// ## 使用命名字段
///
/// ```rust
/// use puniyu_contact::contact_group_temp;
///
/// let group_temp = contact_group_temp!(
///     peer: "789012",
///     name: "Temp Team",
/// );
/// ```
#[macro_export]
macro_rules! contact_group_temp {
    ( $( $key:ident : $value:expr ),+ $(,)? ) => {{
        $crate::GroupTempContact::builder()
            $(
                .$key($value)
            )*
            .build()
    }};

    ($peer:expr, $name:expr) => {{
        $crate::GroupTempContact::builder().peer($peer).name($name).build()
    }};

    ($peer:expr) => {{
        $crate::GroupTempContact::builder().peer($peer).build()
    }};
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::Contact;

	#[test]
	fn test_new_basic() {
		let group_temp = GroupTempContact::builder().peer("246810").name("Temp Team").build();

		assert_eq!(group_temp.peer(), "246810");
		assert_eq!(group_temp.name(), Some("Temp Team"));
		assert_eq!(group_temp.scene(), SceneType::GroupTemp);
	}

	#[test]
	fn test_new_none_name() {
		let group_temp = GroupTempContact::builder().peer("gt1").build();

		assert_eq!(group_temp.peer(), "gt1");
		assert_eq!(group_temp.name(), None);
		assert_eq!(group_temp.scene(), SceneType::GroupTemp);
	}

	#[test]
	fn test_owned_string_creation() {
		let group_temp = GroupTempContact::builder()
			.peer(String::from("246810"))
			.name(String::from("Temp Team"))
			.build();

		assert_eq!(group_temp.peer(), "246810");
		assert_eq!(group_temp.name(), Some("Temp Team"));
	}

	#[test]
	fn test_serde_roundtrip() {
		let group_temp = GroupTempContact::builder().peer("246810").name("Temp Team").build();

		let json = serde_json::to_string(&group_temp).expect("serialize");
		let restored: GroupTempContact = serde_json::from_str(&json).expect("deserialize");

		assert_eq!(group_temp, restored);
	}
}
