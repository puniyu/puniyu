use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

#[derive(
	Debug,
	Clone,
	Default,
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
pub enum NotionType {
	#[strum(serialize = "friendAdd")]
	/// 好友增加
	FriendAdd,
	#[strum(serialize = "friendDecrease")]
	/// 好友删除
	FriendDecrease,
	#[strum(serialize = "privateRecall")]
	/// 私聊撤回
	PrivateRecall,
	#[strum(serialize = "privateFileUpload")]
	PrivateFileUpload,
	#[strum(serialize = "GroupRecall")]
	/// 群聊撤回
	GroupRecall,
	#[strum(serialize = "groupFileUpload")]
	/// 群文件上传
	GroupFileUpload,
	#[strum(serialize = "groupMemberAdd")]
	/// 群成员增加
	GroupMemberAdd,
	#[strum(serialize = "groupMemberDecrease")]
	/// 群成员减少
	GroupMemberDecrease,
	#[strum(serialize = "groupMemberBan")]
	/// 群成员禁言
	GroupMemberBan,
	#[strum(serialize = "groupWholeBan")]
	/// 群全员禁言
	GroupWholeBan,
	/// 全部事件
	#[strum(serialize = "all")]
	#[default]
	All,
}
