mod option;

use option::GroupOption;
use puniyu_common::path::CONFIG_DIR;
use puniyu_common::config;
use serde::{Deserialize, Serialize};
use std::{
	collections::HashMap,
	sync::{Arc, LazyLock, RwLock},
};

pub(crate) static GROUP_CONFIG: LazyLock<Arc<RwLock<GroupConfig>>> = LazyLock::new(|| {
	Arc::new(RwLock::new(config::read_config(CONFIG_DIR.as_path(), "group").unwrap_or_default()))
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

	/// 获取全局群组配置
	pub fn global(&self) -> &GroupOption {
		&self.global
	}

	/// 获取指定群组配置
	pub fn group(&self, group_id: &str) -> &GroupOption {
		self.group.get(group_id).unwrap_or(&self.global)
	}

	/// 获取所有群组配置
	pub fn list(&self) -> Vec<&GroupOption> {
		self.group.values().collect()
	}
}
