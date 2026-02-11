use puniyu_event::EventType;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::{Display, IntoStaticStr};

mod message;
#[doc(inline)]
pub use message::MessageType;
mod notion;
#[doc(inline)]
pub use notion::NotionType;
mod request;
#[doc(inline)]
pub use request::RequestType;

/// 钩子事件类型枚举
///
/// 定义钩子可以监听的事件类型。
///
/// # 变体
///
/// - `Message` - 消息事件钩子
/// - `Notion` - 通知事件钩子
/// - `Request` - 请求事件钩子
/// - `All` - 所有事件钩子（默认）
///
/// # 示例
///
/// ```rust
/// use puniyu_hook::types::HookEventType;
/// use std::str::FromStr;
///
/// let event_type = HookEventType::Message;
/// assert_eq!(event_type.to_string(), "Message");
///
/// let event_type = HookEventType::from_str("notion").unwrap();
/// assert_eq!(event_type, HookEventType::Notion);
/// ```
#[derive(
	Debug,
	Clone,
	Default,
	Display,
	IntoStaticStr,
	Deserialize,
	Serialize,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
)]
pub enum HookEventType {
	/// 消息事件
	Message,
	/// 通知事件
	Notion,
	/// 请求事件
	Request,
	/// 所有事件（默认）
	#[default]
	All,
}

impl From<EventType> for HookEventType {
	fn from(event_type: EventType) -> Self {
		match event_type {
			EventType::Message => Self::Message,
			EventType::Notion => Self::Notion,
			EventType::Request => Self::Request,
			EventType::Unknown => Self::All,
		}
	}
}

impl FromStr for HookEventType {
	type Err = std::convert::Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"message" => Ok(Self::Message),
			"notion" => Ok(Self::Notion),
			"request" => Ok(Self::Request),
			"all" => Ok(Self::All),
			_ => Ok(Self::default()),
		}
	}
}
