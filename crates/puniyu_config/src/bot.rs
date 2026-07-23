use std::{collections::HashMap, path::{Path, PathBuf}};

use crate::{
	Config, OptionConfig, OptionConfigRaw,
	common::{MergeWith, read_config},
};
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

/// Bot 配置结构
///
/// 管理所有 Bot 实例的配置，支持全局配置和单独 Bot 配置。
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BotConfig {
	/// 全局 Bot 配置
	#[serde(default)]
	global: OptionConfigRaw,

	/// 特定 Bot 配置映射
	#[serde(default, skip_serializing_if = "HashMap::is_empty")]
	bot: HashMap<SmolStr, OptionConfigRaw>,

	#[serde(skip)]
	path: PathBuf,
}

impl BotConfig {
	pub fn from_path(path: impl AsRef<Path>) -> Self {
		let mut config: Self = read_config(&path).unwrap_or_default();
		config.path = path.as_ref().to_path_buf();
		config
	}

	pub fn global(&self) -> OptionConfig {
		self.global.merge_with(&OptionConfig::default())
	}

	pub fn bot(&self, bot_id: &str) -> OptionConfig {
		let global = self.global();
		self.bot.get(bot_id).map(|raw| raw.merge_with(&global)).unwrap_or(global)
	}

	pub fn list(&self) -> HashMap<&str, OptionConfig> {
		let global = self.global();
		self.bot.iter().map(|(k, v)| (k.as_str(), v.merge_with(&global))).collect()
	}
}

impl Config for BotConfig {
	fn name(&self) -> &str {
		"bot"
	}

	fn path(&self) -> PathBuf {
		self.path.clone()
	}

	fn to_value(&self) -> toml::Value {
		toml::Value::try_from(self).expect("BotConfig serialization failed")
	}
}
