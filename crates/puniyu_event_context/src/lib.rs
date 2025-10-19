use puniyu_adapter_builder::AdapterApi;
use puniyu_element::Message;
use puniyu_event_message::{FriendMessage, GroupMessage, MessageBase, MessageEvent};
use puniyu_event_utils::contact::Contact;
use std::sync::Arc;

#[derive(Clone)]
pub struct Bot {
	contact: Contact,
	api: Arc<dyn AdapterApi>,
}

impl Bot {
	pub fn new(contact: Contact, api: Arc<dyn AdapterApi>) -> Self {
		Self { contact, api }
	}
	pub fn api(&self) -> &dyn AdapterApi {
		&*self.api
	}
	pub fn reply(&self, message: Message) {
		self.api.send_msg(self.contact.clone(), message)
	}
}

pub struct EventContext {
	message_event: Arc<MessageEvent>,
}

impl EventContext {
	pub fn new(message_event: MessageEvent) -> Self {
		Self { message_event: Arc::new(message_event) }
	}
	pub fn as_friend(&self) -> Option<FriendMessage> {
		match &*self.message_event {
			MessageEvent::Friend(ev) => Some(ev.clone()),
			_ => None,
		}
	}

	pub fn as_group(&self) -> Option<GroupMessage> {
		match &*self.message_event {
			MessageEvent::Group(ev) => Some(ev.clone()),
			_ => None,
		}
	}

	pub fn inner(&self) -> &dyn MessageBase {
		match &*self.message_event {
			MessageEvent::Friend(ev) => ev,
			MessageEvent::Group(ev) => ev,
		}
	}
}

#[macro_export]
macro_rules! create_context_bot {
	($contact:expr, $api:expr) => {
		Bot::new($contact, $api)
	};
}

#[macro_export]
macro_rules! create_event_context {
	($event:expr) => {
		EventContext::new($event)
	};
}
