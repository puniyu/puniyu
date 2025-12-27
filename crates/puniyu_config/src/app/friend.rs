use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FriendConfig {
	/// 好友白名单
	#[serde(default)]
	enable_list: Vec<String>,
	/// 好友黑名单
	#[serde(default)]
	disable_list: Vec<String>,
}

impl FriendConfig {
	/// 获取好友白名单
	pub fn enable_list(&self) -> Vec<String> {
		self.enable_list.clone()
	}

	/// 获取好友黑名单
	pub fn disable_list(&self) -> Vec<String> {
		self.disable_list.clone()
	}
}
