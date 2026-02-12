use crate::bot::BotContext;
use bytes::Bytes;
use puniyu_adapter_core::types::SendMsgType;
use puniyu_command_core::ArgValue;
use puniyu_contact::ContactType;
use puniyu_element::receive::Elements;
use puniyu_error::Result;
use puniyu_event::message::{FriendMessage, GroupMessage, MessageEvent, MessageSubType};
use puniyu_event::{EventBase, EventType};
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
	event: &'c MessageEvent<'c>,
	args: HashMap<&'c str, ArgValue>,
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
	pub fn new(
		event: &'c MessageEvent,
		args: HashMap<&'c str, ArgValue>,
	) -> Self {
		Self { event, args }
	}

	/// 获取机器人上下文
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let bot_ctx = msg_context.bot();
	/// let api = bot_ctx.api();
	/// ```
	pub fn bot(&self) -> BotContext<'_> {
		BotContext::new(self.event.bot())
	}

	/// 回复消息
	///
	/// 向消息来源（好友或群组）发送回复消息。
	///
	/// # 参数
	///
	/// - `message` - 要发送的消息，可以是任何实现了 `Into<Message>` 的类型
	///
	/// # 返回
	///
	/// 返回发送结果，包含消息 ID 等信息。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// // 发送文本消息
	/// ctx.reply("Hello!").await?;
	///
	/// // 发送复杂消息
	/// let message = Message::builder()
	///     .text("Hello")
	///     .build();
	/// ctx.reply(message).await?;
	/// ```
	pub async fn reply<M>(&self, message: M) -> Result<SendMsgType>
	where
		M: Into<Message<'c>>,
	{
		self.bot().api().message().send_msg(&self.contact(), &message.into()).await
	}

	/// 尝试转换为好友消息
	///
	/// 如果是好友消息则返回 `Some`，否则返回 `None`。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// if let Some(friend_msg) = ctx.as_friend() {
	///     println!("好友消息: {}", friend_msg.user_id());
	/// }
	/// ```
	pub fn as_friend(&self) -> Option<&FriendMessage<'_>> {
		self.event.as_friend()
	}

	/// 尝试转换为群组消息
	///
	/// 如果是群组消息则返回 `Some`，否则返回 `None`。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// if let Some(group_msg) = ctx.as_group() {
	///     println!("群组消息: {}", group_msg.group_id());
	/// }
	/// ```
	pub fn as_group(&self) -> Option<&GroupMessage<'_>> {
		self.event.as_group()
	}

	/// 判断是否为群组消息
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// if ctx.is_group() {
	///     println!("这是一条群组消息");
	/// }
	/// ```
	pub fn is_group(&self) -> bool {
		self.event.is_group()
	}

	/// 判断是否为好友消息
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// if ctx.is_friend() {
	///     println!("这是一条好友消息");
	/// }
	/// ```
	pub fn is_friend(&self) -> bool {
		self.event.is_friend()
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
	/// 如果参数存在则返回 `Some(&ArgValue)`，否则返回 `None`。
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

	/// 获取事件 ID
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let event_id = ctx.event_id();
	/// println!("Event ID: {}", event_id);
	/// ```
	pub fn event_id(&self) -> &str {
		self.event.event_id()
	}

	/// 获取事件时间戳
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let timestamp = ctx.time();
	/// println!("Timestamp: {}", timestamp);
	/// ```
	pub fn time(&self) -> u64 {
		self.event.time()
	}

	/// 获取事件类型
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let event_type = ctx.event();
	/// println!("Event type: {:?}", event_type);
	/// ```
	pub fn event(&self) -> &EventType {
		self.event.event()
	}

	/// 获取消息子类型
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let sub_type = ctx.sub_event();
	/// println!("Sub type: {:?}", sub_type);
	/// ```
	pub fn sub_event(&self) -> &MessageSubType {
		self.event.sub_event()
	}

	/// 获取机器人自身 ID
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let self_id = ctx.self_id();
	/// println!("Bot ID: {}", self_id);
	/// ```
	pub fn self_id(&self) -> &str {
		self.event.self_id()
	}

	/// 获取发送者用户 ID
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let user_id = ctx.user_id();
	/// println!("User ID: {}", user_id);
	/// ```
	pub fn user_id(&self) -> &str {
		self.event.user_id()
	}

	/// 获取联系人信息
	///
	/// 返回消息来源的联系人信息（好友或群组）。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let contact = ctx.contact();
	/// match contact {
	///     ContactType::Friend(friend) => println!("好友: {}", friend.user_id()),
	///     ContactType::Group(group) => println!("群组: {}", group.group_id()),
	/// }
	/// ```
	pub fn contact(&self) -> ContactType<'_> {
		match self.event {
			MessageEvent::Friend(ev) => ContactType::Friend(ev.contact().clone()),
			MessageEvent::Group(ev) => ContactType::Group(ev.contact().clone()),
		}
	}

	/// 获取发送者信息
	///
	/// 返回消息发送者的详细信息。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let sender = ctx.sender();
	/// println!("发送者 ID: {}", sender.user_id());
	/// if let Some(name) = sender.name() {
	///     println!("发送者名称: {}", name);
	/// }
	/// ```
	pub fn sender(&self) -> SenderType<'_> {
		match self.event {
			MessageEvent::Friend(ev) => SenderType::Friend(ev.sender().clone()),
			MessageEvent::Group(ev) => SenderType::Group(ev.sender().clone()),
		}
	}

	/// 获取消息元素列表
	///
	/// 返回消息中的所有元素（文本、图片、艾特等）。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// for element in ctx.elements() {
	///     match element {
	///         Elements::Text(text) => println!("文本: {}", text.text),
	///         Elements::Image(img) => println!("图片: {}", img.file),
	///         Elements::At(at) => println!("艾特: {}", at.target_id),
	///         _ => {}
	///     }
	/// }
	/// ```
	pub fn elements(&self) -> &Vec<Elements<'_>> {
		self.event.elements()
	}

	/// 获取所有被艾特的用户 ID
	///
	/// 返回消息中所有被艾特的用户 ID 列表。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let at_users = ctx.get_at();
	/// for user_id in at_users {
	///     println!("艾特了: {}", user_id);
	/// }
	/// ```
	pub fn get_at(&self) -> Vec<&str> {
		self.event.get_at()
	}

	/// 判断是否艾特了全体成员
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// if ctx.mentions_everyone() {
	///     println!("艾特了全体成员");
	/// }
	/// ```
	pub fn mentions_everyone(&self) -> bool {
		self.elements().iter().any(|e| matches!(e, Elements::At(at) if at.is_everyone()))
	}

	/// 判断是否艾特了机器人
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// if ctx.mentions_bot() {
	///     println!("艾特了机器人");
	/// }
	/// ```
	pub fn mentions_bot(&self) -> bool {
		self.elements()
			.iter()
			.any(|e| matches!(e, Elements::At(at) if at.target_id.contains(self.self_id())))
	}

	/// 获取消息中的图片数据
	///
	/// 如果消息包含图片，返回第一张图片的数据。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// if let Some(image_data) = ctx.get_image() {
	///     println!("收到图片，大小: {} 字节", image_data.len());
	/// }
	/// ```
	pub fn get_image(&self) -> Option<Bytes> {
		self.event.get_image()
	}

	/// 获取消息中的语音数据
	///
	/// 如果消息包含语音，返回语音数据。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// if let Some(record_data) = ctx.get_record() {
	///     println!("收到语音，大小: {} 字节", record_data.len());
	/// }
	/// ```
	pub fn get_record(&self) -> Option<Bytes> {
		self.event.get_record()
	}

	/// 获取回复的消息 ID
	///
	/// 如果消息是回复其他消息，返回被回复消息的 ID。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// if let Some(reply_id) = ctx.get_reply_id() {
	///     println!("回复了消息: {}", reply_id);
	/// }
	/// ```
	pub fn get_reply_id(&self) -> Option<&str> {
		self.event.get_reply_id()
	}

	/// 判断发送者是否为主人
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// if ctx.is_master() {
	///     println!("主人发送的消息");
	/// }
	/// ```
	pub fn is_master(&self) -> bool {
		self.event.is_master()
	}
}
