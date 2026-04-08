use puniyu_event::EventType;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::{Display, IntoStaticStr};

mod message;
#[doc(inline)]
pub use message::MessageType;

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
	/// 扩展事件。
	Extension,
	/// 所有事件。
	#[default]
	All,
}

impl From<EventType> for HookEventType {
	fn from(event_type: EventType) -> Self {
		match event_type {
			EventType::Message => Self::Message,
			EventType::Extension => Self::Extension,
			EventType::Unknown => Self::All,
		}
	}
}

impl FromStr for HookEventType {
	type Err = std::convert::Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"message" => Ok(Self::Message),
			"extension" => Ok(Self::Extension),
			"all" => Ok(Self::All),
			_ => Ok(Self::default()),
		}
	}
}
