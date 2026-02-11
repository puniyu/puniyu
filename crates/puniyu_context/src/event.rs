mod message;
#[doc(inline)]
pub use message::MessageContext;

use crate::BotContext;
use puniyu_event::Event;
use std::collections::HashMap;

/// 事件上下文
///
/// 提供对事件信息和命令参数的访问。
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_context::{BotContext, EventContext};
///
/// let event_context = EventContext::new(&bot_context, &event, Some(args));
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
pub struct EventContext<'c> {
	event: &'c Event<'c>
}

impl<'c> EventContext<'c> {
	/// 创建新的事件上下文
	///
	/// # 参数
	///
	/// - `bot` - 机器人上下文的引用
	/// - `event` - 事件的引用
	/// - `args` - 可选的命令参数映射
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let event_context = EventContext::new(&bot_context, &event, Some(args));
	/// ```
	pub fn new(
		event: &'c Event,
	) -> Self {
		Self { event }
	}

	/// 判断是否为好友事件
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// if event_context.is_friend() {
	///     println!("这是一个好友事件");
	/// }
	/// ```
	pub fn is_friend(&self) -> bool {
		self.event.is_friend()
	}

	/// 判断是否为群组事件
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// if event_context.is_group() {
	///     println!("这是一个群组事件");
	/// }
	/// ```
	pub fn is_group(&self) -> bool {
		self.event.is_group()
	}

	/// 判断是否为消息事件
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// if event_context.is_message() {
	///     println!("这是一个消息事件");
	/// }
	/// ```
	pub fn is_message(&self) -> bool {
		self.event.is_message()
	}

	/// 尝试转换为消息上下文
	///
	/// 如果是消息事件且有命令参数，则返回 `Some(MessageContext)`，否则返回 `None`。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// if let Some(msg_ctx) = event_context.as_message() {
	///     msg_ctx.reply("Hello!").await?;
	/// }
	/// ```
	pub fn as_message(&self) -> Option<MessageContext<'_>> {
		self.event.as_message().map(|message| MessageContext::new(message, HashMap::new()))
	}

	pub fn as_bot(&self) -> BotContext<'_> {
		BotContext::new(self.event.bot())
	}
}
