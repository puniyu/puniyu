use puniyu_bot::Bot;
use crate::notion::{NotionBase, NotionBuilder, NotionSubEvent, PrivatePokeOption};
use crate::{EventBase, EventType};
use puniyu_contact::FriendContact as Contact;
use puniyu_sender::FriendSender as Sender;

/// 私聊戳一戳事件
#[derive(Debug, Clone)]
pub struct PrivatePoke<'n> {
	bot: &'n Bot,
	event_id: &'n str,
	time: u64,
	self_id: &'n str,
	user_id: &'n str,
	contact: &'n Contact<'n>,
	sender: &'n Sender<'n>,
	content: &'n PrivatePokeOption,
}

impl<'n> PrivatePoke<'n> {
	pub fn new(
		builder: NotionBuilder<'n, Contact<'n>, Sender<'n>, PrivatePokeOption>,
	) -> Self {
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

impl<'n> EventBase for PrivatePoke<'n> {
	type EventType = EventType;
	type SubEventType = NotionSubEvent;
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

	fn sub_event(&self) -> &NotionSubEvent {
		&NotionSubEvent::PrivatePoke
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

impl NotionBase for PrivatePoke<'_> {
	type Content = PrivatePokeOption;

	fn notion(&self) -> &str {
		"收到好友戳一戳事件"
	}

	fn content(&self) -> &Self::Content {
		self.content
	}
}
