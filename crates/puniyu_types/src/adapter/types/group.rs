use serde::{Deserialize, Serialize};
use crate::sender::{Role, Sex};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupInfo {
    /// 群ID
    pub group_id: String,
    /// 群名称
    pub group_name: String,
    /// 群头像
    pub avatar: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MemberInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户昵称
    pub nick: String,
    /// 用户角色
    pub role: Role,
    /// 群头衔, 如果为空则无
    pub title: Option<String>,
    /// 群名片, 如果为空则与昵称相同
    pub card: String,
    /// 用户头像
    pub avatar: String,
    /// 用户年龄
    pub age: Option<u8>,
    /// 用户性别
    pub sex: Sex,
    /// 加入时间戳(秒)
    pub joined_at: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MuteType {
    /// 禁言
    Set,
    /// 解除禁言
    Remove,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum SetAdminType {
    /// 设置管理员
    Set,
    /// 取消管理员
    Remove,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum SetGroupApplyType {
    /// 同意入群申请
    Agree,
    /// 拒绝入群申请
    Refuse,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupMuteInfo {
    /// 用户ID
    pub user_id: String,
    /// 禁言时间(秒)
    pub mute_time: u64,
}