use crate::bot::Bot;
use crate::contact::GroupContact as Contact;
use crate::event::request::group::types::GroupInviteType;
use crate::event::request::{RequestBase, RequestBuilder, RequestSubEvent};
use crate::event::{EventBase, EventType};
use crate::sender::GroupSender as Sender;

/// 群组邀请事件
#[derive(Debug, Clone)]
pub struct GroupInvite<'n> {
	bot: &'n Bot,
	event_id: &'n str,
	time: u64,
	self_id: &'n str,
	user_id: &'n str,
	contact: &'n Contact,
	sender: &'n Sender,
	content: GroupInviteType,
}

impl<'n> GroupInvite<'n> {
	pub fn new(
		request_builder: RequestBuilder<'n, Contact, Sender>,
		content: GroupInviteType,
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

impl EventBase<EventType, RequestSubEvent> for GroupInvite<'_> {
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
		&RequestSubEvent::GroupInvite
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

impl RequestBase for GroupInvite<'_> {
	type Content = GroupInviteType;

	fn notion(&self) -> &str {
		"收到群组邀请"
	}

	fn content(&self) -> &Self::Content {
		&self.content
	}
}
