use std::{collections::HashMap, path::{Path, PathBuf}};

use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use crate::{
	Config, OptionConfig, OptionConfigRaw,
	common::{MergeWith, read_config},
};

/// 好友配置结构
///
/// 管理所有好友的配置，支持全局配置和单独好友配置。
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FriendConfig {
	/// 全局好友配置
	#[serde(default)]
	global: OptionConfigRaw,

	/// 特定好友配置映射
	#[serde(default, skip_serializing_if = "HashMap::is_empty")]
	friend: HashMap<SmolStr, OptionConfigRaw>,

	#[serde(skip)]
	path: PathBuf,
}

impl FriendConfig {
	pub fn from_path(path: impl AsRef<Path>) -> Self {
		let mut config: Self = read_config(&path).unwrap_or_default();
		config.path = path.as_ref().to_path_buf();
		config
	}

	pub fn global(&self) -> OptionConfig {
		self.global.merge_with(&OptionConfig::default())
	}

	pub fn friend(&self, user_id: &str) -> OptionConfig {
		let global = self.global();
		self.friend.get(user_id).map(|raw| raw.merge_with(&global)).unwrap_or(global)
	}

	pub fn list(&self) -> HashMap<&str, OptionConfig> {
		let global = self.global();
		self.friend.iter().map(|(k, v)| (k.as_str(), v.merge_with(&global))).collect()
	}
}

impl Config for FriendConfig {
	fn name(&self) -> &str {
		"friend"
	}

	fn path(&self) -> PathBuf {
		self.path.clone()
	}

	fn to_value(&self) -> toml::Value {
		toml::Value::try_from(self).expect("FriendConfig serialization failed")
	}
}
