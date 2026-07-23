use std::{collections::HashMap, path::{Path, PathBuf}};

use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use crate::{
	Config, OptionConfig, OptionConfigRaw,
	common::{MergeWith, read_config},
};

/// 群组配置结构
///
/// 管理所有群组的配置，支持全局配置和单独群组配置。
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GroupConfig {
	/// 全局群组配置
	#[serde(default)]
	global: OptionConfigRaw,

	/// 特定群组配置映射
	#[serde(default, skip_serializing_if = "HashMap::is_empty")]
	group: HashMap<SmolStr, OptionConfigRaw>,

	#[serde(skip)]
	path: PathBuf,
}

impl GroupConfig {
	pub fn from_path(path: impl AsRef<Path>) -> Self {
		let mut config: Self = read_config(&path).unwrap_or_default();
		config.path = path.as_ref().to_path_buf();
		config
	}

	pub fn global(&self) -> OptionConfig {
		self.global.merge_with(&OptionConfig::default())
	}

	pub fn group(&self, group_id: &str) -> OptionConfig {
		let global = self.global();
		self.group.get(group_id).map(|raw| raw.merge_with(&global)).unwrap_or(global)
	}

	pub fn list(&self) -> HashMap<&str, OptionConfig> {
		let global = self.global();
		self.group.iter().map(|(k, v)| (k.as_str(), v.merge_with(&global))).collect()
	}
}

impl Config for GroupConfig {
	fn name(&self) -> &str {
		"group"
	}

	fn path(&self) -> PathBuf {
		self.path.clone()
	}

	fn to_value(&self) -> toml::Value {
		toml::Value::try_from(self).expect("GroupConfig serialization failed")
	}
}
