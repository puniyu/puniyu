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
/// # 字段
///
/// - `peer` - 群 ID
/// - `name` - 群名称（可选）
///
/// # 示例
///
/// ```rust
/// use puniyu_contact::GroupTempContact;
///
/// let group_temp = GroupTempContact {
///     peer: "789012".into(),
///     name: Some("Temp Team".into()),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Builder)]
#[serde(bound(deserialize = "'de: 'c"))]
#[builder(setter(into), pattern = "owned")]
pub struct GroupTempContact<'c> {
	/// 群 ID
	#[serde(borrow)]
	pub peer: Cow<'c, str>,
	/// 群名称
	#[builder(default, setter(into, strip_option))]
	#[serde(borrow)]
	pub name: Option<Cow<'c, str>>,
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
