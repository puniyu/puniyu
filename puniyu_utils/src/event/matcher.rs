use crate::event::EventType;

/// 事件匹配器
pub trait Matcher: Send + Sync + 'static {
	/// 匹配事件
	///
	/// ## 参数
	/// `evet` - [EventType]
	/// `data` - 事件内容
	fn matches(&self, event: EventType, data: String) -> bool;

	/// 获取匹配器名称
	fn name(&self) -> &str;

	/// 获取匹配器优先级
	fn rank(&self) -> u8 {
		5
	}
}

/// 消息匹配器
/// TODO:
///     - 全局前缀
///     - BOT前缀
///     - 插件前缀
struct MessageMatcher;

impl Matcher for MessageMatcher {
	fn matches(&self, event: EventType, data: String) -> bool {
		if event.is_message() {
			return !data.is_empty();
		}
		false
	}

	fn name(&self) -> &str {
		EventType::Message.into()
	}
}
