use crate::bot::Bot;
use crate::contact::GroupContact as Contact;
use crate::element::receive::Elements;
use crate::event::message::{MessageBase, MessageBuilder, MessageSubType};
use crate::event::{EventBase, EventType};
use crate::sender::{GroupSender as Sender, Role};
#[derive(Debug, Clone)]
pub struct GroupMessage<'m> {
	bot: &'m Bot,
	event_id: &'m str,
	time: u64,
	self_id: &'m str,
	user_id: &'m str,
	message_id: &'m str,
	elements: Vec<Elements<'m>>,
	contact: &'m Contact,
	sender: &'m Sender,
}

impl<'m> GroupMessage<'m> {
	pub fn new(builder: MessageBuilder<'m, Contact, Sender>) -> Self {
		Self {
			bot: builder.bot,
			event_id: builder.event_id,
			time: builder.time,
			self_id: builder.self_id,
			user_id: builder.user_id,
			message_id: builder.message_id,
			elements: builder.elements,
			contact: builder.contact,
			sender: builder.sender,
		}
	}
	pub fn group_id(&self) -> &str {
		&self.contact.peer
	}

	pub fn is_admin(&self) -> bool {
		matches!(self.sender.role, Role::Admin)
	}

	pub fn is_owner(&self) -> bool {
		matches!(self.sender.role, Role::Owner)
	}
}

impl<'e> EventBase<EventType, MessageSubType> for GroupMessage<'e> {
	type Contact = Contact;
	type Sender = Sender;

	fn bot(&self) -> &Bot {
		self.bot
	}

	fn time(&self) -> u64 {
		self.time
	}

	fn event(&self) -> &EventType {
		&EventType::Message
	}

	fn event_id(&self) -> &str {
		self.event_id
	}

	fn sub_event(&self) -> &MessageSubType {
		&MessageSubType::Group
	}

	fn self_id(&self) -> &str {
		self.self_id
	}

	fn user_id(&self) -> &str {
		self.user_id
	}

	fn contact(&self) -> &Self::Contact {
		&self.contact
	}

	fn sender(&self) -> &Self::Sender {
		&self.sender
	}
}

impl<'m> MessageBase for GroupMessage<'m> {
	fn message_id(&self) -> &str {
		&self.message_id
	}

	fn elements(&self) -> Vec<Elements> {
		self.elements.clone()
	}
}
