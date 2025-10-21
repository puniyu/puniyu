mod registry;
pub use registry::HandlerRegistry;
mod message;
pub use message::MessageHandler;
mod store;

use async_trait::async_trait;
use puniyu_event::Event;

/// 事件处理器
#[async_trait]
pub trait Handler: Send + Sync {
	async fn handle(&self, event: &Event);
	fn name(&self) -> &str;

	fn rank(&self) -> u8 {
		5
	}
}
