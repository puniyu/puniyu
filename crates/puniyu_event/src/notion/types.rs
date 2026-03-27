use crate::notion::{
	FriendAddType, FriendDecreaseType, GroupAdminChangeType, GroupCardChangeType,
	GroupFileUploadType, GroupHighlightsChangeType, GroupHonorChangeType, GroupLuckKingType,
	GroupMemberAddType, GroupMemberBanType, GroupMemberDecreaseType, GroupMemberTitleChangeType,
	GroupMessageReactionType, GroupPokeType, GroupRecallType, GroupSignInType, GroupWholeBanType,
	PrivateFileUploadType, PrivatePokeType, PrivateRecallType, ReceiveLikeType,
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
/// - `ReceiveLike` - 收到点赞通知
/// - `FriendAdd` - 好友增加通知
/// - `FriendDecrease` - 好友删除通知
/// - `PrivatePoke` - 私聊戳一戳通知
/// - `PrivateRecall` - 私聊消息撤回通知
/// - `PrivateFileUpload` - 私聊文件上传通知
///
/// # 群聊通知类型
///
/// - `GroupPoke` - 群戳一戳通知
/// - `GroupRecall` - 群消息撤回通知
/// - `GroupFileUpload` - 群文件上传通知
/// - `GroupCardChange` - 群名片修改通知
/// - `GroupMemberTitleChange` - 群成员头衔变动通知
/// - `GroupHighlightsChange` - 群精华消息变动通知
/// - `GroupMemberAdd` - 群成员增加通知
/// - `GroupMemberDecrease` - 群成员减少通知
/// - `GroupAdminChange` - 群管理员变动通知
/// - `GroupSignIn` - 群打卡通知
/// - `GroupMemberBan` - 群成员禁言通知
/// - `GroupWholeBan` - 群全员禁言通知
/// - `GroupMessageReaction` - 群消息表情动态通知
/// - `GroupLuckKing` - 群幸运王通知
/// - `GroupHonorChange` - 群荣耀变动通知
///
/// # 示例
///
/// ```rust
/// use puniyu_event::notion::NotionSubEventType;
/// use std::str::FromStr;
///
/// // 从字符串解析
/// let notion_type = NotionSubEventType::from_str("groupPoke").unwrap();
/// assert_eq!(notion_type, NotionSubEventType::GroupPoke);
///
/// // 转换为字符串
/// assert_eq!(notion_type.to_string(), "groupPoke");
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
	/// 收到点赞
	ReceiveLike,
	/// 好友增加
	FriendAdd,
	/// 好友删除
	FriendDecrease,
	/// 私聊戳一戳
	PrivatePoke,
	/// 私聊撤回
	PrivateRecall,
	/// 私聊文件上传
	PrivateFileUpload,
	/// 群戳一戳
	GroupPoke,
	/// 群聊撤回
	GroupRecall,
	/// 群文件上传
	GroupFileUpload,
	/// 群名片修改
	GroupCardChange,
	/// 群成员头衔变动
	GroupMemberTitleChange,
	/// 群精华消息变动
	GroupHighlightsChange,
	/// 群成员增加
	GroupMemberAdd,
	/// 群成员减少
	GroupMemberDecrease,
	/// 群管理员变动
	GroupAdminChange,
	/// 群打卡
	GroupSignIn,
	/// 群成员禁言
	GroupMemberBan,
	/// 群全员禁言
	GroupWholeBan,
	/// 群消息表情动态
	GroupMessageReaction,
	/// 群幸运王
	GroupLuckKing,
	/// 群荣耀变动
	GroupHonorChange,
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
///     ContentType::GroupPoke(poke) => {
///         println!("收到群戳一戳");
///     }
///     ContentType::FriendAdd(add) => {
///         println!("新增好友");
///     }
///     _ => {}
/// }
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ContentType {
	/// 收到点赞
	ReceiveLike(ReceiveLikeType),
	/// 好友增加
	FriendAdd(FriendAddType),
	/// 好友删除
	FriendDecrease(FriendDecreaseType),
	/// 私聊戳一戳
	PrivatePoke(PrivatePokeType),
	/// 私聊撤回
	PrivateRecall(PrivateRecallType),
	/// 私聊文件上传
	PrivateFileUpload(PrivateFileUploadType),
	/// 群戳一戳
	GroupPoke(GroupPokeType),
	/// 群聊撤回
	GroupRecall(GroupRecallType),
	/// 群文件上传
	GroupFileUpload(GroupFileUploadType),
	/// 群名片修改
	GroupCardChange(GroupCardChangeType),
	/// 群成员头衔变动
	GroupMemberTitleChange(GroupMemberTitleChangeType),
	/// 群精华消息变动
	GroupHighlightsChange(GroupHighlightsChangeType),
	/// 群成员增加
	GroupMemberAdd(GroupMemberAddType),
	/// 群成员减少
	GroupMemberDecrease(GroupMemberDecreaseType),
	/// 群管理员变动
	GroupAdminChange(GroupAdminChangeType),
	/// 群打卡
	GroupSignIn(GroupSignInType),
	/// 群成员禁言
	GroupMemberBan(GroupMemberBanType),
	/// 群全员禁言
	GroupWholeBan(GroupWholeBanType),
	/// 群消息表情动态
	GroupMessageReaction(GroupMessageReactionType),
	/// 群幸运王
	GroupLuckKing(GroupLuckKingType),
	/// 群荣耀变动
	GroupHonorChange(GroupHonorChangeType),
}

codegen_from_for_content_type! {
	ReceiveLike(ReceiveLikeType),
	FriendAdd(FriendAddType),
	FriendDecrease(FriendDecreaseType),
	PrivatePoke(PrivatePokeType),
	PrivateRecall(PrivateRecallType),
	PrivateFileUpload(PrivateFileUploadType),
	GroupPoke(GroupPokeType),
	GroupRecall(GroupRecallType),
	GroupFileUpload(GroupFileUploadType),
	GroupCardChange(GroupCardChangeType),
	GroupMemberTitleChange(GroupMemberTitleChangeType),
	GroupHighlightsChange(GroupHighlightsChangeType),
	GroupMemberAdd(GroupMemberAddType),
	GroupMemberDecrease(GroupMemberDecreaseType),
	GroupAdminChange(GroupAdminChangeType),
	GroupSignIn(GroupSignInType),
	GroupMemberBan(GroupMemberBanType),
	GroupWholeBan(GroupWholeBanType),
	GroupMessageReaction(GroupMessageReactionType),
	GroupLuckKing(GroupLuckKingType),
	GroupHonorChange(GroupHonorChangeType),
}
