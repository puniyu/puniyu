use puniyu_adapter_builder::AdapterApi;
use puniyu_element::Message;
use puniyu_event_message::MessageBase;
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

pub struct EventContext(pub Arc<dyn MessageBase>);

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
