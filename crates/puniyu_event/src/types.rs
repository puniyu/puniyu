mod permission;
#[doc(inline)]
pub use permission::Permission;

use super::message::MessageEvent;
use super::notion::NotionEvent;
use super::request::RequestEvent;
use puniyu_bot::Bot;
use puniyu_contact::{Contact, SceneType};
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
/// assert_eq!(json, r#""Message""#);
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
pub enum EventType {
	/// 消息事件
	#[strum(serialize = "message")]
	Message,
	/// 通知事件
	#[strum(serialize = "notion")]
	Notion,
	/// 请求事件
	#[strum(serialize = "request")]
	Request,
	/// 未知事件类型
	#[strum(serialize = "unknown")]
	#[default]
	Unknown,
}

/// 事件枚举
///
/// 包含所有类型的事件，使用 Box 包装以减少栈空间占用。
///
/// # 变体
///
/// - `Message` - 消息事件，包含 `MessageEvent`
/// - `Notion` - 通知事件，包含 `NotionEvent`
/// - `Request` - 请求事件，包含 `RequestEvent`
///
/// # 生命周期
///
/// 使用生命周期参数 `'e` 来避免不必要的内存分配，提高性能。
///
/// # 示例
///
/// ## 基本使用
///
/// ```rust,ignore
/// use puniyu_event::Event;
///
/// fn handle_event(event: Event) {
///     match event {
///         Event::Message(msg) => {
///             println!("收到消息事件");
///             // 处理消息
///         }
///         Event::Notion(notion) => {
///             println!("收到通知事件");
///             // 处理通知
///         }
///         Event::Request(request) => {
///             println!("收到请求事件");
///             // 处理请求
///         }
///     }
/// }
/// ```
///
/// ## 类型判断
///
/// ```rust,ignore
/// use puniyu_event::Event;
///
/// fn check_event_type(event: &Event) {
///     if event.is_message() {
///         println!("这是消息事件");
///     } else if event.is_notice() {
///         println!("这是通知事件");
///     } else if event.is_request() {
///         println!("这是请求事件");
///     }
/// }
/// ```
///
/// ## 类型转换
///
/// ```rust,ignore
/// use puniyu_event::Event;
///
/// fn process_message(event: &Event) {
///     if let Some(msg) = event.as_message() {
///         // 处理消息事件
///         println!("消息 ID: {}", msg.message_id());
///     }
/// }
/// ```
#[derive(Clone)]
pub enum Event<'e> {
	/// 消息事件
	Message(Box<MessageEvent<'e>>),
	/// 通知事件
	Notion(Box<NotionEvent<'e>>),
	/// 请求事件
	Request(Box<RequestEvent<'e>>),
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

impl<'e> Event<'e> {
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
			Event::Message(m) => m.bot(),
			Event::Notion(n) => n.bot(),
			Event::Request(r) => r.bot(),
		}
	}

	/// 获取事件时间戳
	///
	/// # 返回值
	///
	/// 返回事件触发的时间戳（秒）
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let timestamp = event.time();
	/// println!("事件时间: {}", timestamp);
	/// ```
	pub fn time(&self) -> u64 {
		match self {
			Event::Message(m) => m.time(),
			Event::Notion(n) => n.time(),
			Event::Request(r) => r.time(),
		}
	}

	/// 获取事件类型
	///
	/// # 返回值
	///
	/// 返回事件类型的引用
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let event_type = event.event();
	/// println!("事件类型: {:?}", event_type);
	/// ```
	pub fn event(&self) -> &EventType {
		match self {
			Event::Message(m) => m.event(),
			Event::Notion(_) => &EventType::Notion,
			Event::Request(_) => &EventType::Request,
		}
	}

	/// 获取事件 ID
	///
	/// # 返回值
	///
	/// 返回事件的唯一标识符
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let event_id = event.event_id();
	/// println!("事件 ID: {}", event_id);
	/// ```
	pub fn event_id(&self) -> &str {
		match self {
			Event::Message(m) => m.event_id(),
			Event::Notion(n) => n.event_id(),
			Event::Request(r) => r.event_id(),
		}
	}

	/// 获取机器人自身 ID
	///
	/// # 返回值
	///
	/// 返回接收事件的机器人 ID
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let self_id = event.self_id();
	/// println!("机器人 ID: {}", self_id);
	/// ```
	pub fn self_id(&self) -> &str {
		match self {
			Event::Message(m) => m.self_id(),
			Event::Notion(n) => n.self_id(),
			Event::Request(r) => r.self_id(),
		}
	}

	/// 获取用户 ID
	///
	/// # 返回值
	///
	/// 返回触发事件的用户 ID
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let user_id = event.user_id();
	/// println!("用户 ID: {}", user_id);
	/// ```
	pub fn user_id(&self) -> &str {
		match self {
			Event::Message(m) => m.user_id(),
			Event::Notion(n) => n.user_id(),
			Event::Request(r) => r.user_id(),
		}
	}

	/// 判断发送者是否为主人
	///
	/// # 返回值
	///
	/// 如果发送者是配置的主人返回 `true`，否则返回 `false`
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// if event.is_master() {
	///     println!("主人触发的事件");
	/// }
	/// ```
	pub fn is_master(&self) -> bool {
		match self {
			Event::Message(m) => m.is_master(),
			Event::Notion(n) => n.is_master(),
			Event::Request(r) => r.is_master(),
		}
	}

	/// 获取事件类型
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let event_type = event.event_type();
	/// println!("事件类型: {}", event_type);
	/// ```
	pub fn event_type(&self) -> &EventType {
		match self {
			Event::Message(_) => &EventType::Message,
			Event::Notion(_) => &EventType::Notion,
			Event::Request(_) => &EventType::Request,
		}
	}

	/// 判断是否为好友事件
	///
	/// # 返回值
	///
	/// 如果是好友相关的事件返回 `true`，否则返回 `false`
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_event::Event;
	///
	/// fn handle_event(event: &Event) {
	///     if event.is_friend() {
	///         println!("这是好友事件");
	///     }
	/// }
	/// ```
	pub fn is_friend(&self) -> bool {
		match self {
			Self::Message(m) => m.is_friend(),
			Self::Notion(n) => n.is_friend(),
			Self::Request(r) => r.is_friend(),
		}
	}

	/// 判断是否为群组事件
	///
	/// # 返回值
	///
	/// 如果是群组相关的事件返回 `true`，否则返回 `false`
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_event::Event;
	///
	/// fn handle_event(event: &Event) {
	///     if event.is_group() {
	///         println!("这是群组事件");
	///     }
	/// }
	/// ```
	pub fn is_group(&self) -> bool {
		match self {
			Self::Message(m) => m.is_group(),
			Self::Notion(n) => n.is_group(),
			Self::Request(r) => r.is_group(),
		}
	}

	/// 判断是否为消息事件
	///
	/// # 返回值
	///
	/// 如果是消息事件返回 `true`，否则返回 `false`
	pub fn is_message(&self) -> bool {
		matches!(self, Event::Message(_))
	}

	/// 判断是否为通知事件
	///
	/// # 返回值
	///
	/// 如果是通知事件返回 `true`，否则返回 `false`
	pub fn is_notice(&self) -> bool {
		matches!(self, Event::Notion(_))
	}

	/// 判断是否为请求事件
	///
	/// # 返回值
	///
	/// 如果是请求事件返回 `true`，否则返回 `false`
	pub fn is_request(&self) -> bool {
		matches!(self, Event::Request(_))
	}

	/// 将事件转换为消息事件
	///
	/// # 返回值
	///
	/// 如果是消息事件返回 `Some(&MessageEvent)`，否则返回 `None`
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_event::Event;
	///
	/// fn process_message(event: &Event) {
	///     if let Some(msg) = event.as_message() {
	///         println!("消息 ID: {}", msg.message_id());
	///     }
	/// }
	/// ```
	pub fn as_message(&self) -> Option<&MessageEvent<'e>> {
		match self {
			Event::Message(message_event) => Some(message_event),
			_ => None,
		}
	}

	/// 将事件转换为通知事件
	///
	/// # 返回值
	///
	/// 如果是通知事件返回 `Some(&NotionEvent)`，否则返回 `None`
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_event::Event;
	///
	/// fn process_notice(event: &Event) {
	///     if let Some(notice) = event.as_notice() {
	///         println!("通知类型: {:?}", notice.sub_event());
	///     }
	/// }
	/// ```
	pub fn as_notice(&self) -> Option<&NotionEvent<'e>> {
		match self {
			Event::Notion(notion_event) => Some(notion_event),
			_ => None,
		}
	}

	/// 将事件转换为请求事件
	///
	/// # 返回值
	///
	/// 如果是请求事件返回 `Some(&RequestEvent)`，否则返回 `None`
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_event::Event;
	///
	/// fn process_request(event: &Event) {
	///     if let Some(request) = event.as_request() {
	///         println!("请求类型: {:?}", request.sub_event());
	///     }
	/// }
	/// ```
	pub fn as_request(&self) -> Option<&RequestEvent<'e>> {
		match self {
			Event::Request(request_event) => Some(request_event),
			_ => None,
		}
	}
}

