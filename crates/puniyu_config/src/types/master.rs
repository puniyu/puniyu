use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use std::collections::HashMap;

/// Bot Master 配置。
///
/// 以 Adapter 名称为键，保存该 Adapter 下拥有 Master 权限的用户 ID
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MasterConfig(HashMap<SmolStr, Vec<SmolStr>>);

impl MasterConfig {
	/// 获取指定 Adapter 的 Master 用户 ID 列表。
	///
	/// Adapter 未配置时返回空列表。
	pub fn get(&self, adapter_name: &str) -> Vec<SmolStr> {
		self.0.get(adapter_name).cloned().unwrap_or_default()
	}
}