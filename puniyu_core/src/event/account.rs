use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// 账户信息
pub struct AccountInfo {
    /// Bot账号的uin
    pub uin: String,
    /// Bot账号的uid
    pub uid: String,
    /// Bot的selfId 一般使用此参数即可
    pub self_id: String,
    /// Bot账号的昵称
    pub name: String,
    /// Bot账号的头像
    pub avatar: String,
}
