use std::net::Ipv4Addr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
	/// 服务器主机地址
	#[serde(default = "default_server_host")]
	host: String,
	/// 服务器端口号
	#[serde(default = "default_server_port")]
	port: u16,
}

fn default_server_host() -> String {
	Ipv4Addr::new(127, 0, 0, 1).to_string()
}
fn default_server_port() -> u16 {
	33720
}

impl Default for ServerConfig {
	#[inline]
	fn default() -> Self {
		Self { host: default_server_host(), port: default_server_port() }
	}
}

impl ServerConfig {
	/// 获取服务器主机地址
	pub fn host(&self) -> &str {
		self.host.as_str()
	}
	/// 获取服务器端口
	pub fn port(&self) -> u16 {
		self.port
	}
}