/// 事件基础 trait
///
/// 定义所有事件类型的通用接口，提供统一的访问方法。
///
/// 所有事件类型（消息、通知、请求）都必须实现此 trait，以提供一致的事件信息访问方式。
///
/// # 类型参数
///
/// - `EventType`: 事件类型枚举，用于标识事件的大类
/// - `SubEventType`: 事件子类型枚举，用于标识事件的具体类型
///
/// # 关联类型
///
/// - `Contact`: 联系人类型，必须实现 `Contact` trait
/// - `Sender`: 发送者类型，必须实现 `Sender` trait
///
/// # 示例
///
/// ## 使用泛型函数处理事件
///
/// ```rust,ignore
/// use puniyu_event::{EventBase, EventType};
///
/// fn print_event_info<E, S>(event: &E)
/// where
///     E: EventBase<EventType, S>,
///     S: PartialEq,
/// {
///     println!("事件 ID: {}", event.event_id());
///     println!("时间戳: {}", event.time());
///     println!("用户 ID: {}", event.user_id());
///     println!("机器人 ID: {}", event.self_id());
/// }
/// ```
///
/// ## 访问联系人和发送者信息
///
/// ```rust,ignore
/// use puniyu_event::EventBase;
///
/// fn process_event<E, T, S>(event: &E)
/// where
///     E: EventBase<T, S>,
///     T: PartialEq,
///     S: PartialEq,
/// {
///     let sender = event.sender();
///     println!("发送者昵称: {}", sender.nickname());
///     
///     let contact = event.contact();
///     println!("联系人类型: {:?}", contact.contact_type());
/// }
/// ```
pub trait EventBase: Send + Sync {
	/// 事件类型枚举
	type EventType: PartialEq;
	/// 事件子类型枚举
	type SubEventType: PartialEq;
	/// 联系人类型
	type Contact: Contact;
	/// 发送者类型
	type Sender: Sender;

	/// 获取事件触发时间戳（秒）
	fn time(&self) -> u64;

	/// 获取事件类型
	fn event(&self) -> &Self::EventType;

	/// 获取事件 ID
	fn event_id(&self) -> &str;

	/// 获取事件子类型
	fn sub_event(&self) -> &Self::SubEventType;

	/// 获取机器人实例
	fn bot(&self) -> &Bot;
	/// 获取机器人 ID
	fn self_id(&self) -> &str;

	/// 获取用户 ID
	fn user_id(&self) -> &str;

	/// 获取联系人信息
	fn contact(&self) -> &Self::Contact;

	/// 获取发送者信息
	fn sender(&self) -> &Self::Sender;

	/// 判断是否为好友消息
	fn is_friend(&self) -> bool {
		matches!(self.contact().scene(), SceneType::Friend)
	}

	/// 判断是否为群消息
	fn is_group(&self) -> bool {
		matches!(self.contact().scene(), SceneType::Group)
	}
}
