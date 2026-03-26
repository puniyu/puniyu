//! 消息事件模块
//!
//! 提供好友消息和群聊消息的事件类型定义。

mod friend;
#[doc(inline)]
pub use friend::FriendMessage;
mod group;
#[doc(inline)]
pub use group::GroupMessage;
mod event;
#[doc(inline)]
pub use event::MessageEvent;
mod types;
#[doc(inline)]
pub use types::*;


use super::EventBase;
use bytes::Bytes;
use puniyu_element::receive::Elements;
use puniyu_bot::Bot;


/// 消息基础 trait
///
/// 定义所有消息事件的通用接口，提供消息内容访问和判断方法。
///
/// 该 trait 继承自 `EventBase`，因此也可以访问所有事件的基础信息。
///
/// # 示例
///
/// ## 处理消息内容
///
/// ```rust,ignore
/// use puniyu_event::message::MessageBase;
///
/// fn process_message<M: MessageBase>(msg: &M) {
///     // 获取消息 ID
///     let msg_id = msg.message_id();
///     
///     // 获取文本内容
///     let texts = msg.get_text();
///     println!("消息内容: {}", texts.join(" "));
///     
///     // 判断是否为主人
///     if msg.is_master() {
///         println!("主人发送的消息");
///     }
/// }
/// ```
///
/// ## 处理消息元素
///
/// ```rust,ignore
/// use puniyu_event::message::MessageBase;
///
/// fn handle_message_elements<M: MessageBase>(msg: &M) {
///     // 获取图片
///     if let Some(image) = msg.get_image() {
///         println!("收到图片，大小: {} 字节", image.len());
///     }
///     
///     // 获取艾特列表
///     let at_list = msg.get_at();
///     if !at_list.is_empty() {
///         println!("艾特了: {:?}", at_list);
///     }
///     
///     // 获取回复消息 ID
///     if let Some(reply_id) = msg.get_reply_id() {
///         println!("回复了消息: {}", reply_id);
///     }
/// }
/// ```
pub trait MessageBase: Send + Sync + EventBase {
	/// 获取消息 ID
	fn message_id(&self) -> &str;

	/// 获取消息元素列表
	fn elements(&self) -> &Vec<Elements<'_>>;

	/// 获取所有文本元素的内容
	///
	/// # 返回值
	///
	/// 返回消息中所有文本元素的字符串切片向量
	fn get_text(&self) -> Vec<&str> {
		self.elements()
			.iter()
			.filter_map(|e| match e {
				Elements::Text(text) => Some(text.text),
				_ => None,
			})
			.collect::<Vec<&str>>()
	}

	/// 获取所有艾特元素的目标 ID
	///
	/// # 返回值
	///
	/// 返回被艾特的用户 ID 列表
	fn get_at(&self) -> Vec<&str> {
		self.elements()
			.iter()
			.filter_map(|e| match e {
				Elements::At(at) => Some(at.target_id),
				_ => None,
			})
			.collect::<Vec<&str>>()
	}

	/// 获取第一个图片元素
	///
	/// # 返回值
	///
	/// 如果消息包含图片，返回 `Some(Bytes)`，否则返回 `None`
	fn get_image(&self) -> Option<Bytes> {
		self.elements()
			.iter()
			.filter_map(|e| match e {
				Elements::Image(image) => Some(image.file.clone()),
				_ => None,
			})
			.next()
	}

	/// 获取第一个语音元素
	///
	/// # 返回值
	///
	/// 如果消息包含语音，返回 `Some(Bytes)`，否则返回 `None`
	fn get_record(&self) -> Option<Bytes> {
		self.elements()
			.iter()
			.filter_map(|e| match e {
				Elements::Record(record) => Some(record.file.clone()),
				_ => None,
			})
			.next()
	}

	/// 获取回复消息的 ID
	///
	/// # 返回值
	///
	/// 如果消息是回复消息，返回被回复消息的 ID，否则返回 `None`
	fn get_reply_id(&self) -> Option<&str> {
		self.elements()
			.iter()
			.filter_map(|e| match e {
				Elements::Reply(reply) => Some(reply.message_id),
				_ => None,
			})
			.next()
	}
}

/// 消息构建器
///
/// 用于构建消息事件的辅助结构。
#[derive(Debug)]
pub struct MessageBuilder<'m, Contact, Sender>
where
	Contact: puniyu_contact::Contact,
	Sender: puniyu_sender::Sender,
{
	/// 机器人实例
	pub bot: &'m Bot,
	/// 事件 ID
	pub event_id: &'m str,
	/// 用户 ID
	pub user_id: &'m str,
	/// 联系人信息
	pub contact: &'m Contact,
	/// 发送者信息
	pub sender: &'m Sender,
	/// 时间戳
	pub time: u64,
	/// 消息 ID
	pub message_id: &'m str,
	/// 消息元素列表
	pub elements: &'m Vec<Elements<'m>>,
}

/// 生成消息事件结构体及其 EventBase、MessageBase 实现
///
/// # 参数
///
/// - `$name`: 结构体名称
/// - `$contact`: 联系人类型（如 `FriendContact`、`GroupContact`）
/// - `$sender`: 发送者类型（如 `FriendSender`、`GroupSender`）
/// - `$sub_event`: 消息子类型变体（如 `MessageSubEventType::Friend`）
macro_rules! codegen_message {
	(
		$(#[$meta:meta])*
		$name:ident, $contact:ident, $sender:ident, $sub_event:expr
	) => {
		$(#[$meta])*
		#[derive(Debug, Clone)]
		pub struct $name<'m> {
			bot: &'m puniyu_bot::Bot,
			event_id: &'m str,
			time: u64,
			user_id: &'m str,
			message_id: &'m str,
			elements: &'m Vec<puniyu_element::receive::Elements<'m>>,
			contact: &'m $contact<'m>,
			sender: &'m $sender<'m>,
		}

		impl<'m> $name<'m> {
			pub fn new(builder: super::MessageBuilder<'m, $contact<'m>, $sender<'m>>) -> Self {
				Self {
					bot: builder.bot,
					event_id: builder.event_id,
					time: builder.time,
					user_id: builder.user_id,
					message_id: builder.message_id,
					elements: builder.elements,
					contact: builder.contact,
					sender: builder.sender,
				}
			}
		}

		impl<'e> $crate::EventBase for $name<'e> {
			type EventType = $crate::EventType;
			type SubEventType = super::MessageSubEventType;
			type Contact = $contact<'e>;
			type Sender = $sender<'e>;

			fn time(&self) -> u64 { self.time }
			fn event_type(&self) -> &$crate::EventType { &$crate::EventType::Message }
			fn event_id(&self) -> &str { self.event_id }
			fn sub_event(&self) -> &super::MessageSubEventType { &$sub_event }
			fn bot(&self) -> &puniyu_bot::Bot { self.bot }
			fn self_id(&self) -> &str { self.bot.account().uin.as_str() }
			fn user_id(&self) -> &str { self.user_id }
			fn contact(&self) -> &Self::Contact { self.contact }
			fn sender(&self) -> &Self::Sender { self.sender }
		}

		impl<'m> super::MessageBase for $name<'m> {
			fn message_id(&self) -> &str { self.message_id }
			fn elements(&self) -> &Vec<puniyu_element::receive::Elements<'_>> { self.elements }
		}
	};
}
pub(crate) use codegen_message;
