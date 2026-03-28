use crate::Event;
use crate::message::MessageSubEventType;
use crate::notion::NotionSubEventType;
use crate::request::RequestSubEventType;
use puniyu_bot::Bot;
use puniyu_contact::Contact;
use puniyu_sender::Sender;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

/// 事件类型枚举
///
/// 定义了所有可能的事件类型，用于区分不同类别的事件。
///
/// # 变体
///
/// - `Message` - 消息事件，包括好友消息和群消息
/// - `Notion` - 通知事件，包括戳一戳、撤回、文件上传等
/// - `Request` - 请求事件，包括好友申请、群申请等
/// - `Unknown` - 未知事件类型（默认值）
///
/// # 示例
///
/// ```rust
/// use puniyu_event::EventType;
/// use std::str::FromStr;
///
/// let event_type = EventType::Message;
/// assert_eq!(event_type.to_string(), "message");
///
/// let event_type = EventType::from_str("notion").unwrap();
/// assert_eq!(event_type, EventType::Notion);
/// ```
///
/// # 序列化
///
/// 该枚举实现了 `Serialize` 和 `Deserialize`，可以直接用于 JSON 序列化：
///
/// ```rust
/// use puniyu_event::EventType;
/// use serde_json;
///
/// let event_type = EventType::Message;
/// let json = serde_json::to_string(&event_type).unwrap();
/// assert_eq!(json, r#""message""#);
/// ```
#[derive(
	Debug,
	Default,
	Clone,
	EnumString,
	Display,
	IntoStaticStr,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	Deserialize,
	Serialize,
)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum EventType {
	/// 消息事件
	Message,
	/// 通知事件
	Notion,
	/// 请求事件
	Request,
	/// 未知事件类型
	#[default]
	Unknown,
}

impl<'e> From<Event<'e>> for EventType {
	fn from(event: Event) -> Self {
		match event {
			Event::Message(_) => EventType::Message,
			Event::Notion(_) => EventType::Notion,
			Event::Request(_) => EventType::Request,
		}
	}
}

impl<'e> From<&Event<'e>> for EventType {
	fn from(event: &Event) -> Self {
		match event {
			Event::Message(_) => EventType::Message,
			Event::Notion(_) => EventType::Notion,
			Event::Request(_) => EventType::Request,
		}
	}
}

impl<'e> std::fmt::Debug for Event<'e> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Event::Message(message_event) => f.debug_tuple("Message").field(message_event).finish(),
			Event::Notion(notion_event) => f.debug_tuple("Notion").field(notion_event).finish(),
			Event::Request(request_event) => f.debug_tuple("Request").field(request_event).finish(),
		}
	}
}

/// 子事件类型枚举
///
/// 统一的子事件类型枚举，包含消息、通知和请求的所有子类型。
///
/// # 变体
///
/// - `Message(MessageSubEventType)` - 消息子类型（好友消息、群消息等）
/// - `Notion(NotionSubEventType)` - 通知子类型（戳一戳、撤回等）
/// - `Request(RequestSubEventType)` - 请求子类型（好友申请、群申请等）
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_event::{SubEventType, message::MessageSubEventType};
///
/// let sub_event = SubEventType::Message(MessageSubEventType::Friend);
/// match sub_event {
///     SubEventType::Message(msg_type) => {
///         println!("消息类型: {:?}", msg_type);
///     }
///     SubEventType::Notion(notion_type) => {
///         println!("通知类型: {:?}", notion_type);
///     }
///     SubEventType::Request(request_type) => {
///         println!("请求类型: {:?}", request_type);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SubEventType {
	Message(MessageSubEventType),
	Notion(NotionSubEventType),
	Request(RequestSubEventType),
}

impl From<MessageSubEventType> for SubEventType {
	fn from(sub_event: MessageSubEventType) -> Self {
		Self::Message(sub_event)
	}
}
impl From<&MessageSubEventType> for SubEventType {
	fn from(sub_event: &MessageSubEventType) -> Self {
		Self::Message(*sub_event)
	}
}
impl From<NotionSubEventType> for SubEventType {
	fn from(sub_event: NotionSubEventType) -> Self {
		Self::Notion(sub_event)
	}
}
impl From<&NotionSubEventType> for SubEventType {
	fn from(sub_event: &NotionSubEventType) -> Self {
		Self::Notion(*sub_event)
	}
}
impl From<RequestSubEventType> for SubEventType {
	fn from(sub_event: RequestSubEventType) -> Self {
		Self::Request(sub_event)
	}
}
impl From<&RequestSubEventType> for SubEventType {
	fn from(sub_event: &RequestSubEventType) -> Self {
		Self::Request(*sub_event)
	}
}

