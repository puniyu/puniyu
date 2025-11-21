use crate::event::Event;

pub trait Matcher: Send + Sync + 'static {
	/// 获取匹配器名称
	fn name(&self) -> &str;

	/// 获取匹配器优先级
	fn rank(&self) -> u8 {
		5
	}

	/// 匹配事件
	fn matches(&self, event: &Event) -> bool;
}
