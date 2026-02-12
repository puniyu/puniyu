use super::friend::FriendMessage;
use super::group::GroupMessage;
use super::{MessageBase, MessageSubType};
use crate::{EventBase, EventType};
use bytes::Bytes;
use puniyu_bot::Bot;
use puniyu_contact::ContactType;
use puniyu_element::receive::Elements;
use puniyu_sender::SenderType;

/// 消息事件枚举
///
/// 统一的消息事件类型，可以是好友消息或群聊消息。
///
/// # 变体
///
/// - `Friend` - 好友消息事件
/// - `Group` - 群聊消息事件
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_event::message::MessageEvent;
///
/// fn handle_message(event: MessageEvent) {
///     match event {
///         MessageEvent::Friend(msg) => {
///             println!("收到好友消息: {:?}", msg.get_text());
///         }
///         MessageEvent::Group(msg) => {
///             println!("收到群消息: {:?}", msg.get_text());
///         }
///     }
/// }
/// ```
#[derive(Debug, Clone)]
pub enum MessageEvent<'m> {
	/// 好友消息事件
	Friend(FriendMessage<'m>),
	/// 群聊消息事件
	Group(GroupMessage<'m>),
}

impl<'m> MessageEvent<'m> {
	/// 获取机器人实例
	///
	/// # 返回值
	///
	/// 返回机器人实例的引用
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let bot = event.bot();
	/// println!("机器人 ID: {}", bot.account().uin);
	/// ```
	pub fn bot(&self) -> &Bot {
		match self {
			Self::Friend(message) => message.bot(),
			Self::Group(message) => message.bot(),
		}
	}

	/// 判断是否为好友消息
	///
	/// # 返回值
	///
	/// 如果是好友消息返回 `true`，否则返回 `false`。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// if event.is_friend() {
	///     println!("这是好友消息");
	/// }
	/// ```
	pub fn is_friend(&self) -> bool {
		match self {
			MessageEvent::Friend(message) => message.is_friend(),
			MessageEvent::Group(message) => message.is_friend(),
		}
	}

	/// 判断是否为群聊消息
	///
	/// # 返回值
	///
	/// 如果是群聊消息返回 `true`，否则返回 `false`。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// if event.is_group() {
	///     println!("这是群聊消息");
	/// }
	/// ```
	pub fn is_group(&self) -> bool {
		match self {
			MessageEvent::Friend(message) => message.is_group(),
			MessageEvent::Group(message) => message.is_group(),
		}
	}

	/// 尝试获取好友消息的引用
	///
	/// # 返回值
	///
	/// 如果是好友消息返回 `Some(&FriendMessage)`，否则返回 `None`。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// if let Some(friend_msg) = event.as_friend() {
	///     println!("好友消息: {:?}", friend_msg.get_text());
	/// }
	/// ```
	pub fn as_friend(&self) -> Option<&FriendMessage<'_>> {
		match self {
			MessageEvent::Friend(msg) => Some(msg),
			_ => None,
		}
	}

	/// 尝试获取群聊消息的引用
	///
	/// # 返回值
	///
	/// 如果是群聊消息返回 `Some(&GroupMessage)`，否则返回 `None`。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// if let Some(group_msg) = event.as_group() {
	///     println!("群聊消息: {:?}", group_msg.get_text());
	/// }
	/// ```
	pub fn as_group(&self) -> Option<&GroupMessage<'_>> {
		match self {
			MessageEvent::Group(msg) => Some(msg),
			_ => None,
		}
	}

	/// 获取消息时间戳
	///
	/// # 返回值
	///
	/// 返回消息的时间戳（秒）。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let timestamp = event.time();
	/// println!("消息时间: {}", timestamp);
	/// ```
	pub fn time(&self) -> u64 {
		match self {
			MessageEvent::Friend(msg) => msg.time(),
			MessageEvent::Group(msg) => msg.time(),
		}
	}

	/// 获取事件类型
	///
	/// # 返回值
	///
	/// 返回事件类型的引用。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let event_type = event.event();
	/// println!("事件类型: {:?}", event_type);
	/// ```
	pub fn event(&self) -> &EventType {
		match self {
			MessageEvent::Friend(msg) => msg.event(),
			MessageEvent::Group(msg) => msg.event(),
		}
	}

	/// 获取事件 ID
	///
	/// # 返回值
	///
	/// 返回事件的唯一标识符。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let event_id = event.event_id();
	/// println!("事件 ID: {}", event_id);
	/// ```
	pub fn event_id(&self) -> &str {
		match self {
			MessageEvent::Friend(msg) => msg.event_id(),
			MessageEvent::Group(msg) => msg.event_id(),
		}
	}

	/// 获取消息子类型
	///
	/// # 返回值
	///
	/// 返回消息的子类型（普通消息、匿名消息等）。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let sub_event = event.sub_event();
	/// println!("消息子类型: {:?}", sub_event);
	/// ```
	pub fn sub_event(&self) -> &MessageSubType {
		match self {
			MessageEvent::Friend(msg) => msg.sub_event(),
			MessageEvent::Group(msg) => msg.sub_event(),
		}
	}

	/// 获取机器人自身 ID
	///
	/// # 返回值
	///
	/// 返回接收消息的机器人 ID。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let self_id = event.self_id();
	/// println!("机器人 ID: {}", self_id);
	/// ```
	pub fn self_id(&self) -> &str {
		match self {
			MessageEvent::Friend(msg) => msg.self_id(),
			MessageEvent::Group(msg) => msg.self_id(),
		}
	}

	/// 获取发送者用户 ID
	///
	/// # 返回值
	///
	/// 返回消息发送者的用户 ID。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let user_id = event.user_id();
	/// println!("发送者 ID: {}", user_id);
	/// ```
	pub fn user_id(&self) -> &str {
		match self {
			MessageEvent::Friend(msg) => msg.user_id(),
			MessageEvent::Group(msg) => msg.user_id(),
		}
	}

	/// 获取消息 ID
	///
	/// # 返回值
	///
	/// 返回消息的唯一标识符。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let message_id = event.message_id();
	/// println!("消息 ID: {}", message_id);
	/// ```
	pub fn message_id(&self) -> &str {
		match self {
			MessageEvent::Friend(msg) => msg.message_id(),
			MessageEvent::Group(msg) => msg.message_id(),
		}
	}

	/// 获取联系人信息
	///
	/// # 返回值
	///
	/// 返回消息的联系人信息（好友或群组）。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let contact = event.contact();
	/// println!("联系人类型: {:?}", contact);
	/// ```
	pub fn contact(&self) -> ContactType<'_> {
		match self {
			MessageEvent::Friend(msg) => ContactType::Friend(msg.contact().clone()),
			MessageEvent::Group(msg) => ContactType::Group(msg.contact().clone()),
		}
	}

	/// 获取发送者信息
	///
	/// # 返回值
	///
	/// 返回消息发送者的详细信息。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let sender = event.sender();
	/// println!("发送者昵称: {:?}", sender.name());
	/// ```
	pub fn sender(&self) -> SenderType<'_> {
		match self {
			MessageEvent::Friend(msg) => SenderType::Friend(msg.sender().clone()),
			MessageEvent::Group(msg) => SenderType::Group(msg.sender().clone()),
		}
	}

	/// 获取消息元素列表
	///
	/// # 返回值
	///
	/// 返回消息包含的所有元素（文本、图片、@等）。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let elements = event.elements();
	/// println!("消息元素数量: {}", elements.len());
	/// ```
	pub fn elements(&self) -> &Vec<Elements<'_>> {
		match self {
			MessageEvent::Friend(msg) => msg.elements(),
			MessageEvent::Group(msg) => msg.elements(),
		}
	}

	/// 获取消息中的所有文本内容
	///
	/// # 返回值
	///
	/// 返回消息中所有文本元素的内容列表。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let texts = event.get_text();
	/// for text in texts {
	///     println!("文本: {}", text);
	/// }
	/// ```
	pub fn get_text(&self) -> Vec<&str> {
		match self {
			MessageEvent::Friend(msg) => msg.get_text(),
			MessageEvent::Group(msg) => msg.get_text(),
		}
	}

	/// 获取消息中的所有 @ 对象
	///
	/// # 返回值
	///
	/// 返回消息中所有被 @ 的用户 ID 列表。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let at_list = event.get_at();
	/// for user_id in at_list {
	///     println!("@了用户: {}", user_id);
	/// }
	/// ```
	pub fn get_at(&self) -> Vec<&str> {
		match self {
			MessageEvent::Friend(msg) => msg.get_at(),
			MessageEvent::Group(msg) => msg.get_at(),
		}
	}

	/// 获取消息中的图片数据
	///
	/// # 返回值
	///
	/// 如果消息包含图片返回 `Some(Bytes)`，否则返回 `None`。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// if let Some(image) = event.get_image() {
	///     println!("收到图片，大小: {} 字节", image.len());
	/// }
	/// ```
	pub fn get_image(&self) -> Option<Bytes> {
		match self {
			MessageEvent::Friend(msg) => msg.get_image(),
			MessageEvent::Group(msg) => msg.get_image(),
		}
	}

	/// 获取消息中的语音数据
	///
	/// # 返回值
	///
	/// 如果消息包含语音返回 `Some(Bytes)`，否则返回 `None`。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// if let Some(record) = event.get_record() {
	///     println!("收到语音，大小: {} 字节", record.len());
	/// }
	/// ```
	pub fn get_record(&self) -> Option<Bytes> {
		match self {
			MessageEvent::Friend(msg) => msg.get_record(),
			MessageEvent::Group(msg) => msg.get_record(),
		}
	}

	/// 获取回复的消息 ID
	///
	/// # 返回值
	///
	/// 如果消息是回复消息返回被回复消息的 ID，否则返回 `None`。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// if let Some(reply_id) = event.get_reply_id() {
	///     println!("回复了消息: {}", reply_id);
	/// }
	/// ```
	pub fn get_reply_id(&self) -> Option<&str> {
		match self {
			MessageEvent::Friend(msg) => msg.get_reply_id(),
			MessageEvent::Group(msg) => msg.get_reply_id(),
		}
	}

	/// 判断发送者是否为主人
	///
	/// # 返回值
	///
	/// 如果发送者是配置的主人返回 `true`，否则返回 `false`。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// if event.is_master() {
	///     println!("主人发送的消息");
	/// }
	/// ```
	pub fn is_master(&self) -> bool {
		match self {
			MessageEvent::Friend(msg) => msg.is_master(),
			MessageEvent::Group(msg) => msg.is_master(),
		}
	}
}
