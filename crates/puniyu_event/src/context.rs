use crate::message::{FriendMessage, GroupMessage, MessageBase, MessageEvent};
use puniyu_adapter_api::AdapterApi;
use puniyu_contact::Contact;
use puniyu_element::Message;
use std::collections::HashMap;
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
	args: HashMap<String, Option<String>>,
}

impl EventContext {
	pub fn new(message_event: MessageEvent, args: HashMap<String, Option<String>>) -> Self {
		Self { message_event: Arc::new(message_event), args }
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

	pub fn arg(&self, name: &str) -> Option<String> {
		self.args.get(name).cloned().unwrap_or(None)
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
	($event:expr, $args:expr) => {
		EventContext::new($event, $args)
	};
}
