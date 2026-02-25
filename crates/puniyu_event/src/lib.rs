//! # puniyu_event
//!
//! 事件类型定义库，提供聊天机器人中的各类事件类型系统。
//!
//! ## 概述
//!
//! `puniyu_event` 提供了完整的事件类型定义，用于处理聊天机器人中的各种事件。
//! 该库将事件分为三大类：
//!
//! - **消息事件（Message）** - 处理好友和群聊消息
//! - **通知事件（Notion）** - 处理各类通知（戳一戳、撤回、文件上传等）
//! - **请求事件（Request）** - 处理好友申请、群申请等请求
//!
//! ## 特性
//!
//! - 🎯 **类型安全** - 使用 Rust 类型系统确保事件处理的正确性
//! - 🔧 **统一接口** - 通过 trait 提供统一的事件访问接口
//! - 📦 **序列化支持** - 内置 serde 支持
//! - 🎨 **生命周期优化** - 使用生命周期参数避免不必要的内存分配
//! - 🔄 **丰富的事件类型** - 支持消息、通知、请求等多种事件类型
//!
//! ## 快速开始
//!
//! ### 处理事件
//!
//! ```rust,ignore
//! use puniyu_event::{Event, EventBase};
//!
//! fn handle_event(event: Event) {
//!     match event {
//!         Event::Message(msg) => {
//!             // 处理消息事件
//!             let texts = msg.get_text();
//!             println!("收到消息: {:?}", texts);
//!         }
//!         Event::Notion(notion) => {
//!             // 处理通知事件
//!             println!("收到通知");
//!         }
//!         Event::Request(request) => {
//!             // 处理请求事件
//!             println!("收到请求");
//!         }
//!     }
//! }
//! ```
//!
//! ### 处理好友消息
//!
//! ```rust,ignore
//! use puniyu_event::message::{FriendMessage, MessageBase};
//!
//! fn handle_friend_message(msg: &FriendMessage) {
//!     // 获取消息文本
//!     let texts = msg.get_text();
//!     println!("收到好友消息: {:?}", texts);
//!     
//!     // 获取发送者信息
//!     let sender = msg.sender();
//!     println!("发送者: {}", sender.user_id());
//!     
//!     // 判断是否为主人
//!     if msg.is_master() {
//!         println!("这是主人发送的消息");
//!     }
//! }
//! ```
//!
//! ### 处理群消息
//!
//! ```rust,ignore
//! use puniyu_event::message::{GroupMessage, MessageBase};
//!
//! fn handle_group_message(msg: &GroupMessage) {
//!     // 获取群 ID
//!     let group_id = msg.group_id();
//!     println!("群 ID: {}", group_id);
//!     
//!     // 判断发送者是否为管理员
//!     if msg.is_admin() {
//!         println!("管理员发送的消息");
//!     }
//!     
//!     // 获取消息文本
//!     let texts = msg.get_text();
//!     println!("群消息: {:?}", texts);
//! }
//! ```
//!
//! ### 使用 EventBase trait
//!
//! ```rust,ignore
//! use puniyu_event::EventBase;
//!
//! fn print_event_info<E>(event: &E)
//! where
//!     E: EventBase
//! {
//!     println!("事件 ID: {}", event.event_id());
//!     println!("时间戳: {}", event.time());
//!     println!("用户 ID: {}", event.user_id());
//!     println!("机器人 ID: {}", event.self_id());
//! }
//! ```

pub mod message;
pub mod notion;
pub mod request;
mod types;
#[doc(inline)]
pub use types::*;

use message::MessageEvent;
use notion::NotionEvent;
use puniyu_bot::Bot;
use puniyu_contact::ContactType;
use puniyu_sender::SenderType;
use request::RequestEvent;

/// 事件枚举
///
/// 统一的事件类型，包含所有可能的事件。
///
/// # 变体
///
/// - `Message(MessageEvent)` - 消息事件，包括好友消息和群消息
/// - `Notion(NotionEvent)` - 通知事件，包括戳一戳、撤回等各类通知
/// - `Request(RequestEvent)` - 请求事件，包括好友申请、群申请等
///
/// # 示例
///
/// ## 基本事件处理
///
/// ```rust,ignore
/// use puniyu_event::{Event, EventBase};
///
/// fn handle_event(event: Event) {
///     match event {
///         Event::Message(msg) => {
///             // 处理消息事件
///             let texts = msg.get_text();
///             println!("收到消息: {:?}", texts);
///         }
///         Event::Notion(notion) => {
///             // 处理通知事件
///             println!("收到通知，类型: {:?}", notion.sub_event());
///         }
///         Event::Request(request) => {
///             // 处理请求事件
///             println!("收到请求，类型: {:?}", request.sub_event());
///         }
///     }
/// }
/// ```
///
/// ## 使用类型判断方法
///
/// ```rust,ignore
/// use puniyu_event::Event;
///
/// fn check_event_type(event: &Event) {
///     if let Some(msg) = event.as_message() {
///         println!("这是消息事件");
///     } else if let Some(notion) = event.as_notion() {
///         println!("这是通知事件");
///     } else if let Some(request) = event.as_request() {
///         println!("这是请求事件");
///     }
/// }
/// ```
///
/// ## 访问通用事件信息
///
/// ```rust,ignore
/// use puniyu_event::Event;
///
/// fn print_event_info(event: &Event) {
///     println!("事件 ID: {}", event.event_id());
///     println!("时间戳: {}", event.time());
///     println!("用户 ID: {}", event.user_id());
///     println!("机器人 ID: {}", event.self_id());
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

