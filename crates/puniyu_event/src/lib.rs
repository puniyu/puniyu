#[cfg(feature = "context")]
pub mod context;
#[cfg(feature = "message")]
pub mod message;

#[cfg(feature = "message")]
use crate::message::MessageEvent;
#[cfg(feature = "message")]
use puniyu_adapter_api::AdapterApi;
use std::sync::Arc;
use strum::{Display, EnumString, IntoStaticStr};

#[cfg(feature = "message")]
#[derive(Clone)]
pub enum Event {
	Message(Arc<dyn AdapterApi>, MessageEvent),
	Notion,
}

#[cfg(feature = "message")]
impl std::fmt::Debug for Event {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

#[cfg(feature = "message")]
impl Event {
	/// 判断是否为消息事件
	pub fn is_message(&self) -> bool {
		matches!(self, Event::Message(..))
	}
}

#[derive(Debug, Clone, EnumString, Display, IntoStaticStr)]
pub enum EventType {
	#[strum(serialize = "message")]
	Message,
	#[strum(serialize = "notice")]
	Notice,
	#[strum(serialize = "request")]
	Request,
	#[strum(serialize = "unknown")]
	Unknown,
}
