use crate::adapter::{AdapterApi, Result, types};
use crate::bot::Bot;
use crate::command::ArgValue;
use crate::contact::ContactType;
use crate::element::{Message, receive::Elements};
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
	args: HashMap<String, ArgValue>,
}

impl MessageContext {
	pub fn new(event: MessageEvent, args: HashMap<String, ArgValue>) -> Self {
		Self { event: Arc::from(event), args }
	}

	pub fn as_friend(&self) -> Option<&FriendMessage> {
		self.event.as_friend()
	}

	pub fn as_group(&self) -> Option<&GroupMessage> {
		self.event.as_group()
	}

	/// 获取参数值
	///
	/// ## 示例
	/// ```rust,ignore
	/// #[command(name = "echo", desc = "回显命令")]
	/// #[arg(name = "message", desc = "要回显的消息")]
	/// #[arg(name = "count", arg_type = "int", default = 1)]
	/// async fn echo(bot: &BotContext, ev: &MessageContext) -> HandlerResult {
	///     let message = ev.arg("message").and_then(|v| v.as_str()).unwrap_or("默认值");
	///     let count = ev.arg("count").and_then(|v| v.as_int()).unwrap_or(1);
	///     HandlerAction::done()
	/// }
	/// ```
	pub fn arg(&self, name: &str) -> Option<&ArgValue> {
		self.args.get(name)
	}

	pub fn event_id(&self) -> &str {
		self.event.event_id()
	}

	pub fn time(&self) -> u64 {
		self.event.time()
	}

	pub fn event(&self) -> &str {
		self.event.event()
	}

	pub fn sub_event(&self) -> &str {
		self.event.sub_event()
	}

	pub fn self_id(&self) -> &str {
		self.event.self_id()
	}

	pub fn user_id(&self) -> &str {
		self.event.user_id()
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

	pub fn elements(&self) -> Vec<Elements> {
		self.event.elements()
	}

	pub fn get_at(&self) -> Option<Vec<String>> {
		self.event.get_at()
	}

	/// 是否为艾特全体成员
	pub fn mentions_everyone(&self) -> bool {
		self.elements().iter().any(|e| matches!(e, Elements::At(at) if !at.is_all()))
	}

	/// 是否为艾特Bot
	pub fn mentions_me(&self) -> bool {
		self.elements()
			.iter()
			.any(|e| matches!(e, Elements::At(at) if at.target_id.contains(self.self_id())))
	}

	pub fn get_image(&self) -> Option<String> {
		self.event.get_image()
	}

	pub fn get_record(&self) -> Option<Vec<u8>> {
		self.event.get_record()
	}

	pub fn get_reply_id(&self) -> Option<String> {
		self.event.get_reply_id()
	}

	pub fn is_master(&self) -> bool {
		self.event.is_master()
	}
}