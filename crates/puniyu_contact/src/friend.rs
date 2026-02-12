//! 好友联系人模块
//!
//! 提供好友联系人的类型定义和构建宏。

use crate::{Contact, SceneType};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// 好友联系人
///
/// 表示一个好友的联系信息。
///
/// # 字段
///
/// - `scene` - 场景类型，固定为 `SceneType::Friend`
/// - `peer` - 好友 ID
/// - `name` - 好友名称（可选）
///
/// # 示例
///
/// ```rust
/// use puniyu_contact::{FriendContact, SceneType};
///
/// let friend = FriendContact {
///     scene: SceneType::Friend,
///     peer: "123456",
///     name: Some("Alice"),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Builder)]
#[serde(bound(deserialize = "'de: 'c"))]
#[builder(setter(into))]
pub struct FriendContact<'c> {
	/// 事件来源
	#[builder(setter(skip), default = "SceneType::Friend")]
	pub scene: SceneType,
	/// 好友ID
	pub peer: &'c str,
	/// 好友名称
	#[builder(default)]
	pub name: Option<&'c str>,
}

impl<'c> FriendContact<'c> {
	/// 判断当前联系人是否为好友
	///
	/// 对于 `FriendContact`，此方法始终返回 `true`。
	///
	/// # 返回值
	///
	/// 始终返回 `true`，因为 `FriendContact` 的场景固定为 `SceneType::Friend`。
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_contact::{FriendContact, SceneType};
	///
	/// let friend = FriendContact {
	///     scene: SceneType::Friend,
	///     peer: "123456",
	///     name: Some("Alice"),
	/// };
	///
	/// assert!(friend.is_friend());
	/// ```
	pub fn is_friend(&self) -> bool {
		matches!(self.scene, SceneType::Friend)
	}

	/// 判断当前联系人是否为群组
	///
	/// 对于 `FriendContact`，此方法始终返回 `false`。
	///
	/// # 返回值
	///
	/// 始终返回 `false`，因为 `FriendContact` 的场景固定为 `SceneType::Friend`。
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_contact::{FriendContact, SceneType};
	///
	/// let friend = FriendContact {
	///     scene: SceneType::Friend,
	///     peer: "123456",
	///     name: Some("Alice"),
	/// };
	///
	/// assert!(!friend.is_group());
	/// ```
	pub fn is_group(&self) -> bool {
		matches!(self.scene, SceneType::Group)
	}
}

impl<'c> Contact for FriendContact<'c> {
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

/// 构建好友联系人宏
///
/// 提供便捷的方式创建好友联系人。
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
        let mut builder = $crate::FriendContactBuilder::default();
		$(
			builder.$key($value);
		)*
		builder.build().expect("Failed to build FriendContact")
    }};

    ($peer:expr, $name:expr) => {{
        let mut builder = $crate::FriendContactBuilder::default();
        builder.peer($peer);
        builder.name($name);
        builder.build().expect("Failed to build FriendContact")
    }};

    ($peer:expr) => {{
        let mut builder = $crate::FriendContactBuilder::default();
        builder.peer($peer);
        builder.build().expect("Failed to build FriendContact")
    }};
}
