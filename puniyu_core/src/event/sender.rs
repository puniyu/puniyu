use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// 性别
pub enum Sex {
    /// 男性
    Male,
    /// 女性
    Female,
    /// 未知
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// 事件发送者角色，私聊时不存在
pub enum Role {
    /// 群主
    Owner,
    /// 管理员
    Admin,
    /// 成员
    Member,
    /// 未知
    Unknown,
}
