//! 群临时联系人模块
//!
//! 提供群临时联系人的类型定义和构建宏。

use crate::{Contact, SceneType};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// 群临时联系人
///
/// 表示一个群临时会话的联系信息。
///
/// # 示例
///
/// ```rust
/// use puniyu_contact::{Contact, GroupTempContact};
///
/// let group_temp = GroupTempContact::new("789012", "Temp Team");
/// assert_eq!(group_temp.peer(), "789012");
/// ```
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Builder)]
#[serde(bound(deserialize = "'de: 'c"))]
#[builder(setter(into), pattern = "owned")]
pub struct GroupTempContact<'c> {
	/// 群 ID
	#[serde(borrow)]
	peer: Cow<'c, str>,
	/// 群名称
	#[builder(default, setter(strip_option))]
	#[serde(borrow)]
	name: Option<Cow<'c, str>>,
}

impl<'c> GroupTempContact<'c> {
	pub fn new<P, N>(peer: P, name: N) -> Self
	where
		P: Into<Cow<'c, str>>,
		N: Into<Cow<'c, str>>,
	{
		Self { peer: peer.into(), name: Some(name.into()) }
	}

	pub fn builder() -> GroupTempContactBuilder<'c> {
		GroupTempContactBuilder::default()
	}
}

impl<'c> Contact for GroupTempContact<'c> {
	fn scene(&self) -> &SceneType {
		&SceneType::GroupTemp
	}

	fn peer(&self) -> &str {
		self.peer.as_ref()
	}

	fn name(&self) -> Option<&str> {
		self.name.as_deref()
	}
}

#[macro_export]
macro_rules! contact_group_temp {
    ( $( $key:ident : $value:expr ),+ $(,)? ) => {{
        $crate::GroupTempContactBuilder::default()
			$(
				.$key($value)
			)*
			.build()
			.expect("Failed to build GroupTempContact")
    }};

    ($peer:expr, $name:expr) => {{
        $crate::GroupTempContactBuilder::default()
            .peer($peer)
            .name($name)
            .build()
            .expect("Failed to build GroupTempContact")
    }};

    ($peer:expr) => {{
        $crate::GroupTempContactBuilder::default()
            .peer($peer)
            .build()
            .expect("Failed to build GroupTempContact")
    }};
}
