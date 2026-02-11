//! 群聊联系人模块
//!
//! 提供群聊联系人的类型定义和构建宏。

use crate::{Contact, SceneType};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// 群聊联系人
///
/// 表示一个群聊的联系信息。
///
/// # 字段
///
/// - `scene` - 场景类型，固定为 `SceneType::Group`
/// - `peer` - 群聊 ID
/// - `name` - 群名称（可选）
///
/// # 示例
///
/// ```rust
/// use puniyu_contact::{GroupContact, SceneType};
///
/// let group = GroupContact {
///     scene: SceneType::Group,
///     peer: "789012",
///     name: Some("Dev Team"),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Builder)]
#[builder(setter(into))]
pub struct GroupContact<'c> {
	/// 事件来源
	#[builder(setter(skip), default = "SceneType::Group")]
	pub scene: SceneType,
	/// 群聊id
	pub peer: &'c str,
	/// 群名称
	#[builder(default)]
	pub name: Option<&'c str>,
}

impl<'c> GroupContact<'c> {
	/// 判断当前联系人是否为好友
	///
	/// 对于 `GroupContact`，此方法始终返回 `false`。
	///
	/// # 返回值
	///
	/// 始终返回 `false`，因为 `GroupContact` 的场景固定为 `SceneType::Group`。
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_contact::{GroupContact, SceneType};
	///
	/// let group = GroupContact {
	///     scene: SceneType::Group,
	///     peer: "789012",
	///     name: Some("Dev Team"),
	/// };
	///
	/// assert!(!group.is_friend());
	/// ```
	pub fn is_friend(&self) -> bool {
		matches!(self.scene, SceneType::Friend)
	}

	/// 判断当前联系人是否为群组
	///
	/// 对于 `GroupContact`，此方法始终返回 `true`。
	///
	/// # 返回值
	///
	/// 始终返回 `true`，因为 `GroupContact` 的场景固定为 `SceneType::Group`。
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_contact::{GroupContact, SceneType};
	///
	/// let group = GroupContact {
	///     scene: SceneType::Group,
	///     peer: "789012",
	///     name: Some("Dev Team"),
	/// };
	///
	/// assert!(group.is_group());
	/// ```
	pub fn is_group(&self) -> bool {
		matches!(self.scene, SceneType::Group)
	}
}

impl<'c> Contact for GroupContact<'c> {
	fn scene(&self) -> &SceneType {
		&self.scene
	}

	fn peer(&self) -> &str {
		self.peer
	}

	fn name(&self) -> Option<&str> {
		self.name
	}
}

/// 构建群聊联系人宏
///
/// 提供便捷的方式创建群聊联系人。
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
        let mut builder = $crate::GroupContactBuilder::default();
		$(
			builder.$key($value);
		)*
		builder.build().expect("Failed to build GroupContact")
    }};

    ($peer:expr, $name:expr) => {{
        let mut builder = $crate::GroupContactBuilder::default();
        builder.peer($peer);
        builder.name($name);
        builder.build().expect("Failed to build GroupContact")
    }};

    ($peer:expr) => {{
        let mut builder = $crate::GroupContactBuilder::default();
        builder.peer($peer);
        builder.build().expect("Failed to build GroupContact")
    }};
}
