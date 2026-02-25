use super::types::GroupRecallType;
use crate::notion::{NotionBase, NotionBuilder, NotionSubEventType};
use crate::{EventBase, EventType};
use puniyu_bot::Bot;
use puniyu_contact::GroupContact as Contact;
use puniyu_sender::GroupSender as Sender;

/// 群聊撤回事件
#[derive(Debug, Clone)]
pub struct GroupRecall<'n> {
	bot: &'n Bot,
	event_id: &'n str,
	time: u64,
	user_id: &'n str,
	contact: &'n Contact<'n>,
	sender: &'n Sender<'n>,
	content: &'n GroupRecallType,
}

impl<'n> GroupRecall<'n> {
	pub fn new(builder: NotionBuilder<'n, Contact<'n>, Sender<'n>, GroupRecallType>) -> Self {
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

impl<'n> EventBase for GroupRecall<'n> {
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
		&NotionSubEventType::GroupRecall
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

impl NotionBase<GroupRecallType> for GroupRecall<'_> {
	fn notion(&self) -> &str {
		"收到群聊撤回事件"
	}

	fn content(&self) -> &GroupRecallType {
		self.content
	}
}
