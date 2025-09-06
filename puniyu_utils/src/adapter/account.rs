use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// 账户信息
pub struct AccountInfo {
    /// Bot账号的uin
    pub uin: &'static str,
    /// Bot账号的uid
    pub uid: Option<&'static str>,
    /// Bot的selfId 一般使用此参数即可
    pub self_id: &'static str,
    /// Bot账号的昵称
    pub name: &'static str,
    /// Bot账号的头像
    pub avatar: &'static str,
}
