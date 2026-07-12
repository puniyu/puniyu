use std::{collections::HashMap, path::PathBuf, sync::LazyLock};

use crate::Config;
use crate::{
	OptionConfig, OptionConfigRaw,
	common::{MergeWith, read_config},
};
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

const NAME: &str = "bot";

static CONFIG_PATH: LazyLock<PathBuf> =
	LazyLock::new(|| puniyu_path::config_dir().join(NAME).with_extension("toml"));

/// Bot 配置结构
///
/// 管理所有 Bot 实例的配置，支持全局配置和单独 Bot 配置。
///
/// # 配置文件示例
///
/// ```toml
/// [global]
/// cd = 0
/// mode = 0
/// alias = []
///
/// [bot.bot_001]
/// cd = 5
/// mode = 1
/// alias = ["小助手", "bot"]
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BotConfig {
	/// 全局 Bot 配置
	///
	/// 作为所有 Bot 的默认配置
	#[serde(default)]
	global: OptionConfig,

	/// 特定 Bot 配置映射
	///
	/// 键为 Bot ID，值为该 Bot 的特定配置
	#[serde(default, skip_serializing_if = "HashMap::is_empty")]
	bot: HashMap<SmolStr, OptionConfigRaw>,
}

impl BotConfig {
	pub fn get() -> Self {
		crate::ConfigRegistry::get(CONFIG_PATH.as_path())
			.unwrap_or_else(|| read_config(CONFIG_PATH.as_path()).unwrap_or_default())
	}

	pub fn global(&self) -> OptionConfig {
		self.global.clone()
	}

	pub fn bot(&self, bot_id: &str) -> OptionConfig {
		self.bot
			.get(bot_id)
			.map(|raw| raw.merge_with(&self.global))
			.unwrap_or_else(|| self.global.clone())
	}

	pub fn list(&self) -> HashMap<&str, OptionConfig> {
		self.bot.iter().map(|(k, v)| (k.as_str(), v.merge_with(&self.global))).collect()
	}
}

impl Config for BotConfig {
	fn name(&self) -> &str {
		NAME
	}

	fn path(&self) -> PathBuf {
		puniyu_path::config_dir()
	}

	#[allow(clippy::unwrap_used)]
	#[inline]
	fn value(&self) -> toml::Value {
		toml::Value::try_from(self).unwrap()
	}
}
