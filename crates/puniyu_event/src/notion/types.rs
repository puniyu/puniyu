use crate::notion::{
	FriendAddType, FriendDecreaseType, GroupFileUploadType, GroupMemberAddType,
	GroupMemberBanType, GroupMemberDecreaseType, GroupRecallType, GroupWholeBanType,
	PrivateFileUploadType, PrivateRecallType,
};
use crate::types::codegen_from_for_content_type;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

/// 通知子类型枚举
///
/// 定义所有可能的通知类型，包括好友通知和群聊通知。
///
/// # 好友通知类型
///
/// - `FriendAdd` - 好友增加通知
/// - `FriendDecrease` - 好友删除通知
/// - `PrivateRecall` - 私聊消息撤回通知
/// - `PrivateFileUpload` - 私聊文件上传通知
///
/// # 群聊通知类型
///
/// - `GroupRecall` - 群消息撤回通知
/// - `GroupFileUpload` - 群文件上传通知
/// - `GroupMemberAdd` - 群成员增加通知
/// - `GroupMemberDecrease` - 群成员减少通知
/// - `GroupMemberBan` - 群成员禁言通知
/// - `GroupWholeBan` - 群全员禁言通知
///
/// # 示例
///
/// ```rust
/// use puniyu_event::notion::NotionSubEventType;
/// use std::str::FromStr;
///
/// // 从字符串解析
/// let notion_type = NotionSubEventType::from_str("groupRecall").unwrap();
/// assert_eq!(notion_type, NotionSubEventType::GroupRecall);
///
/// // 转换为字符串
/// assert_eq!(notion_type.to_string(), "groupRecall");
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
pub enum NotionSubEventType {
	/// 好友增加
	FriendAdd,
	/// 好友删除
	FriendDecrease,
	/// 私聊撤回
	PrivateRecall,
	/// 私聊文件上传
	PrivateFileUpload,
	/// 群聊撤回
	GroupRecall,
	/// 群文件上传
	GroupFileUpload,
	/// 群成员增加
	GroupMemberAdd,
	/// 群成员减少
	GroupMemberDecrease,
	/// 群成员禁言
	GroupMemberBan,
	/// 群全员禁言
	GroupWholeBan,
}

/// 通知内容类型枚举
///
/// 包含所有通知事件的具体内容类型。
///
/// 每个变体对应一种通知类型，包含该通知的详细信息。
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_event::notion::ContentType;
///
/// match content {
///     ContentType::GroupRecall(recall) => {
///         println!("收到群撤回");
///     }
///     ContentType::FriendAdd(add) => {
///         println!("新增好友");
///     }
///     _ => {}
/// }
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ContentType {
	/// 好友增加
	FriendAdd(FriendAddType),
	/// 好友删除
	FriendDecrease(FriendDecreaseType),
	/// 私聊撤回
	PrivateRecall(PrivateRecallType),
	/// 私聊文件上传
	PrivateFileUpload(PrivateFileUploadType),
	/// 群聊撤回
	GroupRecall(GroupRecallType),
	/// 群文件上传
	GroupFileUpload(GroupFileUploadType),
	/// 群成员增加
	GroupMemberAdd(GroupMemberAddType),
	/// 群成员减少
	GroupMemberDecrease(GroupMemberDecreaseType),
	/// 群成员禁言
	GroupMemberBan(GroupMemberBanType),
	/// 群全员禁言
	GroupWholeBan(GroupWholeBanType),
}

codegen_from_for_content_type! {
	FriendAdd(FriendAddType),
	FriendDecrease(FriendDecreaseType),
	PrivateRecall(PrivateRecallType),
	PrivateFileUpload(PrivateFileUploadType),
	GroupRecall(GroupRecallType),
	GroupFileUpload(GroupFileUploadType),
	GroupMemberAdd(GroupMemberAddType),
	GroupMemberDecrease(GroupMemberDecreaseType),
	GroupMemberBan(GroupMemberBanType),
	GroupWholeBan(GroupWholeBanType),
}
