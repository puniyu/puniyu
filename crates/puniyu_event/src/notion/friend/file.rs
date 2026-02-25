use crate::notion::{NotionBase, NotionBuilder, NotionSubEventType, PrivateFileUploadType};
use crate::{EventBase, EventType};
use puniyu_bot::Bot;
use puniyu_contact::FriendContact as Contact;
use puniyu_sender::FriendSender as Sender;

/// 好友文件上传事件
#[derive(Debug, Clone)]
pub struct PrivateFileUpload<'n> {
	bot: &'n Bot,
	event_id: &'n str,
	time: u64,
	user_id: &'n str,
	contact: &'n Contact<'n>,
	sender: &'n Sender<'n>,
	content: &'n PrivateFileUploadType,
}

impl<'n> PrivateFileUpload<'n> {
	pub fn new(builder: NotionBuilder<'n, Contact<'n>, Sender<'n>, PrivateFileUploadType>) -> Self {
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

impl<'e> EventBase for PrivateFileUpload<'e> {
	type EventType = EventType;
	type SubEventType = NotionSubEventType;
	type Contact = Contact<'e>;
	type Sender = Sender<'e>;
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
		&NotionSubEventType::PrivateFileUpload
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

impl NotionBase<PrivateFileUploadType> for PrivateFileUpload<'_> {
	fn notion(&self) -> &str {
		"收到好友撤回事件"
	}
	fn content(&self) -> &PrivateFileUploadType {
		self.content
	}
}
