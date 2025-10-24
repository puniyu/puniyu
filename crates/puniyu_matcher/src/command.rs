use super::Matcher;
use puniyu_element::RawMessage;
use puniyu_event::message::{MessageBase, MessageEvent};
use puniyu_event::{Event, EventType};
use puniyu_logger::{debug, info};

/// 消息匹配器
/// TODO:
///     - 全局前缀
///     - BOT前缀
///     - 插件前缀
pub struct CommandMatcher;

impl Matcher for CommandMatcher {
	fn matches(&self, event: &Event) -> bool {
		if let Event::Message(message_event) = event {
			match message_event.as_ref() {
				MessageEvent::Friend(message) => {
					let has_valid_text = message.elements().iter().any(|element| {
						if let Some(text) = element.as_text() {
							!text.trim().is_empty()
						} else {
							false
						}
					});
					if has_valid_text {
						debug!("收到好友消息: {}", message);
						info!("{}", message.elements().raw());
					}
					has_valid_text
				}
				MessageEvent::Group(message) => {
					let has_valid_text = message.elements().iter().any(|element| {
						if let Some(text) = element.as_text() {
							!text.trim().is_empty()
						} else {
							false
						}
					});
					if has_valid_text {
						debug!("收到群消息: {}", message);
						info!("{}", message.elements().raw());
					}
					has_valid_text
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
