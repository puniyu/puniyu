use super::inner::Error;
use async_trait::async_trait;
use puniyu_error::Result;
use puniyu_adapter_types::Avatar;

#[async_trait]
pub trait AccountApi: Send + Sync {
	/// 设置头像
	///
	/// ## 参数
	/// `avatar` - 头像二进制数据
	///
	async fn set_avatar(&self, avatar: Avatar<'_>) -> Result<()> {
		Err(Error::NotImpl.into())
	}
}
