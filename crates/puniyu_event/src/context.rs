use crate::message::{FriendMessage, GroupMessage, MessageEvent};
use crate::{Event, EventBase};
use puniyu_adapter_api::AdapterApi;
use puniyu_config::Config;
use puniyu_contact::Contact;
use puniyu_element::Message;
use puniyu_sender::Sender;
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
	event_base: Arc<Event>,
	args: HashMap<String, Option<String>>,
}

impl EventContext {
	pub fn new(message_event: Event, args: HashMap<String, Option<String>>) -> Self {
		Self { event_base: Arc::new(message_event), args }
	}

	/// 消息事件
	pub fn as_message(&self) -> Option<MessageContext> {
		match &*self.event_base {
			Event::Message(ev) => Some(MessageContext::new(ev.clone(), self.args.clone())),
			_ => None,
		}
	}

	/// 事件Id
	pub fn event_id(&self) -> String {
		match &*self.event_base {
			Event::Message(_) => self.as_message().unwrap().event_id(),
			Event::Notion(_) => todo!(),
		}
	}

	/// 事件类型
	pub fn event(&self) -> String {
		match &*self.event_base {
			Event::Message(_) => self.as_message().unwrap().event(),
			Event::Notion(_) => todo!(),
		}
	}

	/// 事件子类型
	pub fn sub_event(&self) -> String {
		match &*self.event_base {
			Event::Message(_) => self.as_message().unwrap().sub_event(),
			Event::Notion(_) => todo!(),
		}
	}

	/// Bot自身Id
	pub fn self_id(&self) -> String {
		match &*self.event_base {
			Event::Message(_) => self.as_message().unwrap().self_id(),
			Event::Notion(_) => todo!(),
		}
	}

	/// 用户Id
	pub fn user_id(&self) -> String {
		match &*self.event_base {
			Event::Message(_) => self.as_message().unwrap().user_id(),
			Event::Notion(_) => todo!(),
		}
	}

	/// 联系人信息
	pub fn contact(&self) -> Contact {
		match &*self.event_base {
			Event::Message(_) => self.as_message().unwrap().contact(),
			Event::Notion(_) => todo!(),
		}
	}

	/// 发送者信息
	pub fn sender(&self) -> Sender {
		match &*self.event_base {
			Event::Message(_) => self.as_message().unwrap().sender(),
			Event::Notion(_) => todo!(),
		}
	}

	/// 判断当前事件是否为Bot主人
	pub fn is_master(&self) -> bool {
		let masters = Config::bot().masters();
		masters.contains(&self.self_id())
	}
}

pub struct MessageContext {
	message_base: Arc<MessageEvent>,
	args: HashMap<String, Option<String>>,
}

impl MessageContext {
	pub fn new(message_event: MessageEvent, args: HashMap<String, Option<String>>) -> Self {
		Self { message_base: Arc::from(message_event), args }
	}

	pub fn as_friend(&self) -> Option<FriendMessage> {
		match &*self.message_base {
			MessageEvent::Friend(ev) => Some(ev.clone()),
			_ => None,
		}
	}
	pub fn as_group(&self) -> Option<GroupMessage> {
		match &*self.message_base {
			MessageEvent::Group(ev) => Some(ev.clone()),
			_ => None,
		}
	}

	pub fn arg(&self, name: &str) -> Option<String> {
		self.args.get(name).cloned().unwrap_or(None)
	}

	/// 事件Id
	pub fn event_id(&self) -> String {
		match &*self.message_base {
			MessageEvent::Friend(ev) => ev.event_id().to_string(),
			MessageEvent::Group(ev) => ev.event_id().to_string(),
		}
	}

	/// 事件类型
	pub fn event(&self) -> String {
		match &*self.message_base {
			MessageEvent::Friend(ev) => ev.event().to_string(),
			MessageEvent::Group(ev) => ev.event().to_string(),
		}
	}

	/// 事件子类型
	pub fn sub_event(&self) -> String {
		match &*self.message_base {
			MessageEvent::Friend(ev) => ev.sub_event().to_string(),
			MessageEvent::Group(ev) => ev.sub_event().to_string(),
		}
	}

	/// Bot自身Id
	pub fn self_id(&self) -> String {
		match &*self.message_base {
			MessageEvent::Friend(ev) => ev.self_id().to_string(),
			MessageEvent::Group(ev) => ev.self_id().to_string(),
		}
	}

	/// 用户Id
	pub fn user_id(&self) -> String {
		match &*self.message_base {
			MessageEvent::Friend(ev) => ev.user_id().to_string(),
			MessageEvent::Group(ev) => ev.user_id().to_string(),
		}
	}

	/// 联系人信息
	pub fn contact(&self) -> Contact {
		match &*self.message_base {
			MessageEvent::Friend(ev) => Contact::from(ev.contact()),
			MessageEvent::Group(ev) => Contact::from(ev.contact()),
		}
	}

	/// 发送者信息
	pub fn sender(&self) -> Sender {
		match &*self.message_base {
			MessageEvent::Friend(ev) => Sender::from(ev.sender()),
			MessageEvent::Group(ev) => Sender::from(ev.sender()),
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
	($event:expr, $args:expr) => {
		EventContext::new($event, $args)
	};
}
