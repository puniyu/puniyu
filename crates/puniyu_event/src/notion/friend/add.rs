use crate::notion::{FriendAddType, NotionBase, NotionBuilder, NotionSubEventType};
use crate::{EventBase, EventType};
use puniyu_bot::Bot;
use puniyu_contact::FriendContact as Contact;
use puniyu_sender::FriendSender as Sender;

/// 好友增加事件
#[derive(Debug, Clone)]
pub struct FriendAdd<'n> {
	bot: &'n Bot,
	event_id: &'n str,
	time: u64,
	user_id: &'n str,
	contact: &'n Contact<'n>,
	sender: &'n Sender<'n>,
	content: &'n FriendAddType,
}

impl<'n> FriendAdd<'n> {
	pub fn new(builder: NotionBuilder<'n, Contact<'n>, Sender<'n>, FriendAddType>) -> Self {
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

impl<'e> EventBase for FriendAdd<'e> {
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
		&NotionSubEventType::FriendAdd
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

impl NotionBase<FriendAddType> for FriendAdd<'_> {
	fn notion(&self) -> &str {
		"收到好友增加事件"
	}

	fn content(&self) -> &FriendAddType {
		self.content
	}
}
