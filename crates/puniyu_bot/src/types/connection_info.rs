use bon::Builder;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use super::ConnProtocol;

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
