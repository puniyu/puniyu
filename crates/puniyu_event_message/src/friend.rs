use crate::{MessageBase, MessageSubType};
use puniyu_element::Elements;
use puniyu_event_utils::EventType;
use puniyu_event_utils::contact::Contact;
use puniyu_event_utils::sender::Sender;
use std::fmt;

#[derive(Debug, Clone)]
pub struct FriendMessage {
	/// 事件id
	pub event_id: String,
	/// BotId
	pub self_id: String,
	/// 用户id
	pub user_id: String,
	/// 消息id
	pub message_id: String,
	/// 消息内容
	pub elements: Vec<Elements>,
	/// 事件联系人
	pub contact: Contact,
	/// 事件发送者
	pub sender: Sender,
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

	fn contact(&self) -> Contact {
		self.contact.clone()
	}

	fn self_id(&self) -> &str {
		&self.self_id
	}

	fn user_id(&self) -> &str {
		&self.user_id
	}

	fn sender(&self) -> Sender {
		self.sender.clone()
	}

	fn elements(&self) -> Vec<Elements> {
		self.elements.clone()
	}

	fn message_id(&self) -> &str {
		&self.message_id
	}
}

impl fmt::Display for FriendMessage {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("FriendMessage")
			.field("event_id", &self.event_id)
			.field("self_id", &self.self_id)
			.field("user_id", &self.user_id)
			.field("message_id", &self.message_id)
			.field("elements", &self.elements)
			.finish()
	}
}

#[macro_export]
macro_rules! create_friend_message {
	($adapter:expr, $event_id:expr, $contact:expr, $self_id:expr, $user_id:expr, $message_id:expr, $elements:expr, $sender:expr) => {{
		let message = FriendMessage {
			event_id: $event_id.into(),
			contact: $contact,
			self_id: $self_id.into(),
			user_id: $user_id.into(),
			message_id: $message_id.into(),
			elements: $elements,
			sender: $sender,
		};
		let event = Event::Message(std::sync::Arc::from($adapter), MessageEvent::Friend(message));
		send_event(event);
	}};
}
