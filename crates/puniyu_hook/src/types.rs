mod event;
#[doc(inline)]
pub use event::*;
mod info;
#[doc(inline)]
pub use info::*;
mod status;
#[doc(inline)]
pub use status::*;

use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::{Display, IntoStaticStr};

/// 钩子类型。
#[derive(
	Debug,
	Copy,
	Clone,
	Display,
	IntoStaticStr,
	Deserialize,
	Serialize,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
)]
pub enum HookType {
	/// 事件钩子。
	Event(HookEventType),
	/// 状态钩子。
	Status(StatusType),
}

impl FromStr for HookType {
	type Err = std::convert::Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let parts: Vec<&str> = s.split('.').collect();

		match parts.as_slice() {
			["event"] => Ok(HookType::Event(HookEventType::default())),
			["event", sub_type] => {
				let event_type = HookEventType::from_str(sub_type).unwrap_or_default();
				Ok(HookType::Event(event_type))
			}
			["status"] => Ok(HookType::Status(StatusType::default())),
			["status", sub_type] => {
				let status_type = StatusType::from_str(sub_type).unwrap_or_default();
				Ok(HookType::Status(status_type))
			}
			_ => Ok(HookType::Event(HookEventType::default())),
		}
	}
}

impl Default for HookType {
	fn default() -> Self {
		Self::Event(HookEventType::default())
	}
}
