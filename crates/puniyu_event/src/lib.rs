pub mod message;
mod types;
#[doc(inline)]
pub use types::*;

use ecow::EcoVec;
use puniyu_bot::Bot;
use puniyu_contact::ContactType;
use puniyu_element::receive::Elements;
use puniyu_sender::SenderType;

#[derive(Debug, Clone)]
pub enum Event {
	Message(message::MessageEvent),
}

impl Event {
	#[allow(unreachable_patterns)]
	pub fn as_message(&self) -> Option<&message::MessageEvent> {
		match self {
			Self::Message(event) => Some(event),
			_ => None,
		}
	}
}

macro_rules! forward_event {
	($name:ident -> $ret:ty) => {
		impl Event {
			pub fn $name(&self) -> $ret {
				match self {
					Self::Message(e) => e.$name(),
				}
			}
		}
	};
}

forward_event!(time -> u64);
forward_event!(event_type -> crate::EventType);
forward_event!(event_id -> &str);
impl Event {
	pub fn sub_event(&self) -> SubEventType {
		match self {
			Self::Message(e) => SubEventType::Message(e.sub_event()),
		}
	}
}
forward_event!(bot -> &Bot);
forward_event!(user_id -> &str);
forward_event!(contact -> ContactType);
forward_event!(sender -> SenderType);
forward_event!(message_id -> &str);
forward_event!(elements -> &EcoVec<Elements>);
forward_event!(get_text -> Vec<&str>);
forward_event!(get_at -> Vec<&str>);
forward_event!(get_reply_id -> Option<&str>);
