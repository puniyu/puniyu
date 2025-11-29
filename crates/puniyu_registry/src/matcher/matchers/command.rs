use puniyu_logger::{debug, info};
use puniyu_types::event::EventBase;
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
	fn check_message(event: &Event) -> bool {
		if let Event::Message(message_event) = event {
			match message_event.as_ref() {
				MessageEvent::Friend(m) => {
					debug!("收到{}消息: {:?}", "好友", m.elements());
					info!(
						"[Bot:{}] [{}消息:{}] {}",
						m.self_id(),
						"好友",
						m.user_id(),
						m.elements().raw()
					);
				}
				MessageEvent::Group(m) => {
					debug!("收到{}消息: {:?}", "群", m.elements());
					info!(
						"[Bot:{}] [{}消息:{}-{}] {}",
						m.self_id(),
						"群",
						m.group_id(),
						m.user_id(),
						m.elements().raw()
					);
				}
			};

			true
		} else {
			false
		}
	}
}

impl Matcher for CommandMatcher {
	fn name(&self) -> &str {
		EventType::Message.into()
	}

	fn matches(&self, event: &Event) -> bool {
		Self::check_message(event)
	}
}
