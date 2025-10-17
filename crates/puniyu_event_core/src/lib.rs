use puniyu_adapter_builder::AdapterApi;
use puniyu_event_message::MessageEvent;
use std::fmt;
use std::sync::Arc;

#[derive(Clone)]
pub enum Event {
	Message(Arc<dyn AdapterApi>, MessageEvent),
	Notion,
}

impl fmt::Debug for Event {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Event::Message(_, message_event) => f
				.debug_tuple("Message")
				.field(&format_args!("Arc<dyn AdapterApi>"))
				.field(message_event)
				.finish(),
			Event::Notion => write!(f, "Notion"),
		}
	}
}

impl Event {
	/// 判断是否为消息事件
	pub fn is_message(&self) -> bool {
		matches!(self, Event::Message(..))
	}
}
