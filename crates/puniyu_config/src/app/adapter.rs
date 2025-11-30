use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterConfig {
	#[serde(default = "default_console")]
	console: bool,
}

impl AdapterConfig {
	/// 是否启用控制台适配器
	pub fn console(&self) -> bool {
		self.console
	}
}

impl Default for AdapterConfig {
	#[inline]
	fn default() -> Self {
		Self {
			console: default_console(),
		}
	}
}
fn default_console() -> bool {
	true
}
