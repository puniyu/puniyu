mod error;
pub use error::Error;

use async_trait::async_trait;
use puniyu_param::ParamValue;
use puniyu_session::MessageSession;

/// 从事件会话中提取参数。
#[async_trait]
pub trait Extractor: Send + Sync {
	/// 从会话中提取数据。
	///
	/// - `Ok(ParamValue)` — 提取成功（含 [`ParamValue::Empty`] 表示无数据）
	/// - `Err(Error)` — 提取失败
	async fn extract(&self, session: &MessageSession) -> Result<ParamValue, Error>;
}
