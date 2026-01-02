use async_trait::async_trait;
use super::{Result, Error};
use bytes::Bytes;

#[async_trait]
pub trait AccountApi: Send + Sync {
    /// 设置头像
    ///
    /// ## 参数
    /// `avatar` - 头像二进制数据
    ///
    async fn set_avatar(&self, _avatar: Bytes) -> Result<bool> {
        Err(Error::NotImpl)
    }
}