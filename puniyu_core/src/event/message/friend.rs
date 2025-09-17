use crate::event::EventType;
use crate::event::message::{MessageBase, MessageSubType};
use crate::message::element::receive::ReceiveElements;

pub struct FriendMessage {
	event_id: String,
	self_id: String,
	message_id: String,
	elements: Vec<ReceiveElements>,
	msg: Option<String>,
	raw_message: Option<String>,
}

pub struct FriendMessageBuilder {
	event_id: String,
	self_id: String,
	message_id: String,
	elements: Vec<ReceiveElements>,
	msg: Option<String>,
	raw_message: Option<String>,
}

impl FriendMessage {
	pub fn new(data: FriendMessageBuilder) -> Self {
		Self {
			event_id: data.event_id,
			self_id: data.self_id,
			message_id: data.message_id,
			elements: data.elements,
			msg: data.msg,
			raw_message: data.raw_message,
		}
	}
}

impl MessageBase for FriendMessage {
	fn event_id(&self) -> &str {
		&self.event_id
	}

	fn event(&self) -> &str {
		EventType::Message.into()
	}

	fn sub_event(&self) -> &str {
		MessageSubType::Friend.into()
	}

	fn self_id(&self) -> &str {
		&self.self_id
	}

	fn elements(&self) -> &Vec<ReceiveElements> {
		&self.elements
	}

	fn msg(&self) -> Option<&str> {
		self.msg.as_deref()
	}

	fn raw_message(&self) -> Option<&str> {
		self.raw_message.as_deref()
	}

	fn message_id(&self) -> &str {
		&self.message_id
	}
}
