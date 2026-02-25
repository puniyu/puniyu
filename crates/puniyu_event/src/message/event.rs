use super::{MessageBase, MessageSubEventType};
use super::friend::FriendMessage;
use super::group::GroupMessage;
use crate::{EventBase, EventType};
use puniyu_bot::Bot;
use puniyu_contact::ContactType;
use puniyu_element::receive::Elements;
use puniyu_sender::SenderType;

/// 消息事件枚举
///
/// 统一的消息事件类型，包含好友消息和群消息。
///
/// # 变体
///
/// - `Friend(FriendMessage)` - 好友消息事件
/// - `Group(GroupMessage)` - 群消息事件
///
/// # 示例
///
/// ## 模式匹配处理消息
///
/// ```rust,ignore
/// use puniyu_event::message::{MessageEvent, MessageBase};
///
/// fn handle_message(event: &MessageEvent) {
///     match event {
///         MessageEvent::Friend(msg) => {
///             println!("收到好友消息: {:?}", msg.get_text());
///         }
///         MessageEvent::Group(msg) => {
///             println!("收到群消息: {:?}", msg.get_text());
///             println!("群 ID: {}", msg.group_id());
///         }
///     }
/// }
/// ```
///
/// ## 使用类型转换方法
///
/// ```rust,ignore
/// use puniyu_event::message::MessageEvent;
///
/// fn process_friend_message(event: &MessageEvent) {
///     if let Some(friend_msg) = event.as_friend() {
///         println!("这是好友消息");
///         // 处理好友消息
///     }
/// }
/// ```
///
/// ## 统一处理消息
///
/// ```rust,ignore
/// use puniyu_event::message::{MessageEvent, MessageBase};
///
/// fn get_message_text(event: &MessageEvent) -> Vec<&str> {
///     // MessageEvent 实现了 MessageBase trait
///     event.get_text()
/// }
/// ```
#[derive(Debug, Clone)]
pub enum MessageEvent<'m> {
	/// 好友消息事件
	Friend(FriendMessage<'m>),
	/// 群聊消息事件
	Group(GroupMessage<'m>),
}

impl MessageEvent<'_> {
	/// 尝试将消息事件转换为好友消息
	///
	/// # 返回值
	///
	/// 如果是好友消息，返回 `Some(&FriendMessage)`，否则返回 `None`
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
			MessageEvent::Friend(friend) => Some(friend),
			_ => None,
		}
	}

	/// 尝试将消息事件转换为群消息
	///
	/// # 返回值
	///
	/// 如果是群消息，返回 `Some(&GroupMessage)`，否则返回 `None`
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// if let Some(group_msg) = event.as_group() {
	///     println!("群 ID: {}", group_msg.group_id());
	/// }
	/// ```
	pub fn as_group(&self) -> Option<&GroupMessage<'_>> {
		match self {
			MessageEvent::Group(group) => Some(group),
			_ => None,
		}
	}
}

impl MessageEvent<'_> {
	/// 获取消息触发时间戳
	///
	/// # 返回值
	///
	/// 返回 Unix 时间戳（秒）
	pub fn time(&self) -> u64 {
		match self {
			Self::Group(message) => message.time(),
			Self::Friend(message) => message.time(),
		}
	}

	/// 获取事件类型
	///
	/// # 返回值
	///
	/// 返回 `EventType::Message`
	pub fn event(&self) -> &EventType {
		match self {
			Self::Group(message) => message.event(),
			Self::Friend(message) => message.event(),
		}
	}

	/// 获取事件 ID
	///
	/// # 返回值
	///
	/// 返回事件的唯一标识符
	pub fn event_id(&self) -> &str {
		match self {
			Self::Group(message) => message.event_id(),
			Self::Friend(message) => message.event_id(),
		}
	}

	/// 获取消息子类型
	///
	/// # 返回值
	///
	/// 返回消息子类型（Friend 或 Group）
	pub fn sub_event(&self) -> &MessageSubEventType {
		match self {
			Self::Group(message) => message.sub_event(),
			Self::Friend(message) => message.sub_event(),
		}
	}

	/// 获取机器人实例
	///
	/// # 返回值
	///
	/// 返回处理该消息的机器人实例引用
	pub fn bot(&self) -> &Bot {
		match self {
			Self::Group(message) => message.bot(),
			Self::Friend(message) => message.bot(),
		}
	}

	/// 获取机器人 ID
	///
	/// # 返回值
	///
	/// 返回机器人的唯一标识符
	pub fn self_id(&self) -> &str {
		match self {
			Self::Group(message) => message.self_id(),
			Self::Friend(message) => message.self_id(),
		}
	}

	/// 获取用户 ID
	///
	/// # 返回值
	///
	/// 返回发送消息的用户 ID
	pub fn user_id(&self) -> &str {
		match self {
			Self::Group(message) => message.user_id(),
			Self::Friend(message) => message.user_id(),
		}
	}

	/// 获取联系人信息
	///
	/// # 返回值
	///
	/// 返回消息发生的联系人信息（好友或群聊）
	pub fn contact(&self) -> ContactType<'_> {
		match self {
			Self::Group(message) => ContactType::from(message.contact().clone()),
			Self::Friend(message) => ContactType::from(message.contact().clone()),
		}
	}

	/// 获取发送者信息
	///
	/// # 返回值
	///
	/// 返回发送消息的用户详细信息
	pub fn sender(&self) -> SenderType<'_> {
		match self {
			Self::Group(message) => SenderType::from(message.sender().clone()),
			Self::Friend(message) => SenderType::from(message.sender().clone()),
		}
	}
}


impl MessageEvent<'_> {
	/// 获取消息 ID
	///
	/// ##  返回值
	///
	/// 返回消息的 ID
	pub fn message_id(&self) -> &str {
		match self {
			Self::Group(message) => message.message_id(),
			Self::Friend(message) => message.message_id(),
		}
	}

	/// 获取消息元素
	///
	/// ## 返回值
	///
	/// 返回消息的元素
	pub fn elements(&self) -> &Vec<Elements<'_>> {
		match self {
			Self::Group(message) => message.elements(),
			Self::Friend(message) => message.elements(),
		}
	}
}