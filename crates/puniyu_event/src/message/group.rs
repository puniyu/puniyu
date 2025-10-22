use crate::message::{MessageBase, MessageSubType};
use crate::{EventBase, EventType};
use puniyu_contact::GroupContact;
use puniyu_element::Elements;
use puniyu_sender::GroupSender;
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
	pub fn new(message_builder: GroupMessageBuilder) -> Self {
		use std::time::{SystemTime, UNIX_EPOCH};
		let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
		Self {
			event_id: message_builder.event_id,
			time: timestamp,
			self_id: message_builder.self_id,
			user_id: message_builder.user_id,
			group_id: message_builder.group_id,
			message_id: message_builder.message_id,
			elements: message_builder.elements,
			contact: message_builder.contact,
			sender: message_builder.sender,
		}
	}

	pub fn group_id(&self) -> &str {
		&self.group_id
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
			.field("group_id", &self.group_id)
			.field("user_id", &self.user_id)
			.field("message_id", &self.message_id)
			.field("elements", &self.elements)
			.finish()
	}
}

#[derive(Debug, Clone)]
pub struct GroupMessageBuilder {
	pub event_id: String,
	pub self_id: String,
	pub user_id: String,
	pub group_id: String,
	pub contact: GroupContact,
	pub sender: GroupSender,
	pub message_id: String,
	pub elements: Vec<Elements>,
}

#[cfg(feature = "message")]
#[macro_export]
macro_rules! create_group_message {
	($adapter:expr, $event_id:expr, $contact:expr, $self_id:expr, $user_id:expr, $group_id:expr ,$message_id:expr, $elements:expr, $sender:expr) => {{
		let builder = EventBuilder {
			event_id: $event_id.into(),
			self_id: $self_id.into(),
			user_id: $user_id.into(),
			group_id: $group_id.into(),
			contact: $contact,
			sender: $sender,
			message_id: $message_id.into(),
			elements: $elements,
		};
		let message = GroupMessage::new(builder);
		let event = Event::MessageMessageEvent::Group(message);
		send_event(std::sync::Arc::from($adapter), event);
	}};
}