impl Event<'_> {
	/// 尝试将事件转换为消息事件
	///
	/// # 返回值
	///
	/// 如果是消息事件，返回 `Some(&MessageEvent)`，否则返回 `None`
	pub fn as_message(&self) -> Option<&MessageEvent<'_>> {
		match self {
			Event::Message(m) => Some(m),
			_ => None,
		}
	}

	/// 尝试将事件转换为通知事件
	///
	/// # 返回值
	///
	/// 如果是通知事件，返回 [`Some(&NotionEvent)`]，否则返回 [`None`]
	pub fn as_notion(&self) -> Option<&NotionEvent<'_>> {
		match self {
			Event::Notion(n) => Some(n),
			_ => None,
		}
	}

	/// 尝试将事件转换为请求事件
	///
	/// # 返回值
	///
	/// 如果是请求事件，返回 `Some(&RequestEvent)`，否则返回 `None`
	pub fn as_request(&self) -> Option<&RequestEvent<'_>> {
		match self {
			Event::Request(r) => Some(r),
			_ => None,
		}
	}
}

impl Event<'_> {
	/// 获取事件触发时间戳
	///
	/// # 返回值
	///
	/// 返回 Unix 时间戳（秒）
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let timestamp = event.time();
	/// println!("事件发生时间: {}", timestamp);
	/// ```
	pub fn time(&self) -> u64 {
		match self {
			Self::Message(event) => event.time(),
			Self::Notion(event) => event.time(),
			Self::Request(event) => event.time(),
		}
	}

	/// 获取事件类型
	///
	/// # 返回值
	///
	/// 返回事件类型枚举的引用（Message、Notion 或 Request）
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_event::EventType;
	///
	/// match event.event() {
	///     EventType::Message => println!("消息事件"),
	///     EventType::Notion => println!("通知事件"),
	///     EventType::Request => println!("请求事件"),
	///     _ => {}
	/// }
	/// ```
	pub fn event(&self) -> &EventType {
		match self {
			Self::Message(event) => event.event(),
			Self::Notion(event) => event.event(),
			Self::Request(event) => event.event(),
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
			Self::Message(event) => event.event_id(),
			Self::Notion(event) => event.event_id(),
			Self::Request(event) => event.event_id(),
		}
	}

	/// 获取事件子类型
	///
	/// # 返回值
	///
	/// 返回统一的子事件类型枚举
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_event::SubEventType;
	///
	/// let sub_event = event.sub_event();
	/// match sub_event {
	///     SubEventType::Message(msg_type) => {
	///         println!("消息子类型: {:?}", msg_type);
	///     }
	///     SubEventType::Notion(notion_type) => {
	///         println!("通知子类型: {:?}", notion_type);
	///     }
	///     SubEventType::Request(request_type) => {
	///         println!("请求子类型: {:?}", request_type);
	///     }
	/// }
	/// ```
	pub fn sub_event(&self) -> SubEventType {
		match self {
			Self::Message(event) => SubEventType::from(event.sub_event().clone()),
			Self::Notion(event) => SubEventType::from(event.sub_event().clone()),
			Self::Request(event) => SubEventType::from(event.sub_event().clone()),
		}
	}

	/// 获取机器人实例
	///
	/// # 返回值
	///
	/// 返回处理该事件的机器人实例引用
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let bot = event.bot();
	/// println!("机器人账号: {}", bot.account().uin);
	/// ```
	pub fn bot(&self) -> &Bot {
		match self {
			Self::Message(event) => event.bot(),
			Self::Notion(event) => event.bot(),
			Self::Request(event) => event.bot(),
		}
	}

	/// 获取机器人 ID
	///
	/// # 返回值
	///
	/// 返回机器人的唯一标识符（通常是 QQ 号）
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let bot_id = event.self_id();
	/// println!("机器人 ID: {}", bot_id);
	/// ```
	pub fn self_id(&self) -> &str {
		match self {
			Self::Message(event) => event.self_id(),
			Self::Notion(event) => event.self_id(),
			Self::Request(event) => event.self_id(),
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
			Self::Message(event) => event.user_id(),
			Self::Notion(event) => event.user_id(),
			Self::Request(event) => event.user_id(),
		}
	}

	/// 获取联系人信息
	///
	/// # 返回值
	///
	/// 返回事件发生的联系人信息（好友或群聊）
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_contact::ContactType;
	///
	/// let contact = event.contact();
	/// match contact {
	///     ContactType::Friend(friend) => {
	///         println!("好友联系人");
	///     }
	///     ContactType::Group(group) => {
	///         println!("群聊联系人");
	///     }
	/// }
	/// ```
	pub fn contact(&self) -> ContactType<'_> {
		match self {
			Self::Message(event) => event.contact(),
			Self::Notion(event) => event.contact(),
			Self::Request(event) => event.contact(),
		}
	}

	/// 获取发送者信息
	///
	/// # 返回值
	///
	/// 返回触发事件的发送者详细信息
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_sender::SenderType;
	///
	/// let sender = event.sender();
	/// match sender {
	///     SenderType::Friend(friend_sender) => {
	///         println!("好友发送者");
	///     }
	///     SenderType::Group(group_sender) => {
	///         println!("群成员发送者");
	///     }
	/// }
	/// ```
	pub fn sender(&self) -> SenderType<'_> {
		match self {
			Self::Message(event) => event.sender(),
			Self::Notion(event) => event.sender(),
			Self::Request(event) => event.sender(),
		}
	}
}
