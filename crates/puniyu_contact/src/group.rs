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
/// - `peer` - 群聊 ID
/// - `name` - 群名称（可选）
///
/// # 示例
///
/// ```rust
/// use puniyu_contact::GroupContact;
///
/// let group = GroupContact {
///     peer: "789012",
///     name: Some("Dev Team"),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Builder)]
#[builder(setter(into), pattern = "owned")]
pub struct GroupContact<'c> {
	/// 群聊id
	pub peer: &'c str,
	/// 群名称
	#[builder(default)]
	pub name: Option<&'c str>,
}


impl<'c> Contact for GroupContact<'c> {
	fn scene(&self) -> &SceneType {
		&SceneType::Group
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
