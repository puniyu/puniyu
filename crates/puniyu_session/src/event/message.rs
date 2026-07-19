use std::ops::Deref;
use std::sync::Arc;

use crate::{BotSession, EventArg};
use puniyu_adapter_types::{SendMessageOptions, SendMsgResult};
use puniyu_command_types::FromArgValue;
use puniyu_contact::ContactType;
use puniyu_element::receive::Elements;
use puniyu_event::message::{
	FriendMessage, GroupMessage, GroupTempMessage, GuildMessage, MessageEvent,
};
use puniyu_message::Message;
use smol_str::SmolStr;

/// 消息会话
///
/// 拥有数据，可跨 `.await` 使用，可存入异步任务。
#[derive(Clone)]
pub struct MessageSession {
	inner: Arc<MessageSessionInner>,
}

struct MessageSessionInner {
	event: MessageEvent,
	bot: BotSession,
	args: EventArg,
	is_master: bool,
}

impl MessageSession {
	/// 创建新的消息会话
	pub fn new(event: &MessageEvent, args: EventArg) -> Self {
		let bot = BotSession::new(event.bot());
		let is_master = {
			let adapter_name = bot.adapter_info().name;
			puniyu_config::app::AppConfig::get()
				.master()
				.get(adapter_name.as_str())
				.iter()
				.any(|master| master.as_str() == event.user_id())
		};
		Self {
			inner: Arc::new(MessageSessionInner {
				event: event.clone(),
				bot,
				args,
				is_master,
			}),
		}
	}

	/// 获取内部消息事件
	pub fn event(&self) -> &MessageEvent {
		&self.inner.event
	}

	/// 获取当前消息关联的机器人会话。
	pub fn as_bot(&self) -> &BotSession {
		&self.inner.bot
	}

	/// 获取好友消息引用。
	pub fn as_friend(&self) -> Option<&FriendMessage> {
		self.inner.event.as_friend()
	}

	/// 获取群消息引用。
	pub fn as_group(&self) -> Option<&GroupMessage> {
		self.inner.event.as_group()
	}

	/// 获取群临时消息引用。
	pub fn as_group_temp(&self) -> Option<&GroupTempMessage> {
		self.inner.event.as_group_temp()
	}

	/// 获取频道消息引用。
	pub fn as_guild(&self) -> Option<&GuildMessage> {
		self.inner.event.as_guild()
	}

	/// 向当前消息对应的联系人发送回复消息。
	#[inline]
	pub async fn reply<M>(
		&self,
		message: M,
		options: Option<&SendMessageOptions>,
	) -> puniyu_error::AnyError<SendMsgResult>
	where
		M: Into<Message>,
	{
		let contact = self.inner.event.contact();
		self.inner
			.bot
			.send_message(&contact, &message.into(), options)
			.await
	}

	/// 获取参数的第一个值并转换为目标类型
	pub fn arg<T: FromArgValue>(&self, name: impl Into<SmolStr>) -> Option<T> {
		self.inner
			.args
			.get(&name.into())
			.and_then(|v| v.first())
			.and_then(FromArgValue::from_arg_value)
	}

	/// 获取参数的所有值并转换为目标类型列表
	pub fn arg_list<T: FromArgValue>(&self, name: impl Into<SmolStr>) -> Option<Vec<T>> {
		self.inner
			.args
			.get(&name.into())?
			.iter()
			.map(FromArgValue::from_arg_value)
			.collect()
	}

	/// 判断当前消息发送者是否为 Bot Master。
	pub fn is_master(&self) -> bool {
		self.inner.is_master
	}

	pub fn is_friend(&self) -> bool {
		matches!(self.inner.event.contact(), ContactType::Friend(_))
	}

	pub fn is_group(&self) -> bool {
		matches!(self.inner.event.contact(), ContactType::Group(_))
	}

	pub fn is_group_temp(&self) -> bool {
		matches!(self.inner.event.contact(), ContactType::GroupTemp(_))
	}

	pub fn is_guild(&self) -> bool {
		matches!(self.inner.event.contact(), ContactType::Guild(_))
	}

	/// 判断消息内容是否艾特了当前机器人。
	pub fn mentions_bot(&self) -> bool {
		self.get_at().contains(&self.self_id())
	}

	/// 判断消息内容是否包含 `@全体成员`。
	pub fn mentions_everyone(&self) -> bool {
		self.elements()
			.iter()
			.any(|e| matches!(e, Elements::At(at) if at.is_everyone()))
	}
}

impl Deref for MessageSession {
	type Target = MessageEvent;
	fn deref(&self) -> &Self::Target {
		&self.inner.event
	}
}
