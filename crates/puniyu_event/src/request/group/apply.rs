use crate::request::group::types::GroupApplyType;
use crate::request::{RequestBase, RequestBuilder, RequestSubEventType};
use crate::{EventBase, EventType};
use puniyu_bot::Bot;
use puniyu_contact::GroupContact as Contact;
use puniyu_sender::GroupSender as Sender;

/// 群组加入申请事件
#[derive(Debug, Clone)]
pub struct GroupApply<'n> {
	bot: &'n Bot,
	event_id: &'n str,
	time: u64,
	user_id: &'n str,
	contact: &'n Contact<'n>,
	sender: &'n Sender<'n>,
	content: &'n GroupApplyType,
}

impl<'n> GroupApply<'n> {
	pub fn new(builder: RequestBuilder<'n, Contact<'n>, Sender<'n>, GroupApplyType>) -> Self {
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

impl<'n> EventBase for GroupApply<'n> {
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
		&RequestSubEventType::GroupApply
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

impl<'n> RequestBase for GroupApply<'n> {
	type Content = &'n GroupApplyType;

	fn request(&self) -> &str {
		"收到群组加入申请"
	}

	fn content(&self) -> Self::Content {
		self.content
	}
}
