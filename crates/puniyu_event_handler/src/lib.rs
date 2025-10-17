mod registry;
pub use registry::HandlerRegistry;
mod message;
mod store;

pub use message::MessageHandler;

use async_trait::async_trait;
use puniyu_event_core::Event;

/// 事件处理器
#[async_trait]
pub trait Handler: Send + Sync {
	async fn handle(&self, event: &Event, plugin_name: &[String], command_name: String);
	fn name(&self) -> &str;

	fn rank(&self) -> u8 {
		5
	}
}
