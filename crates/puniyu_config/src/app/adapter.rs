use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterConfig {
	#[serde(default = "default_adapter")]
	console: bool,
	#[serde(default = "default_adapter")]
	server: bool
}

impl AdapterConfig {
	/// 是否启用控制台适配器
	pub fn console(&self) -> bool {
		self.console
	}
	/// 是否启用服务端适配器
	pub fn server(&self) -> bool {
		self.server
	}
}

impl Default for AdapterConfig {
	#[inline]
	fn default() -> Self {
		Self { console: default_adapter(), server: default_adapter() }
	}
}
fn default_adapter() -> bool {
	true
}
