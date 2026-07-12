use std::ops::Deref;

use crate::{BotContext, EventArg};
use puniyu_adapter_types::{SendMessageOptions, SendMsgResult};
use puniyu_command_types::FromArgValue;
use puniyu_contact::ContactType;
use puniyu_element::receive::Elements;
use puniyu_event::message::{
	FriendMessage, GroupMessage, GroupTempMessage, GuildMessage, MessageEvent,
};
use puniyu_message::Message;
use smol_str::SmolStr;

/// 消息上下文
///
/// 提供对消息事件的专门处理，包括消息回复、参数获取、发送者信息等。
///
/// 消息类型判断方法（如 `is_friend()`、`is_group()`、`is_group_temp()`）
/// 由 [`MessageBase`] trait 默认提供。
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_context::MessageContext;
///
/// async fn handle_message(ctx: &MessageContext<'_>) {
///     // 回复消息
///     ctx.reply("Hello!").await?;
///
///     // 获取命令参数
///     if let Some(name) = ctx.arg("name").and_then(|v| v.as_str()) {
///         ctx.reply(format!("Hello, {}!", name)).await?;
///     }
///
///     // 判断消息类型
///     if ctx.is_group() {
///         println!("群消息");
///     }
///
///     // 获取发送者信息
///     let sender = ctx.sender();
///     println!("发送者: {}", sender.user_id());
/// }
/// ```
#[derive(Clone)]
pub struct MessageContext<'c> {
	event: &'c MessageEvent<'c>,
	bot: BotContext<'c>,
	args: EventArg,
}

impl<'c> MessageContext<'c> {
	/// 创建新的消息上下文
	///
	/// # 参数
	///
	/// - `event` - 消息事件的引用
	/// - `args` - 命令参数映射
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let msg_context = MessageContext::new(&message_event, args);
	/// ```
	pub fn new(event: &'c MessageEvent, args: EventArg) -> Self {
		Self { event, bot: BotContext::new(event.bot()), args }
	}

	/// 获取内部消息事件
	pub fn event(&self) -> &'c MessageEvent<'c> {
		self.event
	}

	/// 获取当前消息关联的机器人上下文。
	pub fn as_bot(&self) -> &BotContext<'_> {
		&self.bot
	}

	/// 获取好友消息引用。
	///
	/// 如需仅做消息类型判断，优先使用 [`MessageBase`] 提供的
	/// `is_friend()` / `is_group()` / `is_group_temp()`。
	///
	/// 如果当前消息为好友消息则返回 [`Some`]，否则返回 [`None`]。
	pub fn as_friend(&self) -> Option<&FriendMessage<'_>> {
		self.event.as_friend()
	}

	/// 获取群消息引用。
	///
	/// 如果当前消息为群消息则返回 [`Some`]，否则返回 [`None`]。
	pub fn as_group(&self) -> Option<&GroupMessage<'_>> {
		self.event.as_group()
	}

	/// 获取群临时消息引用。
	///
	/// 如果当前消息为群临时消息则返回 [`Some`]，否则返回 [`None`]。
	pub fn as_group_temp(&self) -> Option<&GroupTempMessage<'_>> {
		self.event.as_group_temp()
	}

	/// 获取频道消息引用。
	///
	/// 如果当前消息为频道消息则返回 [`Some`]，否则返回 [`None`]。
	pub fn as_guild(&self) -> Option<&GuildMessage<'_>> {
		self.event.as_guild()
	}

	/// 向当前消息对应的联系人发送回复消息。
	///
	/// 参数 `message` 支持任意可转换为 [`Message`] 的类型
	#[inline]
	pub async fn reply<M>(
		&self,
		message: M,
		options: Option<&SendMessageOptions>,
	) -> puniyu_error::AnyError<SendMsgResult>
	where
		M: Into<Message>,
	{
		let contact = self.event.contact();
		self.bot.send_message(&contact, &message.into(), options).await
	}

	/// 获取参数的第一个值并转换为目标类型
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// if let Some(name) = ctx.arg::<String>("name") { ... }
	/// if let Some(count) = ctx.arg::<i64>("count") { ... }
	/// ```
	pub fn arg<T: FromArgValue>(&self, name: impl Into<SmolStr>) -> Option<T> {
		self.args.get(&name.into()).and_then(|v| v.first()).and_then(FromArgValue::from_arg_value)
	}

	/// 获取参数的所有值并转换为目标类型列表
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// if let Some(names) = ctx.arg_list::<String>("names") {
	///     for name in &names { ... }
	/// }
	/// ```
	pub fn arg_list<T: FromArgValue>(&self, name: impl Into<SmolStr>) -> Option<Vec<T>> {
		self.args
			.get(&name.into())?
			.iter()
			.map(FromArgValue::from_arg_value)
			.collect()
	}

	/// TODO: 完善
	/// 判断当前消息发送者是否为 Bot Master。
	pub fn is_master(&self) -> bool {
		false
	}
}

impl MessageContext<'_> {
	pub fn is_friend(&self) -> bool {
		matches!(self.event.contact(), ContactType::Friend(_))
	}

	pub fn is_group(&self) -> bool {
		matches!(self.event.contact(), ContactType::Group(_))
	}

	pub fn is_group_temp(&self) -> bool {
		matches!(self.event.contact(), ContactType::GroupTemp(_))
	}

	pub fn is_guild(&self) -> bool {
		matches!(self.event.contact(), ContactType::Guild(_))
	}
}

impl<'c> MessageContext<'c> {
	/// 判断消息内容是否艾特了当前机器人。
	pub fn mentions_bot(&self) -> bool {
		self.get_at().contains(&self.self_id())
	}

	/// 判断消息内容是否包含 `@全体成员`。
	pub fn mentions_everyone(&self) -> bool {
		self.elements().iter().any(|e| matches!(e, Elements::At(at) if at.is_everyone()))
	}
}

impl<'e> Deref for MessageContext<'e> {
	type Target = MessageEvent<'e>;
	fn deref(&self) -> &Self::Target {
		self.event
	}
}