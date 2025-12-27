use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GroupConfig {
	/// 群白名单
	#[serde(default)]
	enable_list: Vec<String>,
	/// 群黑名单
	#[serde(default)]
	disable_list: Vec<String>,
}

impl GroupConfig {
	/// 获取群白名单
	pub fn enable_list(&self) -> Vec<String> {
		self.enable_list.clone()
	}

	/// 获取群黑名单
	pub fn disable_list(&self) -> Vec<String> {
		self.disable_list.clone()
	}
}