/// 事件基础 trait
///
/// 定义所有事件类型的通用接口，提供事件的基本信息访问方法。
///
/// 该 trait 是所有事件类型的基础，包括消息事件、通知事件和请求事件。
/// 通过实现此 trait，可以统一访问事件的基本属性。
///
/// # 关联类型
///
/// - `EventType`: 事件类型枚举，用于区分消息、通知、请求等
/// - `SubEventType`: 事件子类型枚举，用于区分具体的事件类型
/// - `Contact`: 联系人类型，表示事件发生的场景（好友、群聊等）
/// - `Sender`: 发送者类型，表示触发事件的用户信息
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_event::EventBase;
///
/// fn print_event_info<E>(event: &E)
/// where
///     E: EventBase
/// {
///     println!("事件 ID: {}", event.event_id());
///     println!("时间戳: {}", event.time());
///     println!("用户 ID: {}", event.user_id());
///     println!("机器人 ID: {}", event.self_id());
/// }
/// ```
///
/// # 实现者
///
/// - `FriendMessage` - 好友消息事件
/// - `GroupMessage` - 群消息事件
/// - 各类通知事件（戳一戳、撤回等）
/// - 各类请求事件（好友申请、群申请等）
pub trait EventBase: Send + Sync {
	/// 事件类型枚举
	type EventType: PartialEq;
	/// 事件子类型枚举
	type SubEventType: PartialEq;
	/// 联系人类型
	type Contact: Contact + ?Sized;
	/// 发送者类型
	type Sender: Sender + ?Sized;

	/// 获取事件触发时间戳（秒）
	///
	/// # 返回值
	///
	/// 返回 Unix 时间戳（秒）
	fn time(&self) -> u64;

	/// 获取事件类型
	///
	/// # 返回值
	///
	/// 返回事件类型的引用，可能是 Message、Notion 或 Request
	fn event_type(&self) -> &Self::EventType;

	/// 获取事件 ID
	///
	/// # 返回值
	///
	/// 返回事件的唯一标识符
	fn event_id(&self) -> &str;

	/// 获取事件子类型
	///
	/// # 返回值
	///
	/// 返回事件的具体子类型，如好友消息、群消息等
	fn sub_event(&self) -> &Self::SubEventType;

	/// 获取机器人实例
	///
	/// # 返回值
	///
	/// 返回处理该事件的机器人实例引用
	fn bot(&self) -> &Bot;

	/// 获取机器人 ID
	///
	/// # 返回值
	///
	/// 返回机器人的唯一标识符（通常是 QQ 号）
	fn self_id(&self) -> &str;

	/// 获取用户 ID
	///
	/// # 返回值
	///
	/// 返回触发事件的用户 ID
	fn user_id(&self) -> &str;

	/// 获取联系人信息
	///
	/// # 返回值
	///
	/// 返回事件发生的联系人信息（好友或群聊）
	fn contact(&self) -> &Self::Contact;

	/// 获取发送者信息
	///
	/// # 返回值
	///
	/// 返回触发事件的发送者详细信息
	fn sender(&self) -> &Self::Sender;
}

macro_rules! codegen_from_for_content_type {
    ($($variant:ident($inner_type:ty)),* $(,)?) => {
        $(
            impl From<$inner_type> for ContentType {
                fn from(value: $inner_type) -> Self {
                    Self::$variant(value)
                }
            }
        )*
    };
}
pub(crate) use codegen_from_for_content_type;

/// 为事件枚举生成方法委托和类型转换方法
///
/// 生成 `as_*` 方法、简单委托方法和类型转换方法。
macro_rules! codegen_delegate_to_variants {
	($self:ident, $method:ident, $($variant:ident),+) => {
		match $self {
			$(Self::$variant(inner) => inner.$method(),)+
		}
	};
}
pub(crate) use codegen_delegate_to_variants;

macro_rules! codegen_delegate_to_variants_convert {
	($self:ident, $method:ident, $convert:ty, $($variant:ident),+) => {
		match $self {
			$(Self::$variant(inner) => <$convert>::from(inner.$method().clone()),)+
		}
	};
}
pub(crate) use codegen_delegate_to_variants_convert;

macro_rules! codegen_impl_as {
	($enum_name:ident { $($variant:ident($type:ident) => $method_name:ident),* $(,)? }) => {
		impl $enum_name<'_> {
			$(
				pub fn $method_name(&self) -> Option<&$type<'_>> {
					match self {
						Self::$variant(inner) => Some(inner),
						_ => None,
					}
				}
			)*
		}
	};
}
pub(crate) use codegen_impl_as;
