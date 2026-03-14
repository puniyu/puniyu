use crate::BotContext;
use bytes::Bytes;
use puniyu_adapter_core::types::message::SendMsgType;
use puniyu_bot::Bot;
use puniyu_command_core::ArgValue;
use puniyu_config::Config;
use puniyu_contact::ContactType;
use puniyu_element::receive::Elements;
use puniyu_event::EventType;
use puniyu_event::message::{FriendMessage, GroupMessage, MessageEvent, MessageSubEventType};
use puniyu_message::Message;
use puniyu_sender::SenderType;
use std::collections::HashMap;

/// 消息上下文
///
/// 提供对消息事件的专门处理，包括消息回复、参数获取、发送者信息等。
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_context::MessageContext;
///
/// async fn handle_message(ctx: &MessageContext) {
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
	inner: &'c MessageEvent<'c>,
	bot: BotContext<'c>,
	args: HashMap<String, ArgValue>,
}

impl<'c> MessageContext<'c> {
	/// 创建新的消息上下文
	///
	/// # 参数
	///
	/// - `bot` - 机器人上下文的引用
	/// - `event` - 消息事件的引用
	/// - `args` - 命令参数映射的引用
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let msg_context = MessageContext::new(&bot_context, &message_event, &args);
	/// ```
	pub fn new(event: &'c MessageEvent, args: HashMap<String, ArgValue>) -> Self {
		Self { inner: event, bot: BotContext::new(event.bot()), args }
	}

	/// 获取内部消息事件
	pub fn inner(&self) -> &MessageEvent<'c> {
		self.inner
	}

	pub fn is_friend(&self) -> bool {
		matches!(self.inner, MessageEvent::Friend(_))
	}

	pub fn is_group(&self) -> bool {
		matches!(self.inner, MessageEvent::Group(_))
	}

	pub fn as_bot(&self) -> &BotContext<'_> {
		&self.bot
	}
	pub fn as_friend(&self) -> Option<&FriendMessage<'_>> {
		self.inner.as_friend()
	}

	pub fn as_group(&self) -> Option<&GroupMessage<'_>> {
		self.inner.as_group()
	}

	pub async fn reply<M>(&self, message: M) -> puniyu_error::Result<SendMsgType>
	where
		M: Into<Message<'c>>,
	{
		self.bot.api().message().send_msg(&self.contact(), &message.into()).await
	}

	/// 获取命令参数值
	///
	/// 根据参数名获取对应的参数值。
	///
	/// # 参数
	///
	/// - `name` - 参数名称
	///
	/// # 返回
	///
	/// 如果参数存在则返回 [`Some(&ArgValue)`]，否则返回 [`None`]。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// // 获取字符串参数
	/// if let Some(name) = ctx.arg("name").and_then(|v| v.as_str()) {
	///     println!("Name: {}", name);
	/// }
	///
	/// // 获取整数参数
	/// if let Some(count) = ctx.arg("count").and_then(|v| v.as_int()) {
	///     println!("Count: {}", count);
	/// }
	/// ```
	pub fn arg(&self, name: &str) -> Option<&ArgValue> {
		self.args.get(name)
	}

	/// 判断用户是否为Bot Master
	pub fn is_master(&self) -> bool {
		let config = Config::app();
		let masters = config.masters();
		masters.contains(&self.user_id().to_string())
	}
}

impl<'c> MessageContext<'c> {
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
	pub fn event(&self) -> &EventType {
		self.inner.event()
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
	pub fn sub_event(&self) -> &MessageSubEventType {
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

impl<'c> MessageContext<'c> {
	pub fn message_id(&self) -> &str {
		self.inner.message_id()
	}

	pub fn elements(&self) -> &Vec<Elements<'_>> {
		self.inner.elements()
	}

	pub fn get_at(&self) -> Vec<&str> {
		self.elements()
			.iter()
			.filter_map(|e| match e {
				Elements::At(at) => Some(at.target_id),
				_ => None,
			})
			.collect()
	}

	pub fn mentions_bot(&self) -> bool {
		self.get_at().contains(&self.self_id())
	}

	pub fn mentions_everyone(&self) -> bool {
		self.elements().iter().any(|e| matches!(e, Elements::At(at) if at.is_everyone()))
	}

	pub fn get_image(&self) -> Vec<&Bytes> {
		self.elements()
			.iter()
			.filter_map(|e| match e {
				Elements::Image(image) => Some(&image.file),
				_ => None,
			})
			.collect()
	}

	pub fn get_reply_id(&self) -> Option<&str> {
		self.elements().iter().find_map(|e| match e {
			Elements::Reply(reply) => Some(reply.message_id),
			_ => None,
		})
	}
}
