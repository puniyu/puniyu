//! 群聊联系人模块
//!
//! 提供群聊联系人与群临时联系人的类型定义和构建宏。

mod temp;
pub use temp::*;

use crate::{Contact, SceneType};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// 群聊联系人
///
/// 表示一个群聊的联系信息。
///
/// # 字段
///
/// - `peer` - 群聊 ID
/// - `name` - 群名称（可选）
///
/// # 示例
///
/// ```rust
/// use puniyu_contact::GroupContact;
///
/// let group = GroupContact {
///     peer: "789012".into(),
///     name: Some("Dev Team".into()),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Builder)]
#[serde(bound(deserialize = "'de: 'c"))]
#[builder(setter(into), pattern = "owned")]
pub struct GroupContact<'c> {
	/// 群聊id
	#[serde(borrow)]
	pub peer: Cow<'c, str>,
	/// 群名称
	#[builder(default, setter(into, strip_option))]
	#[serde(borrow)]
	pub name: Option<Cow<'c, str>>,
}

impl<'c> GroupContact<'c> {
	pub fn new<N>(peer: N, name: N) -> Self
	where
		N: Into<Cow<'c, str>>,
	{
		Self { peer: peer.into(), name: Some(name.into()) }
	}

	pub fn builder() -> GroupContactBuilder<'c> {
		GroupContactBuilder::default()
	}
}

impl<'c> Contact for GroupContact<'c> {
	fn scene(&self) -> &SceneType {
		&SceneType::Group
	}

	fn peer(&self) -> &str {
		self.peer.as_ref()
	}

	fn name(&self) -> Option<&str> {
		self.name.as_deref()
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
        $crate::GroupContactBuilder::default()
		$(
			.$key($value)
		)*
		.build()
		.expect("Failed to build GroupContact")
    }};

    ($peer:expr, $name:expr) => {{
        $crate::GroupContactBuilder::default()
            .peer($peer)
            .name($name)
            .build()
            .expect("Failed to build GroupContact")
    }};

    ($peer:expr) => {{
        $crate::GroupContactBuilder::default()
            .peer($peer)
            .build()
            .expect("Failed to build GroupContact")
    }};
}
