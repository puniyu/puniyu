mod message;
#[doc(inline)]
pub use message::MessageSession;

use crate::BotSession;
use puniyu_event::Event;
use std::{ops::Deref, sync::Arc};

struct Inner {
	event: Event,
	bot: BotSession,
}
/// 事件会话
#[derive(Clone)]
pub struct EventSession {
	inner: Arc<Inner>,
}

impl EventSession {
	pub fn new(event: Event) -> Self {
		let bot = BotSession::new(event.bot());
		Self {
			inner: Arc::new(Inner {
				event,
				bot,
			}),
		}
	}

	/// 获取当前事件关联的机器人会话。
	pub fn as_bot(&self) -> &BotSession {
		&self.inner.bot
	}

	/// 尝试将当前事件会话转换为消息会话。
	pub fn as_message(&self) -> Option<MessageSession> {
		self.inner.event.as_message().map(MessageSession::new)
	}
}

impl Deref for EventSession {
	type Target = Event;

	fn deref(&self) -> &Self::Target {
		&self.inner.event
	}
}
