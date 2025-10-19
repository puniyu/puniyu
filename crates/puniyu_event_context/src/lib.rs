use puniyu_adapter_builder::AdapterApi;
use puniyu_element::Message;
use puniyu_event_message::{FriendMessage, GroupMessage, MessageBase, MessageEvent};
use puniyu_event_utils::contact::Contact;
use std::sync::Arc;

#[derive(Clone)]
pub struct Bot {
	pub contact: Contact,
	pub api: Arc<dyn AdapterApi>,
}

impl Bot {
	pub fn api(&self) -> &dyn AdapterApi {
		&*self.api
	}
	pub fn reply(&self, message: Message) {
		self.api.send_msg(self.contact.clone(), message)
	}
}

pub struct EventContext(pub Arc<MessageEvent>);

impl EventContext {
	pub fn as_friend(&self) -> Option<FriendMessage> {
		match &*self.0 {
			MessageEvent::Friend(ev) => Some(ev.clone()),
			_ => None,
		}
	}

	pub fn as_group(&self) -> Option<GroupMessage> {
		match &*self.0 {
			MessageEvent::Group(ev) => Some(ev.clone()),
			_ => None,
		}
	}

	pub fn inner(&self) -> &dyn MessageBase {
		match &*self.0 {
			MessageEvent::Friend(ev) => ev,
			MessageEvent::Group(ev) => ev,
		}
	}
}

#[macro_export]
macro_rules! create_context_bot {
	($contact:expr, $api:expr) => {
		Bot { contact: $contact, api: std::sync::Arc::from($api) }
	};
}

#[macro_export]
macro_rules! create_event_context {
	($event:expr) => {
		EventContext(std::sync::Arc::from($event))
	};
}
