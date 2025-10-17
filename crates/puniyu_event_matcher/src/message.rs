use super::Matcher;
use puniyu_command_registry::CommandRegistry;
use puniyu_element::RawMessage;
use puniyu_event_core::Event;
use puniyu_event_message::MessageEvent;
use puniyu_event_utils::EventType;
use puniyu_logger::{debug, info};

/// 消息匹配器
/// TODO:
///     - 全局前缀
///     - BOT前缀
///     - 插件前缀
pub struct MessageMatcher;

impl Matcher for MessageMatcher {
	fn matches(&self, event: &Event) -> (Option<Vec<String>>, Option<String>) {
		if let Event::Message(.., message_event) = event {
			match message_event {
				MessageEvent::Friend(message) => {
					debug!("收到好友消息: {}", message);
					info!("{}", message.elements.raw());

					let mut matched_text = String::new();
					let matched_plugins = message
						.elements
						.iter()
						.filter_map(|element| {
							if let Some(text) = element.as_text() {
								matched_text = text.to_string();
								let plugin_names = CommandRegistry::get_plugins_with_command(text);
								Some(plugin_names)
							} else {
								None
							}
						})
						.flatten()
						.collect::<Vec<String>>();

					if !matched_plugins.is_empty() {
						(Some(matched_plugins), Some(matched_text))
					} else {
						(None, None)
					}
				}
			}
		} else {
			(None, None)
		}
	}

	fn name(&self) -> &str {
		EventType::Message.into()
	}
}
