use puniyu_bot::Bot;
use crate::notion::{NotionBase, NotionBuilder, NotionSubEvent};
use crate::{EventBase, EventType};
use puniyu_contact::FriendContact as Contact;
use puniyu_sender::FriendSender as Sender;

/// 好友减少事件
#[derive(Debug, Clone)]
pub struct FriendDecrease<'n> {
	bot: &'n Bot,
	event_id: &'n str,
	time: u64,
	self_id: &'n str,
	user_id: &'n str,
	contact: &'n Contact<'n>,
	sender: &'n Sender<'n>,
	content: &'n (),
}

impl<'n> FriendDecrease<'n> {
	pub fn new(builder: NotionBuilder<'n, Contact<'n>, Sender<'n>, ()>) -> Self {
		Self {
			bot: builder.bot,
			event_id: builder.event_id,
			time: builder.time,
			self_id: builder.self_id,
			user_id: builder.user_id,
			contact: builder.contact,
			sender: builder.sender,
			content: builder.content,
		}
	}
}

impl<'e> EventBase for FriendDecrease<'e> {
	type EventType = EventType;
	type SubEventType = NotionSubEvent;
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

	fn sub_event(&self) -> &NotionSubEvent {
		&NotionSubEvent::FriendDecrease
	}

	fn bot(&self) -> &Bot {
		self.bot
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

impl NotionBase for FriendDecrease<'_> {
	type Content = ();

	fn notion(&self) -> &str {
		"收到好友减少事件"
	}

	fn content(&self) -> &Self::Content {
		self.content
	}
}
