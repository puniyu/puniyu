use crate::request::{GroupApplyType, GroupInviteType, PrivateApplyType};
use crate::types::codegen_from_for_content_type;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

/// 请求子类型枚举
///
/// 定义所有可能的请求类型。
///
/// # 变体
///
/// - `PrivateApply` - 好友申请请求
/// - `GroupApply` - 群申请请求（用户申请加入群）
/// - `GroupInvite` - 群邀请请求（邀请机器人加入群）
///
/// # 示例
///
/// ```rust
/// use puniyu_event::request::RequestSubEventType;
/// use std::str::FromStr;
///
/// // 从字符串解析
/// let request_type = RequestSubEventType::from_str("privateApply").unwrap();
/// assert_eq!(request_type, RequestSubEventType::PrivateApply);
///
/// // 转换为字符串
/// assert_eq!(request_type.to_string(), "privateApply");
/// ```
///
/// # 序列化
///
/// ```rust
/// use puniyu_event::request::RequestSubEventType;
/// use serde_json;
///
/// let request_type = RequestSubEventType::GroupApply;
/// let json = serde_json::to_string(&request_type).unwrap();
/// assert_eq!(json, r#""groupApply""#);
/// ```
#[derive(
	Debug,
	Copy,
	Clone,
	EnumString,
	Display,
	IntoStaticStr,
	Deserialize,
	Serialize,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
)]
#[strum(serialize_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub enum RequestSubEventType {
	/// 好友申请
	PrivateApply,
	/// 群申请
	GroupApply,
	/// 邀请入群
	GroupInvite,
}

/// 请求内容类型枚举
///
/// 包含所有请求事件的具体内容类型。
///
/// 每个变体对应一种请求类型，包含该请求的详细信息。
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_event::request::ContentType;
///
/// match content {
///     ContentType::PrivateApply(apply) => {
///         println!("收到好友申请");
///     }
///     ContentType::GroupInvite(invite) => {
///         println!("收到群邀请");
///     }
///     _ => {}
/// }
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ContentType {
	/// 好友申请
	PrivateApply(PrivateApplyType),
	/// 群申请
	GroupApply(GroupApplyType),
	/// 群邀请
	GroupInvite(GroupInviteType),
}

codegen_from_for_content_type! {
	PrivateApply(PrivateApplyType),
	GroupApply(GroupApplyType),
	GroupInvite(GroupInviteType)
}
