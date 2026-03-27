mod message;
#[doc(inline)]
pub use message::MessageContext;

use crate::BotContext;
use puniyu_bot::Bot;
use puniyu_command_types::ArgValue;
use puniyu_contact::ContactType;
use puniyu_event::{Event, EventType, SubEventType};
use puniyu_sender::SenderType;
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
	inner: &'c Event<'c>,
	bot: BotContext<'c>,
	args: Option<HashMap<String, ArgValue>>,
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
	pub fn new(event: &'c Event, args: Option<HashMap<String, ArgValue>>) -> Self {
		Self { inner: event, bot: BotContext::new(event.bot()), args }
	}

	pub fn as_bot(&self) -> &BotContext<'_> {
		&self.bot
	}

	pub fn as_message(&self) -> Option<MessageContext<'_>> {
		self.inner
			.as_message()
			.and_then(|message| self.args.clone().map(|args| MessageContext::new(message, args)))
	}
}

impl<'c> EventContext<'c> {
	/// 获取消息触发时间戳
	///
	/// # 返回值
	///
	/// 返回 Unix 时间戳（秒）
	pub fn time(&self) -> u64 {
		self.inner.time()
	}

	/// 获取事件类型
	///
	/// # 返回值
	///
	/// 返回 `EventType::Message`
	pub fn event_type(&self) -> &EventType {
		self.inner.event_type()
	}

	/// 获取事件 ID
	///
	/// # 返回值
	///
	/// 返回事件的唯一标识符
	pub fn event_id(&self) -> &str {
		self.inner.event_id()
	}

	/// 获取消息子类型
	///
	/// # 返回值
	///
	/// 返回消息子类型（Friend 或 Group）
	pub fn sub_event(&self) -> SubEventType {
		self.inner.sub_event()
	}

	/// 获取机器人实例
	///
	/// # 返回值
	///
	/// 返回处理该消息的机器人实例引用
	pub fn bot(&self) -> &Bot {
		self.inner.bot()
	}

	/// 获取机器人 ID
	///
	/// # 返回值
	///
	/// 返回机器人的唯一标识符
	pub fn self_id(&self) -> &str {
		self.inner.self_id()
	}

	/// 获取用户 ID
	///
	/// # 返回值
	///
	/// 返回发送消息的用户 ID
	pub fn user_id(&self) -> &str {
		self.inner.user_id()
	}

	/// 获取联系人信息
	///
	/// # 返回值
	///
	/// 返回消息发生的联系人信息（好友或群聊）
	pub fn contact(&self) -> ContactType<'_> {
		self.inner.contact()
	}

	/// 获取发送者信息
	///
	/// # 返回值
	///
	/// 返回发送消息的用户详细信息
	pub fn sender(&self) -> SenderType<'_> {
		self.inner.sender()
	}
}
