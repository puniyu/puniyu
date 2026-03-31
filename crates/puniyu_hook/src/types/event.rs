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

/// 事件钩子类型。
#[derive(
	Debug,
	Copy,
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
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum HookEventType {
	/// 消息事件。
	Message,
	/// 通知事件。
	Notion,
	/// 请求事件。
	Request,
	/// 所有事件。
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
