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
/// - `peer` - 好友 ID
/// - `name` - 好友名称（可选）
///
/// # 示例
///
/// ```rust
/// use puniyu_contact::FriendContact;
///
/// let friend = FriendContact {
///     peer: "123456",
///     name: Some("Alice"),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Builder)]
#[serde(bound(deserialize = "'de: 'c"))]
#[builder(setter(into), pattern = "owned")]
pub struct FriendContact<'c> {
	/// 好友ID
	pub peer: &'c str,
	/// 好友名称
	#[builder(default)]
	pub name: Option<&'c str>,
}

impl<'c> Contact for FriendContact<'c> {
	fn scene(&self) -> &SceneType {
		&SceneType::Friend
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
        $crate::FriendContactBuilder::default()
            $(
                .$key($value)
            )*
            .build()
            .expect("Failed to build FriendContact")
    }};

    ($peer:expr, $name:expr) => {{
        $crate::FriendContactBuilder::default()
            .peer($peer)
            .name($name)
            .build()
            .expect("Failed to build FriendContact")
    }};

    ($peer:expr) => {{
        $crate::FriendContactBuilder::default()
            .peer($peer)
            .build()
            .expect("Failed to build FriendContact")
    }};
}
