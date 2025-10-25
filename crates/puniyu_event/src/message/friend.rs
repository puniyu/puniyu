use crate::message::{MessageBase, MessageBuilder, MessageSubType};
use crate::{EventBase, EventType};
use puniyu_contact::FriendContact;
use puniyu_element::Elements;
use puniyu_sender::FriendSender;
use std::fmt;

#[derive(Debug, Clone)]
pub struct FriendMessage {
	/// 事件id
	event_id: String,
	/// 时间戳
	time: u64,
	/// BotId
	self_id: String,
	/// 用户id
	user_id: String,
	/// 消息id
	message_id: String,
	/// 消息内容
	elements: Vec<Elements>,
	/// 事件联系人
	contact: FriendContact,
	/// 事件发送者
	sender: FriendSender,
}

impl FriendMessage {
	pub fn new(message_builder: MessageBuilder<FriendContact, FriendSender>) -> Self {
		Self {
			event_id: message_builder.event_id,
			time: message_builder.time,
			self_id: message_builder.self_id,
			user_id: message_builder.user_id,
			message_id: message_builder.message_id,
			elements: message_builder.elements,
			contact: message_builder.contact,
			sender: message_builder.sender,
		}
	}
}

impl EventBase for FriendMessage {
	type ContactType = FriendContact;

	type SenderType = FriendSender;

	fn time(&self) -> u64 {
		self.time
	}

	fn event(&self) -> &str {
		EventType::Message.into()
	}

	fn event_id(&self) -> &str {
		&self.event_id
	}

	fn sub_event(&self) -> &str {
		MessageSubType::Friend.into()
	}

	fn self_id(&self) -> &str {
		&self.self_id
	}

	fn user_id(&self) -> &str {
		&self.user_id
	}

	fn contact(&self) -> Self::ContactType {
		self.contact.clone()
	}

	fn sender(&self) -> Self::SenderType {
		self.sender.clone()
	}
}

impl MessageBase for FriendMessage {
	fn message_id(&self) -> &str {
		&self.message_id
	}
	fn elements(&self) -> Vec<Elements> {
		self.elements.clone()
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

#[cfg(feature = "message")]
#[macro_export]
macro_rules! create_friend_message {
	(
		$adapter:expr,
		$event_id:expr,
		$contact:expr,
		$self_id:expr,
		$user_id:expr,
		$message_id:expr,
		$elements:expr,
		$sender:expr,
		$time:expr
	) => {
		let builder = MessageBuilder {
			event_id: $event_id.to_string(),
			self_id: $self_id.to_string(),
			user_id: $user_id.to_string(),
			contact: $contact,
			sender: $sender,
			time: $time,
			message_id: $message_id.to_string(),
			elements: $elements,
		};
		let message = FriendMessage::new(builder);
		let event = Event::Message(Box::new(MessageEvent::Friend(message)));
		send_event($adapter, event);
	};
}
