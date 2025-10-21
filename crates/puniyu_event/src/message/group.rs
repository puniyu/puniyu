use crate::EventType;
use crate::message::{MessageBase, MessageBuilder, MessageSubType};
use puniyu_contact::{Contact, GroupContact};
use puniyu_element::Elements;
use puniyu_sender::{GroupSender, Sender};
use std::fmt;

#[derive(Debug, Clone)]
pub struct GroupMessage {
	/// 事件id
	event_id: String,
	/// BotId
	self_id: String,
	/// 用户id
	user_id: String,
	/// 当前群昵称
	group_id: String,
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
	pub fn new(
		message_builder: MessageBuilder,
		group_id: String,
		contact: GroupContact,
		sender: GroupSender,
	) -> Self {
		Self {
			event_id: message_builder.event_id,
			self_id: message_builder.self_id,
			user_id: message_builder.user_id,
			group_id,
			message_id: message_builder.message_id,
			elements: message_builder.elements,
			contact,
			sender,
		}
	}

	pub fn group_id(&self) -> &str {
		&self.group_id
	}
}
impl MessageBase for GroupMessage {
	fn event_id(&self) -> &str {
		&self.event_id
	}

	fn event(&self) -> &str {
		EventType::Message.into()
	}

	fn sub_event(&self) -> &str {
		MessageSubType::Group.into()
	}

	fn contact(&self) -> Contact {
		self.contact.clone().into()
	}

	fn self_id(&self) -> &str {
		&self.self_id
	}

	fn user_id(&self) -> &str {
		&self.user_id
	}

	fn sender(&self) -> Sender {
		self.sender.clone().into()
	}

	fn elements(&self) -> Vec<Elements> {
		self.elements.clone()
	}

	fn message_id(&self) -> &str {
		&self.message_id
	}
}

impl fmt::Display for GroupMessage {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("GroupMessage")
			.field("event_id", &self.event_id)
			.field("self_id", &self.self_id)
			.field("group_id", &self.group_id)
			.field("user_id", &self.user_id)
			.field("message_id", &self.message_id)
			.field("elements", &self.elements)
			.finish()
	}
}

#[cfg(feature = "message")]
#[macro_export]
macro_rules! create_group_message {
	($adapter:expr, $event_id:expr, $contact:expr, $self_id:expr, $user_id:expr, $group_id:expr ,$message_id:expr, $elements:expr, $sender:expr) => {{
		let builder = MessageBuilder {
			event_id: $event_id.into(),
			self_id: $self_id.into(),
			user_id: $user_id.into(),
			message_id: $message_id.into(),
			elements: $elements,
		};
		let message = GroupMessage::new(builder, $group_id.into(), $contact, $sender);
		let event = Event::Message(std::sync::Arc::from($adapter), MessageEvent::Group(message));
		send_event(event);
	}};
}
