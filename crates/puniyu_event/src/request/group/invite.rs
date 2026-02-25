use crate::request::group::types::GroupInviteType;
use crate::request::{RequestBase, RequestBuilder, RequestSubEventType};
use crate::{EventBase, EventType};
use puniyu_bot::Bot;
use puniyu_contact::GroupContact as Contact;
use puniyu_sender::GroupSender as Sender;

/// 群组邀请事件
#[derive(Debug, Clone)]
pub struct GroupInvite<'n> {
	bot: &'n Bot,
	event_id: &'n str,
	time: u64,
	user_id: &'n str,
	contact: &'n Contact<'n>,
	sender: &'n Sender<'n>,
	content: &'n GroupInviteType,
}

impl<'n> GroupInvite<'n> {
	pub fn new(builder: RequestBuilder<'n, Contact<'n>, Sender<'n>, GroupInviteType>) -> Self {
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

impl<'n> EventBase for GroupInvite<'n> {
	type EventType = EventType;
	type SubEventType = RequestSubEventType;
	type Contact = Contact<'n>;
	type Sender = Sender<'n>;
	fn time(&self) -> u64 {
		self.time
	}

	fn event(&self) -> &EventType {
		&EventType::Request
	}

	fn event_id(&self) -> &str {
		self.event_id
	}

	fn sub_event(&self) -> &RequestSubEventType {
		&RequestSubEventType::GroupInvite
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

impl<'n> RequestBase for GroupInvite<'n> {
	type Content = &'n GroupInviteType;

	fn request(&self) -> &str {
		"收到群组邀请"
	}

	fn content(&self) -> Self::Content {
		self.content
	}
}
