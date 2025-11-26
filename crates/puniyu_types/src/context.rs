use crate::adapter::{AdapterApi, Result, types};
use crate::bot::Bot;
use crate::contact::ContactType;
use crate::element::{Message, receive::Elements};
use crate::event::EventBase;
use crate::event::message::{FriendMessage, GroupMessage, MessageBase, MessageEvent};
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
	pub fn new(event: MessageEvent, args: HashMap<String, String>) -> Self {
		Self { event: Arc::from(event), args }
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
	///
	/// ## 示例
	/// ```rust,ignore
	/// #[command(
	/// name = "echo",
	/// args = ["name"],
	/// rank = 50,
	/// )]
	/// async fn test(bot: &BotContext, ev: &MessageContext) -> HandlerResult {
	///     let name = ev.arg("name").unwrap();
	///     Ok(HandlerAction::Done)
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

	/// 消息元素
	pub fn elements(&self) -> Vec<Elements> {
		match &*self.event {
			MessageEvent::Friend(ev) => ev.elements(),
			MessageEvent::Group(ev) => ev.elements(),
		}
	}
	/// 获取艾特用户
	///
	pub fn get_at(&self) -> Option<String> {
		if let Some(at_list) = self.get_at_list() { at_list.first().cloned() } else { None }
	}

	/// 获取艾特列表
	pub fn get_at_list(&self) -> Option<Vec<String>> {
		match &*self.event {
			MessageEvent::Friend(ev) => ev.get_at(),
			MessageEvent::Group(ev) => ev.get_at(),
		}
	}

	/// 是否为艾特全体成员
	pub fn mentions_everyone(&self) -> bool {
		self.elements().iter().any(|e| matches!(e, Elements::At(at) if !at.is_all()))
	}

	/// 是否为艾特Bot
	pub fn mentions_me(&self) -> bool {
		self.elements()
			.iter()
			.any(|e| matches!(e, Elements::At(at) if at.target_id.contains(&self.self_id())))
	}

	/// 获取图片
	pub fn get_image(&self) -> Option<Vec<u8>> {
		match &*self.event {
			MessageEvent::Friend(ev) => ev.get_image(),
			MessageEvent::Group(ev) => ev.get_image(),
		}
	}

	/// 获取语音元素
	pub fn get_record(&self) -> Option<Vec<u8>> {
		match &*self.event {
			MessageEvent::Friend(ev) => ev.get_record(),
			MessageEvent::Group(ev) => ev.get_record(),
		}
	}

	/// 获取回复id
	pub fn get_reply_id(&self) -> Option<String> {
		match &*self.event {
			MessageEvent::Friend(ev) => ev.get_reply_id(),
			MessageEvent::Group(ev) => ev.get_reply_id(),
		}
	}
}

#[cfg(feature = "context")]
#[macro_export]
macro_rules! create_context_bot {
	($bot:expr, $contact:expr) => {
		BotContext::new($bot, $contact)
	};
	($adapter:expr, $adapter_api:expr, $account:expr, $contact:expr) => {
		let bot = Bot {
			adapter: $adapter,
			adapter_api: $adapter_api,
			account: $account,
		}
		BotContext::new(bot, $contact)
	};
}

#[cfg(feature = "context")]
#[macro_export]
macro_rules! create_message_event_context {
	($event:expr, $args:expr) => {
		MessageContext::new($event, $args)
	};
}
