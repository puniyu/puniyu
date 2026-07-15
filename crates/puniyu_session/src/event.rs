mod message;
#[doc(inline)]
pub use message::MessageSession;

use crate::BotSession;
use puniyu_event::Event;
use std::{collections::HashMap, ops::Deref};

/// 事件会话
///
/// 提供对事件信息和命令参数的访问。
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_session::{BotSession, EventSession};
///
/// let bot_context = BotSession::new(&bot);
/// let event_context = EventSession::new(&event, args);
///
/// // 判断事件类型
/// if event_context.is_message() {
///     if let Some(msg_ctx) = event_context.as_message() {
///         msg_ctx.reply("收到消息").await?;
///     }
/// }
///
/// // 判断场景类型
/// if event_context.is_friend() {
///     println!("好友事件");
/// } else if event_context.is_group() {
///     println!("群组事件");
/// }
/// ```
#[derive(Clone)]
pub struct EventSession<'c> {
	inner: &'c Event<'c>,
}

impl<'c> EventSession<'c> {
	/// 创建新的事件会话
	///
	/// # 参数
	///
	/// - `event` - 事件的引用
	/// - `args` - 可选的命令参数映射
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let event_context = EventSession::new(&event, args);
	/// ```
	pub fn new(event: &'c Event) -> Self {
		Self { inner: event }
	}

	/// 获取当前事件关联的机器人会话。
	pub fn as_bot(&self) -> BotSession<'_> {
		BotSession::new(self.inner.bot())
	}

	/// 尝试将当前事件会话转换为消息会话。
	///
	/// 当事件为消息事件时返回 [`Some`]，否则返回 [`None`]。
	pub fn as_message(&self) -> Option<MessageSession<'_>> {
		self.inner.as_message().map(|message| MessageSession::new(message, HashMap::new()))
	}
}
impl<'e> Deref for EventSession<'e> {
	type Target = Event<'e>;

	fn deref(&self) -> &Self::Target {
		self.inner
	}
}

impl<'e> From<&'e Event<'e>> for EventSession<'e> {
	#[inline]
	fn from(event: &'e Event) -> Self {
		Self::new(event)
	}
}

impl<'e> From<&'e EventSession<'e>> for &'e Event<'e> {
	#[inline]
	fn from(context: &'e EventSession<'e>) -> Self {
		context.inner
	}
}
