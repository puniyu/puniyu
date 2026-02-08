mod friend;
#[doc(inline)]
pub use friend::*;
mod group;
#[doc(inline)]
pub use group::*;

use super::{EventBase, EventType};
use crate::bot::Bot;
use strum::{Display, EnumString, IntoStaticStr};
use serde::{Deserialize, Serialize};

#[derive(
	Debug,
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

#[derive(Debug, Clone)]
pub enum NotionEvent<'n> {
	ReceiveLike(ReceiveLike<'n>),
	FriendAdd(FriendDecrease<'n>),
	PrivatePoke(PrivateRecall<'n>),
	PrivateRecall(PrivateRecall<'n>),
	PrivateFileUpload(PrivateFileUpload<'n>),
	GroupPoke(GroupPoke<'n>),
	GroupRecall(GroupRecall<'n>),
	GroupFileUpload(GroupFileUpload<'n>),
	GroupCardChange(GroupCardChange<'n>),
	GroupMemberTitleChange(GroupMemberTitleChange<'n>),
}
pub trait NotionBase: Send + Sync + EventBase<EventType, NotionSubEvent> {
	type Content;
	/// 通知消息
	fn notion(&self) -> &str;

	/// 通知内容
	fn content(&self) -> &Self::Content;
}

pub struct NotionBuilder<'n, Contact, Sender>
where
	Contact: crate::contact::Contact,
	Sender: crate::sender::Sender,
{
	pub bot: &'n Bot,
	pub event_id: &'n str,
	pub time: u64,
	pub self_id: &'n str,
	pub user_id: &'n str,
	pub contact: &'n Contact,
	pub sender: &'n Sender,
}
