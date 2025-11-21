#[derive(Debug, Clone)]
pub struct GroupPokeType {
	/// 被戳的用户id
	pub target_id: String,
}

#[derive(Debug, Clone)]
pub struct GroupRecallType {
	pub message_id: String,
}

#[derive(Debug, Clone)]
pub struct GroupFileUploadType {
	/// 文件id
	pub file_id: String,
	/// 文件名
	pub file_name: String,
	/// 文件大小
	pub file_size: u64,
}

#[derive(Debug, Clone)]
pub struct GroupCardChangeType {
	/// 群名片，新的群名片
	pub card: String,
}

#[derive(Debug, Clone)]
pub struct GroupMemberTitleChangeType {
	pub title: String,
}

#[derive(Debug, Clone)]
pub enum HighlightsAction {
	/// 添加精华
	Add,
	/// 移除精华
	Remove,
}

#[derive(Debug, Clone)]
pub struct GroupHighlightsChangeType {
	/// 被添加精华消息的用户id
	pub target_id: String,
	pub message_id: String,
	pub action: HighlightsAction,
}

#[derive(Debug, Clone)]
pub struct GroupMemberAddType {
	/// 加入的用户id
	pub target_id: String,
	pub join_type: GroupJoinType,
}

#[derive(Debug, Clone)]
pub enum GroupJoinType {
	/// 邀请
	Invite,
	/// 群管理同意
	Apply,
}


#[derive(Debug, Clone)]
pub struct GroupMemberDecreaseType {
	/// 离开的用户id
	pub target_id: String,
	pub leave_type: GroupLeaveType,
}

#[derive(Debug, Clone)]
pub enum GroupLeaveType {
	/// 主动退群
	Leave,
	/// 群成员被踢
	Kick,
	/// Bot自身被踢
	KickBot,
}

#[derive(Debug, Clone)]
pub struct GroupAdminChangeType {
	/// 被操作的id
	pub target_id: String,
	pub admin_type: GroupAdminType,
}

#[derive(Debug, Clone)]
pub enum GroupAdminType {
	/// 设置
	Set,
	/// 取消
	Remove,
}

#[derive(Debug, Clone)]
pub struct GroupMemberBanType {
	/// 被禁用的用户id
	pub target_id: String,
	/// 禁言时长(秒)
	pub duration: u64,
	pub ban_type: GroupBanType,
}

#[derive(Debug, Clone)]
pub enum GroupBanType {
	/// 禁言
	Ban,
	/// 解除禁言
	Unban,
}

#[derive(Debug, Clone)]
pub struct GroupWholeBanType {
	pub action: GroupWholeBanActionType,
}

#[derive(Debug, Clone)]
pub enum GroupWholeBanActionType {
	/// 全员禁言
	WholeBan,
	/// 全员解除禁言
	WholeUnban,
}

#[derive(Debug, Clone)]
pub struct GroupMessageReactionType {
	pub message_id: String,
	pub face_id: u32,
	pub count: u8,
	pub action: GroupMessageReactionAction,
}

#[derive(Debug, Clone)]
pub enum GroupMessageReactionAction {
	/// 添加
	Add,
	/// 移除
	Remove,
}

#[derive(Debug, Clone)]
pub struct GroupLuckKingType {
	/// 运气王id
	pub target_id: String,
}

#[derive(Debug, Clone)]
pub struct GroupHonorChangeType {
	pub honor_type: HonorType,
}

#[derive(Debug, Clone)]
pub enum HonorType {
	/// 龙王
	Talkative,
	/// 群聊之火
	Performer,
	/// 快乐源泉
	Emotion,
}