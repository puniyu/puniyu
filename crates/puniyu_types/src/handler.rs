use crate::bot::Bot;
use crate::event::Event;
use async_trait::async_trait;

/// 事件处理器
#[async_trait]
pub trait Handler: Send + Sync {
	/// 处理器名称
	fn name(&self) -> &str;
	/// 优先级
	fn rank(&self) -> u8 {
		5
	}
	/// 处理事件
	async fn handle(&self, bot: Bot, event: Event);
}
