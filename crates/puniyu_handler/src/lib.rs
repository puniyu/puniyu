mod registry;

pub use registry::HandlerRegistry;
use std::sync::Arc;
mod command;
pub use command::CommandHandler;
mod store;

use async_trait::async_trait;
use puniyu_adapter_api::AdapterApi;
use puniyu_event::Event;

/// 事件处理器
#[async_trait]
pub trait Handler: Send + Sync {
	async fn handle(&self, adapter: Arc<dyn AdapterApi>, event: Event);
	fn name(&self) -> &str;

	fn rank(&self) -> u8 {
		5
	}
}
