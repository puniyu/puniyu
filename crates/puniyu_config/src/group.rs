use puniyu_common::path::CONFIG_DIR;
use puniyu_common::toml;
use serde::{Deserialize, Serialize};
use std::{
	collections::HashMap,
	sync::{Arc, LazyLock, RwLock},
};

pub(crate) static GROUP_CONFIG: LazyLock<Arc<RwLock<GroupConfig>>> = LazyLock::new(|| {
	Arc::new(RwLock::new(toml::read_config(CONFIG_DIR.as_path(), "group").unwrap_or_default()))
});
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupConfigOption {
	/// 全局cd冷却时间
	#[serde(default = "default_group_cd")]
	pub cd: u8,
	/// 用户cd冷却时间
	#[serde(default = "default_group_user_cd")]
	pub user_cd: u8,
}

fn default_group_cd() -> u8 {
	0
}

fn default_group_user_cd() -> u8 {
	0
}

impl GroupConfigOption {
	pub fn cd(&self) -> u8 {
		self.cd
	}
	pub fn user_cd(&self) -> u8 {
		self.user_cd
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GroupConfig {
	#[serde(default)]
	global: GroupConfigOption,
	#[serde(default, skip_serializing_if = "HashMap::is_empty")]
	group: HashMap<String, GroupConfigOption>,
}

impl Default for GroupConfigOption {
	fn default() -> Self {
		Self { cd: default_group_cd(), user_cd: default_group_user_cd() }
	}
}

impl GroupConfig {
	/// 根据群组配置, 包括全局和所有群组的配置
	pub fn get() -> Self {
		GROUP_CONFIG.read().unwrap().clone()
	}

	/// 获取指定群组配置
	pub fn group(&self, group_id: &str) -> GroupConfigOption {
		let config = GROUP_CONFIG.read().unwrap();
		config.group.get(group_id).cloned().unwrap_or_default()
	}

	/// 获取所有群组配置
	pub fn groups(&self) -> Vec<GroupConfigOption> {
		let config = GROUP_CONFIG.read().unwrap();
		config.group.values().cloned().collect()
	}

	/// 获取全局群组配置
	pub fn global(&self) -> GroupConfigOption {
		let config = GROUP_CONFIG.read().unwrap();
		config.global.clone()
	}
}
