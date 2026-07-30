use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

#[derive(
	Debug,
	Default,
	Clone,
	Copy,
	PartialEq,
	Eq,
	Display,
	Deserialize,
	Serialize,
	EnumString,
	IntoStaticStr,
)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
/// 连接状态
pub enum Status {
	/// 未连接
	#[default]
	Offline,
	/// 正在连接
	Connecting,
	/// 在线
	Online,
	/// 已断开
	Disconnect,
	/// 重连中
	Reconnecting,
}
