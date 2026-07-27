mod connection;
#[doc(inline)]
pub use connection::ConnProtocol;
mod platform;
#[doc(inline)]
pub use platform::Platform;
mod protocol;
#[doc(inline)]
pub use protocol::Protocol;
mod standard;
#[doc(inline)]
pub use standard::Standard;

use semver::Version;
use smol_str::SmolStr;
use std::{borrow::Cow, time::Duration};
use bon::Builder;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

/// 机器人标识符。
///
/// 用于在 [`BotRegistry`](crate::BotRegistry) 中按注册索引或机器人 UIN 定位实例。
///
/// # 示例
///
/// ```rust
/// use puniyu_bot::BotId;
///
/// let index: BotId = 123u64.into();
/// let self_id: BotId = "123456".into();
///
/// assert_eq!(index, BotId::Index(123));
/// assert_eq!(self_id, BotId::SelfId("123456".into()));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BotId<'b> {
	/// 注册表索引
	Index(u64),
	/// 机器人 UIN
	SelfId(Cow<'b, str>),
}

impl From<u64> for BotId<'_> {
	fn from(index: u64) -> Self {
		Self::Index(index)
	}
}

impl<'b> From<&'b str> for BotId<'b> {
	fn from(name: &'b str) -> Self {
		Self::SelfId(Cow::Borrowed(name))
	}
}

impl From<String> for BotId<'_> {
	fn from(name: String) -> Self {
		Self::SelfId(Cow::Owned(name))
	}
}


/// Bot 连接信息。
///
/// 描述单个 Bot 实例的连接详情，包括通信方式、地址、连接时间等。
#[derive(Debug, Clone, Builder, Deserialize, Serialize)]
pub struct ConnectionInfo {
	/// 通信方式
	#[builder(default)]
	pub conn_protocol: ConnProtocol,
	/// 连接地址
	pub address: Option<String>,
	/// 连接时间
	#[builder(default = ConnectionInfo::default_connect_time())]
	pub connect_time: Timestamp,
	/// 鉴权密钥
	pub secret: Option<String>,
}

impl PartialEq for ConnectionInfo {
	fn eq(&self, other: &Self) -> bool {
		self.conn_protocol == other.conn_protocol
			&& self.address == other.address
			&& self.connect_time == other.connect_time
			&& self.secret == other.secret
	}
}

impl Eq for ConnectionInfo {}

impl ConnectionInfo {
	fn default_connect_time() -> Timestamp {
		Timestamp::now()
	}
}

/// 适配器元信息。
///
/// # 示例
///
/// ```rust
/// use puniyu_bot::{AdapterInfo, Platform, Protocol, Standard};
/// use semver::Version;
///
/// let info = AdapterInfo {
///     name: "onebot".into(),
///     version: Version::new(1, 0, 0),
///     author: vec!["Puniyu".into()],
///     description: Some("OneBot 协议适配器".into()),
///     platform: Platform::QQ,
///     protocol: Protocol::NapCat,
///     standard: Standard::OneBotV11,
/// };
///
/// assert_eq!(info.name, "onebot");
/// assert_eq!(info.platform, Platform::QQ);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Builder)]
pub struct AdapterInfo {
	/// 适配器名称（如 "onebot"、"telegram"）
	#[builder(into)]
	pub name: SmolStr,
	/// 适配器版本
	pub version: Version,
	/// 适配器作者列表
	#[builder(into)]
	pub author: Vec<SmolStr>,
	/// 适配器描述
	#[builder(into)]
	pub description: Option<SmolStr>,
	/// 适配器平台
	pub platform: Platform,
	/// 适配器协议实现
	pub protocol: Protocol,
	/// 适配器标准
	pub standard: Standard,
}


#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct SendMsgResult {
	/// 消息 ID
	pub message_id: SmolStr,
	/// 发送时间戳，单位为秒
	pub time: Duration,
}