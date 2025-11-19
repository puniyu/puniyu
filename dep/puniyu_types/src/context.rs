use crate::adapter::{AdapterApi, Result, types};
use crate::bot::Bot;
use crate::contact::ContactType;
use crate::element::Message;
use crate::event::EventBase;
use crate::event::message::{FriendMessage, GroupMessage, MessageEvent};
use crate::sender::SenderType;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct BotContext {
	inner: Bot,
	contact: ContactType,
}

impl BotContext {
	pub fn new(bot: Bot, contact: ContactType) -> Self {
		Self { inner: bot, contact }
	}
	pub fn api(&self) -> &dyn AdapterApi {
		self.inner.api
	}
	pub async fn reply(&self, message: Message) -> Result<types::SendMsgType> {
		self.inner.send_msg(self.contact.clone(), message).await
	}
}

#[derive(Debug, Clone)]
pub struct MessageContext {
	event: Arc<MessageEvent>,
	args: HashMap<String, String>,
}

impl MessageContext {
	pub fn new(message_event: MessageEvent, args: HashMap<String, String>) -> Self {
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
		self.args.get(name).cloned()
	}

	/// 事件Id
	pub fn event_id(&self) -> String {
		match &*self.event {
			MessageEvent::Friend(ev) => ev.event_id().to_string(),
			MessageEvent::Group(ev) => ev.event_id().to_string(),
		}
	}

	/// 事件时间
	pub fn time(&self) -> u64 {
		match &*self.event {
			MessageEvent::Friend(ev) => ev.time(),
			MessageEvent::Group(ev) => ev.time(),
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
	pub fn contact(&self) -> ContactType {
		match &*self.event {
			MessageEvent::Friend(ev) => ContactType::Friend(ev.contact().clone()),
			MessageEvent::Group(ev) => ContactType::Group(ev.contact().clone()),
		}
	}

	/// 发送者信息
	pub fn sender(&self) -> SenderType {
		match &*self.event {
			MessageEvent::Friend(ev) => SenderType::Friend(ev.sender().clone()),
			MessageEvent::Group(ev) => SenderType::Group(ev.sender().clone()),
		}
	}
}
#[macro_export]
macro_rules! create_context_bot {
	($bot:expr, $contact:expr) => {
		BotContext::new($bot, $contact)
	};
}

#[macro_export]
macro_rules! create_message_event_context {
	($event:expr, $args:expr) => {
		MessageContext::new($event, $args)
	};
}
