use puniyu_sender::Role;
use puniyu_sender::Sex;
use serde::{Deserialize, Serialize};

/// 群资料。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupInfo {
	/// 群 ID。
	pub group_id: String,
	/// 群名称。
	pub group_name: String,
	/// 群头像地址。
	pub avatar: String,
}

/// 群成员资料。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MemberInfo {
	/// 用户 ID。
	pub user_id: String,
	/// 用户昵称。
	pub nick: String,
	/// 用户角色。
	pub role: Role,
	/// 群头衔，为空表示无头衔。
	pub title: Option<String>,
	/// 群名片。
	pub card: String,
	/// 用户头像地址。
	pub avatar: String,
	/// 用户年龄。
	pub age: Option<u32>,
	/// 用户性别。
	pub sex: Sex,
	/// 入群时间戳，单位为秒。
	pub joined_at: u64,
}

/// 群禁言动作。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MuteType {
	/// 设置禁言。
	Set,
	/// 解除禁言。
	Remove,
}

/// 群管理员设置动作。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum SetAdminType {
	/// 设置管理员。
	Set,
	/// 取消管理员。
	Remove,
}

/// 入群申请处理动作。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum SetGroupApplyType {
	/// 同意入群申请。
	Agree,
	/// 拒绝入群申请。
	Refuse,
}

/// 群禁言信息。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupMuteInfo {
	/// 用户 ID。
	pub user_id: String,
	/// 禁言时长，单位为秒。
	pub mute_time: u64,
}
