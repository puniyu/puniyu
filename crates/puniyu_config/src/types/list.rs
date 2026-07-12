use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ListConfig {
	/// 白名单列表
	///
	/// 包含允许通过的项的 ID 列表。
	/// 当此列表不为空时，只有列表中的项才会被处理。
	#[serde(default)]
	enable_list: Vec<SmolStr>,

	/// 黑名单列表
	///
	/// 包含需要屏蔽的项的 ID 列表。
	/// 列表中的项将被明确拒绝处理。
	#[serde(default)]
	disable_list: Vec<SmolStr>,
}

impl ListConfig {
	pub fn enable_list(&self) -> Vec<&str> {
		self.enable_list.iter().map(|s| s.as_str()).collect()
	}

	pub fn disable_list(&self) -> Vec<&str> {
		self.disable_list.iter().map(|s| s.as_str()).collect()
	}
}
