use super::types::GroupHonorChangeType;
use crate::bot::Bot;
use crate::contact::GroupContact as Contact;
use crate::event::notion::{NotionBase, NotionBuilder, NotionSubEvent};
use crate::event::{EventBase, EventType};
use crate::sender::GroupSender as Sender;

/// 群聊荣誉变更事件
#[derive(Debug, Clone)]
pub struct GroupHonorChange<'n> {
	bot: &'n Bot,
	event_id: &'n str,
	time: u64,
	self_id: &'n str,
	user_id: &'n str,
	contact: &'n Contact,
	sender: &'n Sender,
	content: GroupHonorChangeType,
}

impl<'n> GroupHonorChange<'n> {
	pub fn new(
		notion_builder: NotionBuilder<'n, Contact, Sender>,
		content: GroupHonorChangeType,
	) -> Self {
		Self {
			bot: notion_builder.bot,
			event_id: notion_builder.event_id,
			time: notion_builder.time,
			self_id: notion_builder.self_id,
			user_id: notion_builder.user_id,
			contact: notion_builder.contact,
			sender: notion_builder.sender,
			content,
		}
	}
}

impl EventBase<EventType, NotionSubEvent> for GroupHonorChange<'_> {
	type Contact = Contact;
	type Sender = Sender;

	fn bot(&self) -> &Bot {
		self.bot
	}

	fn time(&self) -> u64 {
		self.time
	}

	fn event(&self) -> &EventType {
		&EventType::Notion
	}

	fn event_id(&self) -> &str {
		self.event_id
	}

	fn sub_event(&self) -> &NotionSubEvent {
		&NotionSubEvent::GroupHonorChange
	}

	fn self_id(&self) -> &str {
		self.self_id
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

impl NotionBase for GroupHonorChange<'_> {
	type Content = GroupHonorChangeType;

	fn notion(&self) -> &str {
		"收到群聊荣誉变更事件"
	}

	fn content(&self) -> &Self::Content {
		&self.content
	}
}
