mod option;

pub use option::GroupOption;
use puniyu_common::read_config;
use puniyu_path::CONFIG_DIR;
use serde::{Deserialize, Serialize};
use std::{
	collections::HashMap,
	sync::{Arc, LazyLock, RwLock},
};

pub(crate) static GROUP_CONFIG: LazyLock<Arc<RwLock<GroupConfig>>> = LazyLock::new(|| {
	Arc::new(RwLock::new(read_config(CONFIG_DIR.as_path(), "group").unwrap_or_default()))
});

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
	/// 返回当前的群组配置副本
	pub fn get() -> Self {
		GROUP_CONFIG.read().expect("Failed to read group config").clone()
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
