use super::types::GroupMemberDecreaseType;
use crate::notion::{NotionBase, NotionBuilder, NotionSubEventType};
use crate::{EventBase, EventType};
use puniyu_bot::Bot;
use puniyu_contact::GroupContact as Contact;
use puniyu_sender::GroupSender as Sender;

/// 群成员减少事件
#[derive(Debug, Clone)]
pub struct GroupMemberDecrease<'n> {
	bot: &'n Bot,
	event_id: &'n str,
	time: u64,
	user_id: &'n str,
	contact: &'n Contact<'n>,
	sender: &'n Sender<'n>,
	content: &'n GroupMemberDecreaseType,
}

impl<'n> GroupMemberDecrease<'n> {
	pub fn new(
		builder: NotionBuilder<'n, Contact<'n>, Sender<'n>, GroupMemberDecreaseType>,
	) -> Self {
		Self {
			bot: builder.bot,
			event_id: builder.event_id,
			time: builder.time,
			user_id: builder.user_id,
			contact: builder.contact,
			sender: builder.sender,
			content: builder.content,
		}
	}
}

impl<'n> EventBase for GroupMemberDecrease<'n> {
	type EventType = EventType;
	type SubEventType = NotionSubEventType;
	type Contact = Contact<'n>;
	type Sender = Sender<'n>;

	fn time(&self) -> u64 {
		self.time
	}

	fn event(&self) -> &EventType {
		&EventType::Notion
	}

	fn event_id(&self) -> &str {
		self.event_id
	}

	fn sub_event(&self) -> &NotionSubEventType {
		&NotionSubEventType::GroupMemberDecrease
	}

	fn bot(&self) -> &Bot {
		self.bot
	}

	fn self_id(&self) -> &str {
		self.bot.account().uin.as_str()
	}

	fn user_id(&self) -> &str {
		self.user_id
	}

	fn contact(&self) -> &Self::Contact {
		self.contact
	}

	fn sender(&self) -> &Self::Sender {
		self.sender
	}
}

impl NotionBase<GroupMemberDecreaseType> for GroupMemberDecrease<'_> {
	fn notion(&self) -> &str {
		"收到群成员减少事件"
	}

	fn content(&self) -> &GroupMemberDecreaseType {
		self.content
	}
}
