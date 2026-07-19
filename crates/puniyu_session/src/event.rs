mod message;
#[doc(inline)]
pub use message::MessageSession;

use crate::BotSession;
use puniyu_event::Event;
use std::{collections::HashMap, ops::Deref};

/// 事件会话
#[derive(Clone)]
pub struct EventSession<'c> {
	inner: &'c Event,
}

impl<'c> EventSession<'c> {
	pub fn new(event: &'c Event) -> Self {
		Self { inner: event }
	}

	/// 获取当前事件关联的机器人会话。
	pub fn as_bot(&self) -> BotSession {
		BotSession::new(self.inner.bot())
	}

	/// 尝试将当前事件会话转换为消息会话。
	pub fn as_message(&self) -> Option<MessageSession> {
		self.inner
			.as_message()
			.map(|message| MessageSession::new(message, HashMap::new()))
	}
}

impl<'c> Deref for EventSession<'c> {
	type Target = Event;

	fn deref(&self) -> &Self::Target {
		self.inner
	}
}

impl<'c> From<&'c Event> for EventSession<'c> {
	#[inline]
	fn from(event: &'c Event) -> Self {
		Self::new(event)
	}
}

impl<'c> From<&'c EventSession<'c>> for &'c Event {
	#[inline]
	fn from(context: &'c EventSession<'c>) -> Self {
		context.inner
	}
}
