use crate::sender::GroupSender;
use crate::contact::GroupContact;
use crate::element::Elements;
use super::{MessageBase, MessageBuilder, MessageSubType};
use crate::event::{EventBase, EventType};
use std::fmt;

#[derive(Debug, Clone)]
pub struct GroupMessage {
	/// 事件id
	event_id: String,
	/// 时间发生时间戳
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
	contact: GroupContact,
	/// 事件发送者
	sender: GroupSender,
}

impl GroupMessage {
	pub fn new(message_builder: MessageBuilder<GroupContact, GroupSender>) -> Self {
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

	pub fn group_id(&self) -> &str {
		&self.contact.peer
	}
}

impl EventBase for GroupMessage {
	type ContactType = GroupContact;

	type SenderType = GroupSender;

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
		MessageSubType::Group.into()
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

	fn sender(&self) -> GroupSender {
		self.sender.clone()
	}
}

impl MessageBase for GroupMessage {
	fn message_id(&self) -> &str {
		&self.message_id
	}

	fn elements(&self) -> Vec<Elements> {
		self.elements.clone()
	}
}

impl fmt::Display for GroupMessage {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("GroupMessage")
			.field("event_id", &self.event_id)
			.field("self_id", &self.self_id)
			.field("user_id", &self.user_id)
			.field("message_id", &self.message_id)
			.field("elements", &self.elements)
			.finish()
	}
}

#[cfg(feature = "event")]
#[macro_export]
macro_rules! create_group_message {
	(
		$bot:expr,
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
		let message = GroupMessage::new(builder);
		let event = Event::Message(Box::new(MessageEvent::Group(message)));
		send_event($bot, event);
	};
}
