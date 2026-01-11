mod bot;
use crate::adapter::{types, MessageApi};
use crate::command::ArgValue;
use crate::contact::ContactType;
use crate::element::{receive::Elements, Message};
use crate::event::message::{FriendMessage, GroupMessage, MessageEvent};
use crate::event::EventBase;
use crate::sender::SenderType;
pub use bot::BotContext;
use bytes::Bytes;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct MessageContext {
	bot: BotContext,
	event: Arc<MessageEvent>,
	args: HashMap<String, ArgValue>,
}

impl MessageContext {
	pub fn new(bot: BotContext, event: MessageEvent, args: HashMap<String, ArgValue>) -> Self {
		Self { bot, event: Arc::new(event), args }
	}
	pub fn bot(&self) -> &BotContext {
		&self.bot
	}

	pub async fn reply(&self, message: impl Into<Message>) -> crate::adapter::Result<types::SendMsgType> {
		self.bot.api().send_msg(self.contact().clone(), message.into()).await
	}

	pub fn as_friend(&self) -> Option<&FriendMessage> {
		self.event.as_friend()
	}

	pub fn as_group(&self) -> Option<&GroupMessage> {
		self.event.as_group()
	}

	pub fn is_group(&self) -> bool {
		self.event.is_group()
	}
	pub fn is_friend(&self) -> bool {
		self.event.is_friend()
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
		self.elements().iter().any(|e| matches!(e, Elements::At(at) if at.is_everyone()))
	}

	/// 是否为艾特Bot
	pub fn mentions_bot(&self) -> bool {
		self.elements()
			.iter()
			.any(|e| matches!(e, Elements::At(at) if at.target_id.contains(self.self_id())))
	}

	pub fn get_image(&self) -> Option<Bytes> {
		self.event.get_image()
	}

	pub fn get_record(&self) -> Option<Bytes> {
		self.event.get_record()
	}

	pub fn get_reply_id(&self) -> Option<String> {
		self.event.get_reply_id()
	}

	pub fn is_master(&self) -> bool {
		self.event.is_master()
	}
}
