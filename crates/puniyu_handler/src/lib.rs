mod types;
#[doc(inline)]
pub use types::*;
#[cfg(feature = "registry")]
mod registry;
#[cfg(feature = "registry")]
pub use registry::HandlerRegistry;

use async_trait::async_trait;
use puniyu_error::Result;
use puniyu_event::Event;

#[async_trait]
pub trait Handler: Send + Sync + 'static {
	/// 处理器名称
	fn name(&self) -> &'static str;

	/// 优先级
	fn priority(&self) -> u32 {
		5
	}

	/// 处理事件
	///
	/// - `event`: 事件
	///
	async fn handle(&self, event: &Event) -> Result;
}

impl PartialEq for dyn Handler {
	fn eq(&self, other: &Self) -> bool {
		self.name() == other.name() && self.priority() == other.priority()
	}
}
