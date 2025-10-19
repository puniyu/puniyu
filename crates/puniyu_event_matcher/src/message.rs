use super::Matcher;
use puniyu_element::RawMessage;
use puniyu_event_core::Event;
use puniyu_event_message::{MessageBase, MessageEvent};
use puniyu_event_utils::EventType;
use puniyu_logger::{debug, info};

/// 消息匹配器
/// TODO:
///     - 全局前缀
///     - BOT前缀
///     - 插件前缀
pub struct MessageMatcher;

impl Matcher for MessageMatcher {
	fn matches(&self, event: &Event) -> bool {
		if let Event::Message(.., message_event) = event {
			match message_event {
				MessageEvent::Friend(message) => {
					debug!("收到好友消息: {}", message);
					info!("{}", message.elements().raw());
				}
				MessageEvent::Group(message) => {
					debug!("收到群消息: {}", message);
					info!("{}", message.elements().raw());
				}
			}
			true
		} else {
			false
		}
	}

	fn name(&self) -> &str {
		EventType::Message.into()
	}
}
