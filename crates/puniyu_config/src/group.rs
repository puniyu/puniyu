mod option;

use puniyu_common::path::CONFIG_DIR;
use puniyu_common::toml;
use serde::{Deserialize, Serialize};
use std::{
	collections::HashMap,
	sync::{Arc, LazyLock, RwLock},
};
use option::GroupOption;

pub(crate) static GROUP_CONFIG: LazyLock<Arc<RwLock<GroupConfig>>> = LazyLock::new(|| {
	Arc::new(RwLock::new(toml::read_config(CONFIG_DIR.as_path(), "group").unwrap_or_default()))
});


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GroupConfig {
	#[serde(default)]
	global: GroupOption,
	#[serde(default, skip_serializing_if = "HashMap::is_empty")]
	group: HashMap<String, GroupOption>,
}



impl GroupConfig {
	/// 根据群组配置, 包括全局和所有群组的配置
	pub fn get() -> Self {
		GROUP_CONFIG.read().unwrap().clone()
	}

	/// 获取指定群组配置
	pub fn group(&self, group_id: &str) -> GroupOption {
		let config = GROUP_CONFIG.read().unwrap();
		config.group.get(group_id).cloned().unwrap_or_default()
	}

	/// 获取所有群组配置
	pub fn groups(&self) -> Vec<GroupOption> {
		let config = GROUP_CONFIG.read().unwrap();
		config.group.values().cloned().collect()
	}

	/// 获取全局群组配置
	pub fn global(&self) -> GroupOption {
		let config = GROUP_CONFIG.read().unwrap();
		config.global.clone()
	}
}
