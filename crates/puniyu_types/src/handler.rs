use crate::event::Event;
use async_trait::async_trait;

pub type HandlerResult<T = ()> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// 事件处理器
#[async_trait]
pub trait Handler: Send + Sync + 'static {
	/// 处理器名称
	fn name(&self) -> &str;

	/// 优先级
	fn rank(&self) -> u32 {
		5
	}

	/// 处理事件
	///
	/// - `event`: 事件
	///
	async fn handle(&self, event: &Event) -> HandlerResult;
}
