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
mod adapter_info;
#[doc(inline)]
pub use adapter_info::AdapterInfo;
mod status;
#[doc(inline)]
pub use status::Status;


use std::time::Duration;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use  bon::Builder;
use jiff::Timestamp;


/// Bot 连接信息。
///
/// 描述单个 Bot 实例的连接详情，包括通信方式、地址、连接时间等。
#[derive(Debug, Clone, Builder, Deserialize, Serialize)]
pub struct ConnectionInfo {
	/// 连接状态
	#[builder(default)]
	pub status: Status,
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
		self.status == other.status
			&& self.conn_protocol == other.conn_protocol
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


#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct SendMsgResult {
	/// 消息 ID
	pub message_id: SmolStr,
	/// 发送时间戳，单位为秒
	pub time: Duration,
}

impl SendMsgResult {
    pub fn new(message_id: impl Into<SmolStr>, time: impl Into<Duration>) -> Self {
        Self {
            message_id: message_id.into(),
            time: time.into(),
        }
    }
}
