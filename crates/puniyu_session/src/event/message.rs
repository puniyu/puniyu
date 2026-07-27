use std::ops::Deref;
use std::sync::Arc;

use crate::BotSession;
use puniyu_contact::ContactType;
use puniyu_element::receive::Elements;
use puniyu_event::message::{
	FriendMessage, GroupMessage, GroupTempMessage, GuildMessage, MessageEvent,
};

/// 消息会话

#[derive(Clone)]
pub struct MessageSession {
	inner: Arc<Inner>,
}

struct Inner {
	event: MessageEvent,
	bot: BotSession,
}

impl MessageSession {
	/// 创建新的消息会话
	pub fn new(event: &MessageEvent) -> Self {
		let bot = BotSession::new(event.bot());
		Self {
			inner: Arc::new(Inner {
				event: event.clone(),
				bot,
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
