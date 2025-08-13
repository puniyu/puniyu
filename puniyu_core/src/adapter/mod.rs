pub mod info;
use super::adapter::info::{AccountInfo, AdapterInfo};

/// 适配器基类
/// 开发者需要自行实现开发适配器，部分函数需要开发者自行实现
/// TODO: 未完成消息发送以及获取消息
pub trait AdapterBase {
    /// 获取适配器信息
    fn adapter(&self) -> AdapterInfo;
    /// 获取账户信息
    fn account(&self) -> AccountInfo;

    /// 获取Bot账号的selfId
    ///
    /// # 返回值
    ///
    /// * `String` - Bot账号的selfId
    fn get_self_id(&self) -> String {
        self.account().self_id
    }

    /// 获取Bot账号的昵称
    ///
    /// # 返回值
    ///
    /// * `String` - Bot账号的昵称
    fn get_self_name(&self) -> String {
        self.account().name
    }

    /// 打印适配器专属日志
    fn logger(&self, msg: &str) {
        log::info!("[Adapter][{}]{}", self.adapter().name, msg);
    }
}
