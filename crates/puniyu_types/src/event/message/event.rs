use super::friend::FriendMessage;
use super::group::GroupMessage;
use crate::bot::Bot;
use crate::contact::ContactType;
use crate::element::receive::Elements;
use crate::sender::SenderType;
use bytes::Bytes;
use crate::event::{EventBase, EventType};
use crate::event::message::{MessageBase, MessageSubType};

#[derive(Debug, Clone)]
pub enum MessageEvent<'m> {
	Friend(FriendMessage<'m>),
	Group(GroupMessage<'m>),
}

impl<'m> MessageEvent<'m> {
	pub fn is_friend(&self) -> bool {
		match self {
			MessageEvent::Friend(message) => message.is_friend(),
			MessageEvent::Group(message) => message.is_friend(),
		}
	}

	pub fn is_group(&self) -> bool {
		match self {
			MessageEvent::Friend(message) => message.is_group(),
			MessageEvent::Group(message) => message.is_group(),
		}
	}

	pub fn as_friend(&self) -> Option<&FriendMessage> {
		match self {
			MessageEvent::Friend(msg) => Some(msg),
			_ => None,
		}
	}
	pub fn as_group(&self) -> Option<&GroupMessage> {
		match self {
			MessageEvent::Group(msg) => Some(msg),
			_ => None,
		}
	}

	pub fn bot(&self) -> &Bot {
		match self {
			MessageEvent::Friend(msg) => msg.bot(),
			MessageEvent::Group(msg) => msg.bot(),
		}
	}

	pub fn time(&self) -> u64 {
		match self {
			MessageEvent::Friend(msg) => msg.time(),
			MessageEvent::Group(msg) => msg.time(),
		}
	}

	pub fn event(&self) -> &EventType {
		match self {
			MessageEvent::Friend(msg) => msg.event(),
			MessageEvent::Group(msg) => msg.event(),
		}
	}

	pub fn event_id(&self) -> &str {
		match self {
			MessageEvent::Friend(msg) => msg.event_id(),
			MessageEvent::Group(msg) => msg.event_id(),
		}
	}

	pub fn sub_event(&self) -> &MessageSubType {
		match self {
			MessageEvent::Friend(msg) => msg.sub_event(),
			MessageEvent::Group(msg) => msg.sub_event(),
		}
	}

	pub fn self_id(&self) -> &str {
		match self {
			MessageEvent::Friend(msg) => msg.self_id(),
			MessageEvent::Group(msg) => msg.self_id(),
		}
	}

	pub fn user_id(&self) -> &str {
		match self {
			MessageEvent::Friend(msg) => msg.user_id(),
			MessageEvent::Group(msg) => msg.user_id(),
		}
	}

	pub fn message_id(&self) -> &str {
		match self {
			MessageEvent::Friend(msg) => msg.message_id(),
			MessageEvent::Group(msg) => msg.message_id(),
		}
	}

	pub fn contact(&self) -> ContactType {
		match self {
			MessageEvent::Friend(msg) => ContactType::Friend(msg.contact().clone()),
			MessageEvent::Group(msg) => ContactType::Group(msg.contact().clone()),
		}
	}

	pub fn sender(&self) -> SenderType {
		match self {
			MessageEvent::Friend(msg) => SenderType::Friend(msg.sender().clone()),
			MessageEvent::Group(msg) => SenderType::Group(msg.sender().clone()),
		}
	}

	pub fn elements(&self) -> Vec<Elements> {
		match self {
			MessageEvent::Friend(msg) => msg.elements(),
			MessageEvent::Group(msg) => msg.elements(),
		}
	}

	pub fn get_text(&self) -> Vec<&str> {
		match self {
			MessageEvent::Friend(msg) => msg.get_text(),
			MessageEvent::Group(msg) => msg.get_text(),
		}
	}

	pub fn get_at(&self) -> Vec<&str> {
		match self {
			MessageEvent::Friend(msg) => msg.get_at(),
			MessageEvent::Group(msg) => msg.get_at(),
		}
	}

	pub fn get_image(&self) -> Option<Bytes> {
		match self {
			MessageEvent::Friend(msg) => msg.get_image(),
			MessageEvent::Group(msg) => msg.get_image(),
		}
	}

	pub fn get_record(&self) -> Option<Bytes> {
		match self {
			MessageEvent::Friend(msg) => msg.get_record(),
			MessageEvent::Group(msg) => msg.get_record(),
		}
	}

	pub fn get_reply_id(&self) -> Option<&str> {
		match self {
			MessageEvent::Friend(msg) => msg.get_reply_id(),
			MessageEvent::Group(msg) => msg.get_reply_id(),
		}
	}

	pub fn group_id(&self) -> Option<&str> {
		match self {
			MessageEvent::Group(msg) => Some(msg.group_id()),
			_ => None,
		}
	}

	pub fn is_master(&self) -> bool {
		match self {
			MessageEvent::Friend(msg) => msg.is_master(),
			MessageEvent::Group(msg) => msg.is_master(),
		}
	}
}
