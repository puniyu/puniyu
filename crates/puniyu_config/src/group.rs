use std::{collections::HashMap, path::PathBuf, sync::LazyLock};

use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use crate::Config;
use crate::{
	OptionConfig, OptionConfigRaw,
	common::{MergeWith, read_config},
};

const NAME: &str = "group";

static CONFIG_PATH: LazyLock<PathBuf> =
	LazyLock::new(|| puniyu_path::config_dir().join(NAME).with_extension("toml"));

/// 群组配置结构
///
/// 管理所有群组的配置，支持全局配置和单独群组配置。
///
/// # 配置文件示例
///
/// ```toml
/// [global]
/// cd = 0
/// user_cd = 0
/// mode = 0
/// alias = []
///
/// [group.group_123]
/// cd = 10
/// user_cd = 5
/// mode = 1
/// alias = ["bot"]
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GroupConfig {
	/// 全局群组配置
	///
	/// 作为所有群组的默认配置
	#[serde(default)]
	global: OptionConfigRaw,

	/// 特定群组配置映射
	///
	/// 键为群组 ID，值为该群组的特定配置
	#[serde(default, skip_serializing_if = "HashMap::is_empty")]
	group: HashMap<SmolStr, OptionConfigRaw>,
}

impl GroupConfig {
	pub fn get() -> Self {
		crate::ConfigStore::get(CONFIG_PATH.as_path())
			.unwrap_or_else(|| read_config::<Self>(CONFIG_PATH.as_path()).unwrap_or_default())
	}

	pub fn global(&self) -> OptionConfig {
		self.global.merge_with(&OptionConfig::default())
	}

	pub fn group(&self, group_id: &str) -> OptionConfig {
		self.resolve(group_id, &OptionConfig::default())
	}

	/// 在上层配置基础上解析群组最终配置。
	pub fn resolve(&self, group_id: &str, inherited: &OptionConfig) -> OptionConfig {
		let global = self.global.merge_with(inherited);
		self.group.get(group_id).map(|raw| raw.merge_with(&global)).unwrap_or(global)
	}

	pub fn list(&self) -> HashMap<&str, OptionConfig> {
		let global = self.global();
		self.group.iter().map(|(k, v)| (k.as_str(), v.merge_with(&global))).collect()
	}
}

impl Config for GroupConfig {
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
