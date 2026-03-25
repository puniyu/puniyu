use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::{Display, IntoStaticStr};

/// 钩子状态类型枚举
///
/// 定义钩子可以监听的状态变化类型。
///
/// # 变体
///
/// - `Start` - 启动状态（默认）
/// - `Stop` - 停止状态
///
/// # 示例
///
/// ```rust
/// use puniyu_hook::types::StatusType;
/// use std::str::FromStr;
///
/// let status = StatusType::Start;
/// assert_eq!(status.to_string(), "Start");
///
/// let status = StatusType::from_str("stop").unwrap();
/// assert_eq!(status, StatusType::Stop);
/// ```
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
pub enum StatusType {
	/// 启动状态（默认）
	#[default]
	Start,
	/// 停止状态
	Stop,
}

impl FromStr for StatusType {
	type Err = std::convert::Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"start" => Ok(Self::Start),
			"stop" => Ok(Self::Stop),
			_ => Ok(Self::default()),
		}
	}
}
