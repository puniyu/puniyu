use crate::request::friend::types::PrivateApplyType;
use crate::request::{RequestBase, RequestBuilder, RequestSubEventType};
use crate::{EventBase, EventType};
use puniyu_bot::Bot;
use puniyu_contact::FriendContact as Contact;
use puniyu_sender::FriendSender as Sender;

#[derive(Debug, Clone)]
pub struct PrivateApply<'n> {
	bot: &'n Bot,
	event_id: &'n str,
	time: u64,
	user_id: &'n str,
	contact: &'n Contact<'n>,
	sender: &'n Sender<'n>,
	content: &'n PrivateApplyType,
}

impl<'n> PrivateApply<'n> {
	pub fn new(builder: RequestBuilder<'n, Contact<'n>, Sender<'n>, PrivateApplyType>) -> Self {
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

impl<'n> EventBase for PrivateApply<'n> {
	type EventType = EventType;
	type SubEventType = RequestSubEventType;
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

	fn sub_event(&self) -> &RequestSubEventType {
		&RequestSubEventType::PrivateApply
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

impl<'n> RequestBase for PrivateApply<'n> {
	type Content = &'n PrivateApplyType;

	fn request(&self) -> &str {
		"收到好友申请请求"
	}

	fn content(&self) -> Self::Content {
		self.content
	}
}
