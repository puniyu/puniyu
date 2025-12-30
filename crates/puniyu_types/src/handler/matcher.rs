use crate::event::Event;

/// 事件匹配
pub trait Matcher: Send + Sync + 'static {
	/// 匹配事件，返回匹配结果
	fn matches(&self, event: &Event) -> bool;
}
