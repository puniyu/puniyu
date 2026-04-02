use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupRecallType {
	pub message_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupFileUploadType {
	/// 文件id
	pub file_id: String,
	/// 文件名
	pub file_name: String,
	/// 文件大小
	pub file_size: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupMemberAddType {
	/// 加入的用户id
	pub target_id: String,
	pub join_type: GroupJoinType,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum GroupJoinType {
	/// 邀请
	Invite,
	/// 群管理同意
	Apply,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupMemberDecreaseType {
	/// 离开的用户id
	pub target_id: String,
	pub leave_type: GroupLeaveType,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum GroupLeaveType {
	/// 主动退群
	Leave,
	/// 群成员被踢
	Kick,
	/// Bot自身被踢
	KickBot,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupMemberBanType {
	/// 被禁用的用户id
	pub target_id: String,
	/// 禁言时长(秒)
	pub duration: u64,
	pub ban_type: GroupBanType,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum GroupBanType {
	/// 禁言
	Ban,
	/// 解除禁言
	Unban,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupWholeBanType {
	pub action: GroupWholeBanActionType,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum GroupWholeBanActionType {
	/// 全员禁言
	WholeBan,
	/// 全员解除禁言
	WholeUnban,
}

