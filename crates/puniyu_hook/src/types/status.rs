use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::{Display, IntoStaticStr};

/// 状态钩子类型。
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
	/// 启动状态。
	#[default]
	Start,
	/// 停止状态。
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
