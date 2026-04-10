//! 好友联系人模块
//!
//! 提供好友联系人的类型定义和构建宏。

use crate::{Contact, SceneType};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// 好友联系人
///
/// 表示一个好友的联系信息。
///
/// # 示例
///
/// ```rust
/// use puniyu_contact::{Contact, FriendContact};
///
/// let friend = FriendContact::new("123456", "Alice");
/// assert_eq!(friend.peer(), "123456");
/// ```
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Builder)]
#[serde(bound(deserialize = "'de: 'c"))]
#[builder(setter(into), pattern = "owned")]
pub struct FriendContact<'c> {
	/// 好友ID
	#[serde(borrow)]
	peer: Cow<'c, str>,
	/// 好友名称
	#[builder(default, setter(into, strip_option))]
	#[serde(borrow)]
	name: Option<Cow<'c, str>>,
}
impl<'c> FriendContact<'c> {
	pub fn new<N>(peer: N, name: N) -> Self
	where
		N: Into<Cow<'c, str>>,
	{
		Self { peer: peer.into(), name: Some(name.into()) }
	}

	pub fn builder() -> FriendContactBuilder<'c> {
		FriendContactBuilder::default()
	}
}

impl<'c> Contact for FriendContact<'c> {
	fn scene(&self) -> &SceneType {
		&SceneType::Friend
	}

	fn peer(&self) -> &str {
		self.peer.as_ref()
	}

	fn name(&self) -> Option<&str> {
		self.name.as_deref()
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
