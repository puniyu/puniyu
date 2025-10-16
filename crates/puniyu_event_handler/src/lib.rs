mod registry;
pub use registry::HandlerRegistry;
mod message;
pub use message::MessageHandler;

use async_trait::async_trait;
use puniyu_event_core::Event;

#[derive(Debug, Clone)]
pub enum HandlerResult {
	/// 处理完成，或者是不处理
	Ok,
	/// 继续处理
	Continue,
}

/// 事件处理器
#[async_trait]
pub trait Handler: Send + Sync {
	async fn handle(&self, event: &Event) -> HandlerResult;
	fn name(&self) -> &str;

	fn rank(&self) -> u8 {
		5
	}
}
