use super::Matcher;
use puniyu_event_core::Event;
use puniyu_event_message::MessageEvent;
use puniyu_event_utils::EventType;
use puniyu_logger::info;

/// 消息匹配器
/// TODO:
///     - 全局前缀
///     - BOT前缀
///     - 插件前缀
pub struct MessageMatcher;

impl Matcher for MessageMatcher {
	fn matches(&self, event: &Event) -> bool {
		if let Event::Message(message_event) = event {
			match message_event {
				MessageEvent::Friend(message) => {
					info!("收到好友消息: {}", message);
					true
				}
			}
		} else {
			false
		}
	}

	fn name(&self) -> &str {
		EventType::Message.into()
	}
}
