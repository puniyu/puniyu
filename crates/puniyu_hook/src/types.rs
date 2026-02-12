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

/// 钩子类型枚举
///
/// 定义钩子的主要类型，用于确定钩子的触发时机。
///
/// # 变体
///
/// - `Event` - 事件钩子，在特定事件发生时触发
/// - `Status` - 状态钩子，在状态变化时触发
///
/// # 示例
///
/// ## 基本使用
///
/// ```rust, ignore
/// use puniyu_hook::types::{HookType, HookEventType};
///
/// let hook_type = HookType::Event(HookEventType::Message);
/// assert_eq!(hook_type.to_string(), "Event(Message)");
/// ```
///
/// ## 从字符串解析
///
/// ```rust
/// use puniyu_hook::types::HookType;
/// use std::str::FromStr;
///
/// // 解析事件类型
/// let hook_type = HookType::from_str("event.message").unwrap();
///
/// // 解析状态类型
/// let hook_type = HookType::from_str("status.start").unwrap();
/// ```
#[derive(
	Debug, Clone, Display, IntoStaticStr, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord,
)]
pub enum HookType {
	/// 事件钩子
	Event(HookEventType),
	/// 状态钩子
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
		HookType::Event(HookEventType::default())
	}
}
