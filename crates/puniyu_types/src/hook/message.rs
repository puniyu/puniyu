use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

#[derive(
	Debug,
	Clone,
	Default,
	EnumString,
	Display,
	IntoStaticStr,
	Deserialize,
	Serialize,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
)]
pub enum HookMessageType {
	/// 好友消息
	#[strum(serialize = "friend")]
	Friend,
	/// 群消息
	#[strum(serialize = "group")]
	Group,
	/// 频道消息
	#[strum(serialize = "guild")]
	Guild,
	/// 全部消息
	#[strum(serialize = "all")]
	#[default]
	All,
}
