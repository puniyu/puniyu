use puniyu_logger::{debug, info};
use puniyu_types::matcher::Matcher;
use puniyu_types::{
	element::RawMessage,
	event::{
		Event, EventType,
		message::{MessageBase, MessageEvent},
	},
};

/// 消息匹配器
/// TODO:
///     - 全局前缀
///     - BOT前缀
///     - 插件前缀
pub struct CommandMatcher;

impl CommandMatcher {
	fn check_message<M: MessageBase + std::fmt::Display>(message: &M, msg_type: &str) -> bool {
		let has_valid_text = message
			.elements()
			.iter()
			.any(|element| element.as_text().is_some_and(|text| !text.trim().is_empty()));

		if has_valid_text {
			debug!("收到{}消息: {}", msg_type, message);
		}
		info!("{}", message.elements().raw());
		has_valid_text
	}
}

impl Matcher for CommandMatcher {
	fn name(&self) -> &str {
		EventType::Message.into()
	}

	fn matches(&self, event: &Event) -> bool {
		match event {
			Event::Message(message_event) => match message_event.as_ref() {
				MessageEvent::Friend(message) => Self::check_message(message, "好友"),
				MessageEvent::Group(message) => Self::check_message(message, "群"),
			},
			_ => false,
		}
	}
}
