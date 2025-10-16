use puniyu_adapter::AdapterApi;
use puniyu_element::Message;
use puniyu_event_message::MessageBase;
use puniyu_event_utils::contact::Contact;
use std::sync::Arc;

pub struct Bot {
	pub contact: Contact,
	pub(crate) api: Box<dyn AdapterApi>,
}

impl Bot {
	pub fn api(&self) -> &dyn AdapterApi {
		&*self.api
	}
	pub fn reply(&self, message: Message) {
		self.api.send_msg(self.contact.clone(), message)
	}
}

pub struct EventContext(Vec<Arc<dyn MessageBase>>);
