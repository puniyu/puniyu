use crate::bot::Bot;
use crate::contact::GroupContact as Contact;
use crate::event::request::group::types::GroupApplyType;
use crate::event::request::{RequestBase, RequestBuilder, RequestSubEvent};
use crate::event::{EventBase, EventType};
use crate::sender::GroupSender as Sender;

/// 群组加入申请事件
#[derive(Debug, Clone)]
pub struct GroupApply<'n> {
	bot: &'n Bot,
	event_id: &'n str,
	time: u64,
	self_id: &'n str,
	user_id: &'n str,
	contact: &'n Contact,
	sender: &'n Sender,
	content: GroupApplyType,
}

impl<'n> GroupApply<'n> {
	pub fn new(
		request_builder: RequestBuilder<'n, Contact, Sender>,
		content: GroupApplyType,
	) -> Self {
		Self {
			bot: request_builder.bot,
			event_id: request_builder.event_id,
			time: request_builder.time,
			self_id: request_builder.self_id,
			user_id: request_builder.user_id,
			contact: request_builder.contact,
			sender: request_builder.sender,
			content,
		}
	}
}

impl EventBase<EventType, RequestSubEvent> for GroupApply<'_> {
	type Contact = Contact;
	type Sender = Sender;

	fn bot(&self) -> &Bot {
		&self.bot
	}

	fn time(&self) -> u64 {
		self.time
	}

	fn event(&self) -> &EventType {
		&EventType::Request
	}

	fn event_id(&self) -> &str {
		&self.event_id
	}

	fn sub_event(&self) -> &RequestSubEvent {
		&RequestSubEvent::GroupApply
	}

	fn self_id(&self) -> &str {
		&self.self_id
	}

	fn user_id(&self) -> &str {
		&self.user_id
	}

	fn contact(&self) -> &Self::Contact {
		&self.contact
	}

	fn sender(&self) -> &Self::Sender {
		&self.sender
	}
}

impl RequestBase for GroupApply<'_> {
	type Content = GroupApplyType;

	fn notion(&self) -> &str {
		"收到群组加入申请"
	}

	fn content(&self) -> &Self::Content {
		&self.content
	}
}
