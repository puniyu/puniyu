use serde::Serialize;
use crate::sender::Sex;

#[derive(Debug, Clone, Serialize)]
pub struct UserInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户昵称
    pub nick: String,
    /// 用户头像
    pub avatar: String,
    /// 用户年龄
    pub age: Option<u8>,
    /// 用户性别
    pub sex: Sex,
}