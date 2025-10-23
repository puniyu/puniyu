mod friend;
pub use friend::*;
mod group;
pub use group::*;

use crate::EventBase;
use strum::{Display, EnumString, IntoStaticStr};

#[derive(Debug, Clone, EnumString, Display, IntoStaticStr)]
pub enum NotionSubEvent {
	#[strum(serialize = "receiveLike")]
	/// 收到点赞
	ReceiveLike,
	#[strum(serialize = "friendAdd")]
	/// 好友增加
	FriendAdd,
	#[strum(serialize = "friendDecrease")]
	/// 好友删除
	FriendDecrease,
	#[strum(serialize = "privatePoke")]
	/// 私聊戳一戳
	PrivatePoke,
	#[strum(serialize = "privateRecall")]
	/// 私聊撤回
	PrivateRecall,
	#[strum(serialize = "privateFileUpload")]
	PrivateFileUpload,
	#[strum(serialize = "groupPoke")]
	/// 群戳一戳
	GroupPoke,
	#[strum(serialize = "GroupRecall")]
	/// 群聊撤回
	GroupRecall,
	#[strum(serialize = "groupFileUpload")]
	/// 群文件上传
	GroupFileUpload,
	#[strum(serialize = "groupCardChange")]
	/// 群名片修改
	GroupCardChange,
	#[strum(serialize = "groupMemberTitleChange")]
	/// 群成员头衔变动
	GroupMemberTitleChange,
	#[strum(serialize = "groupHighlightsChange")]
	/// 群精华消息变动
	GroupHighlightsChange,
	#[strum(serialize = "groupMemberAdd")]
	/// 群成员增加
	GroupMemberAdd,
	#[strum(serialize = "groupMemberDecrease")]
	/// 群成员减少
	GroupMemberDecrease,
	#[strum(serialize = "groupAdminChange")]
	/// 群管理员变动
	GroupAdminChange,
	#[strum(serialize = "groupSignIn")]
	/// 群打卡
	GroupSignIn,
	#[strum(serialize = "groupMemberBan")]
	/// 群成员禁言
	GroupMemberBan,
	#[strum(serialize = "groupWholeBan")]
	/// 群全员禁言
	GroupWholeBan,
	#[strum(serialize = "groupMessageReaction")]
	/// 群消息表情动态
	GroupMessageReaction,
	#[strum(serialize = "groupLuckKing")]
	/// 群幸运王
	GroupLuckKing,
	#[strum(serialize = "groupHonorChange")]
	/// 群荣耀变动
	GroupHonorChange,
}

pub enum NotionEvent {
	ReceiveLike(ReceiveLike),
	FriendAdd(FriendAdd),
	PrivatePoke(PrivatePoke),
	PrivateRecall(PrivateRecall),
	PrivateFileUpload(PrivateFileUpload),
	GroupPoke(GroupPoke),
	GroupRecall(GroupRecall),
	GroupFileUpload(GroupFileUpload),
	GroupCardChange(GroupCardChange),
	GroupMemberTitleChange(GroupMemberTitleChange),
}
pub trait NotionBase: Send + Sync + EventBase {
	type Content;
	/// 通知消息
	fn notion(&self) -> &str;

	/// 通知内容
	fn content(&self) -> Self::Content;
}

#[derive(Debug, Clone)]
pub struct NotionBuilder<Contact, Sender> {
	pub event_id: String,
	pub self_id: String,
	pub user_id: String,
	pub contact: Contact,
	pub sender: Sender,
}
