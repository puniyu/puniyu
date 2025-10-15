use crate::event::{
	EventType,
	message::{MessageBase, MessageSubType},
};
use crate::message::element::receive::ReceiveElements;
use std::fmt::Display;

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
	pub elements: Vec<ReceiveElements>,
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

	fn get_user_id(&self) -> &str {
		&self.user_id
	}

	fn elements(&self) -> Vec<ReceiveElements> {
		self.elements.clone()
	}

	fn message_id(&self) -> &str {
		&self.message_id
	}
}

impl Display for FriendMessage {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
	($event_id:expr, $self_id:expr, $user_id:expr, $message_id:expr, $elements:expr) => {
		let message = FriendMessage {
			event_id: $event_id.into(),
			self_id: $self_id.into(),
			user_id: $user_id.into(),
			message_id: $message_id.into(),
			elements: $elements,
		};
		let event = Event::Message(MessageEvent::Friend(message));
		send_event(event);
	};
}
