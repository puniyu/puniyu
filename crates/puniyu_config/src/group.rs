use crate::GroupOption;
use puniyu_common::read_config;
use puniyu_path::config_dir;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf, sync::LazyLock};

static CONFIG_PATH: LazyLock<PathBuf> = LazyLock::new(|| config_dir().join("group.toml"));

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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GroupConfig {
	/// 全局群组配置
	///
	/// 作为所有群组的默认配置
	#[serde(default)]
	global: GroupOption,

	/// 特定群组配置映射
	///
	/// 键为群组 ID，值为该群组的特定配置
	#[serde(default, skip_serializing_if = "HashMap::is_empty")]
	group: HashMap<String, GroupOption>,
}

impl GroupConfig {
	/// 获取群组配置实例
	///
	/// # 返回值
	///
	/// 返回当前的群组配置副本，从注册表获取
	pub fn get() -> Self {
		use crate::ConfigRegistry;
		ConfigRegistry::get(CONFIG_PATH.as_path()).and_then(|v| v.try_into().ok()).unwrap_or_else(
			|| read_config::<Self>(config_dir().as_path(), "group").unwrap_or_default(),
		)
	}

	/// 获取全局群组配置
	///
	/// # 返回值
	///
	/// 返回全局群组配置的引用，作为所有群组的默认配置
	pub fn global(&self) -> &GroupOption {
		&self.global
	}

	/// 获取指定群组的配置
	///
	/// # 参数
	///
	/// - `group_id`: 群组的唯一标识符
	///
	/// # 返回值
	///
	/// 返回对应群组的配置。特定群组配置会自动继承全局配置，
	/// 只有显式设置的字段会覆盖全局配置
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_config::Config;
	///
	/// let group_config = Config::group();
	/// // 如果 group_123 只设置了 cd，其他字段会继承全局配置
	/// let group_option = group_config.group("group_123");
	/// println!("群组 CD: {}", group_option.cd());
	/// println!("用户 CD: {}", group_option.user_cd());
	/// ```
	pub fn group(&self, group_id: &str) -> GroupOption {
		self.group
			.get(group_id)
			.map(|specific| specific.merge_with(&self.global))
			.unwrap_or_else(|| self.global.clone())
	}

	/// 获取所有群组配置列表
	///
	/// # 返回值
	///
	/// 返回包含所有群组配置的 Vec，每个配置都已与全局配置合并
	pub fn list(&self) -> Vec<GroupOption> {
		self.group.values().map(|specific| specific.merge_with(&self.global)).collect()
	}
}

impl crate::Config for GroupConfig {
	fn config(&self) -> crate::ConfigInfo {
		crate::ConfigInfo {
			name: "group".to_string(),
			path: CONFIG_PATH.clone(),
			value: toml::to_string(self)
				.expect("Failed to serialize GroupConfig to TOML string")
				.parse()
				.expect("Failed to parse TOML string to Value"),
		}
	}
}
