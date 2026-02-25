use super::types::GroupFileUploadType;
use crate::notion::{NotionBase, NotionBuilder, NotionSubEventType};
use crate::{EventBase, EventType};
use puniyu_bot::Bot;
use puniyu_contact::GroupContact as Contact;
use puniyu_sender::GroupSender as Sender;

/// 群文件上传事件
#[derive(Debug, Clone)]
pub struct GroupFileUpload<'n> {
	bot: &'n Bot,
	event_id: &'n str,
	time: u64,
	user_id: &'n str,
	contact: &'n Contact<'n>,
	sender: &'n Sender<'n>,
	content: &'n GroupFileUploadType,
}

impl<'n> GroupFileUpload<'n> {
	pub fn new(builder: NotionBuilder<'n, Contact<'n>, Sender<'n>, GroupFileUploadType>) -> Self {
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

impl<'n> EventBase for GroupFileUpload<'n> {
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
		&NotionSubEventType::GroupFileUpload
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

impl NotionBase<GroupFileUploadType> for GroupFileUpload<'_> {
	fn notion(&self) -> &str {
		"收到群聊文件上传事件"
	}

	fn content(&self) -> &GroupFileUploadType {
		self.content
	}
}
