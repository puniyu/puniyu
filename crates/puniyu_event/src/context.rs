use crate::EventBase;
use crate::message::{FriendMessage, GroupMessage, MessageEvent};
use puniyu_adapter_api::{AdapterApi, types};
use puniyu_contact::Contact;
use puniyu_element::Message;
use puniyu_sender::Sender;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct BotContext {
	contact: Contact,
	api: Arc<dyn AdapterApi>,
}

impl BotContext {
	pub fn new(contact: Contact, api: Arc<dyn AdapterApi>) -> Self {
		Self { contact, api }
	}
	pub fn api(&self) -> &dyn AdapterApi {
		&*self.api
	}
	pub async fn reply(
		&self,
		message: Message,
	) -> Result<types::SendMsgType, Box<dyn std::error::Error>> {
		self.api.send_msg(self.contact.clone(), message).await
	}
}

#[derive(Debug, Clone)]
pub struct MessageContext {
	event: Arc<MessageEvent>,
	args: HashMap<String, Option<String>>,
}

impl MessageContext {
	pub fn new(message_event: MessageEvent, args: HashMap<String, Option<String>>) -> Self {
		Self { event: Arc::from(message_event), args }
	}

	pub fn as_friend(&self) -> Option<FriendMessage> {
		match &*self.event {
			MessageEvent::Friend(ev) => Some(ev.clone()),
			_ => None,
		}
	}
	pub fn as_group(&self) -> Option<GroupMessage> {
		match &*self.event {
			MessageEvent::Group(ev) => Some(ev.clone()),
			_ => None,
		}
	}

	/// 从上下文中获取参数值
	/// ## 实例
	/// ```rust,ignore
	/// #[command(
	/// name = "echo",
	/// args = ["name"],
	/// rank = 50,
	/// )]
	/// async fn test(bot: &Bot, ev: &EventContext) -> HandlerResult {
	///     let name = ev.arg("name").unwrap();
	///     HandlerResult::Ok
	/// }
	/// ```
	pub fn arg(&self, name: &str) -> Option<String> {
		self.args.get(name).cloned().unwrap_or(None)
	}

	/// 事件Id
	pub fn event_id(&self) -> String {
		match &*self.event {
			MessageEvent::Friend(ev) => ev.event_id().to_string(),
			MessageEvent::Group(ev) => ev.event_id().to_string(),
		}
	}

	/// 事件类型
	pub fn event(&self) -> String {
		match &*self.event {
			MessageEvent::Friend(ev) => ev.event().to_string(),
			MessageEvent::Group(ev) => ev.event().to_string(),
		}
	}

	/// 事件子类型
	pub fn sub_event(&self) -> String {
		match &*self.event {
			MessageEvent::Friend(ev) => ev.sub_event().to_string(),
			MessageEvent::Group(ev) => ev.sub_event().to_string(),
		}
	}

	/// Bot自身Id
	pub fn self_id(&self) -> String {
		match &*self.event {
			MessageEvent::Friend(ev) => ev.self_id().to_string(),
			MessageEvent::Group(ev) => ev.self_id().to_string(),
		}
	}

	/// 用户Id
	pub fn user_id(&self) -> String {
		match &*self.event {
			MessageEvent::Friend(ev) => ev.user_id().to_string(),
			MessageEvent::Group(ev) => ev.user_id().to_string(),
		}
	}

	/// 联系人信息
	pub fn contact(&self) -> Contact {
		match &*self.event {
			MessageEvent::Friend(ev) => Contact::from(ev.contact()),
			MessageEvent::Group(ev) => Contact::from(ev.contact()),
		}
	}

	/// 发送者信息
	pub fn sender(&self) -> Sender {
		match &*self.event {
			MessageEvent::Friend(ev) => Sender::from(ev.sender()),
			MessageEvent::Group(ev) => Sender::from(ev.sender()),
		}
	}
}
#[macro_export]
macro_rules! create_context_bot {
	($contact:expr, $api:expr) => {
		BotContext::new($contact, $api)
	};
}

#[macro_export]
macro_rules! create_message_event_context {
	($event:expr, $args:expr) => {
		MessageContext::new($event, $args)
	};
}
