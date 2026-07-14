use std::{collections::HashMap, path::PathBuf, sync::LazyLock};

use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use crate::Config;
use crate::{
	OptionConfig, OptionConfigRaw,
	common::{MergeWith, read_config},
};
const NAME: &str = "friend";

static CONFIG_PATH: LazyLock<PathBuf> =
	LazyLock::new(|| puniyu_path::config_dir().join(NAME).with_extension("toml"));

/// 好友配置结构
///
/// 管理所有好友的配置，支持全局配置和单独好友配置。
///
/// # 配置文件示例
///
/// ```toml
/// [global]
/// cd = 0
/// mode = 0
/// alias = []
///
/// [friend.user_123]
/// cd = 5
/// mode = 1
/// alias = ["bot"]
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FriendConfig {
	/// 全局好友配置
	///
	/// 作为所有好友的默认配置
	#[serde(default)]
	global: OptionConfigRaw,

	/// 特定好友配置映射
	///
	/// 键为用户 ID，值为该好友的特定配置
	#[serde(default, skip_serializing_if = "HashMap::is_empty")]
	friend: HashMap<SmolStr, OptionConfigRaw>,
}

impl FriendConfig {
	pub fn get() -> Self {
		crate::ConfigRegistry::get(CONFIG_PATH.as_path())
			.unwrap_or_else(|| read_config::<Self>(CONFIG_PATH.as_path()).unwrap_or_default())
	}

	pub fn global(&self) -> OptionConfig {
		self.global.merge_with(&OptionConfig::default())
	}

	pub fn friend(&self, user_id: &str) -> OptionConfig {
		self.resolve(user_id, &OptionConfig::default())
	}

	/// 在上层配置基础上解析好友最终配置。
	pub fn resolve(&self, user_id: &str, inherited: &OptionConfig) -> OptionConfig {
		let global = self.global.merge_with(inherited);
		self.friend.get(user_id).map(|raw| raw.merge_with(&global)).unwrap_or(global)
	}

	pub fn list(&self) -> HashMap<&str, OptionConfig> {
		let global = self.global();
		self.friend.iter().map(|(k, v)| (k.as_str(), v.merge_with(&global))).collect()
	}
}

impl Config for FriendConfig {
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
