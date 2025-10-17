mod registry;
pub use registry::MatcherRegistry;
mod message;
mod store;

pub use message::MessageMatcher;
use puniyu_event_core::Event;

/// 事件匹配器
pub trait Matcher: Send + Sync + 'static {
	/// 匹配事件
	fn matches(&self, event: &Event) -> (Option<Vec<String>>, Option<String>);
	/// 获取匹配器名称
	fn name(&self) -> &str;

	/// 获取匹配器优先级
	fn rank(&self) -> u8 {
		5
	}
}
