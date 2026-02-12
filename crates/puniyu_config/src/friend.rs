use puniyu_common::read_config;
use puniyu_path::CONFIG_DIR;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, RwLock};

mod option;
use option::FriendOption;

pub(crate) static FRIEND_CONFIG: LazyLock<Arc<RwLock<FriendConfig>>> = LazyLock::new(|| {
	Arc::new(RwLock::new(read_config(CONFIG_DIR.as_path(), "friend").unwrap_or_default()))
});

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
	global: FriendOption,

	/// 特定好友配置映射
	///
	/// 键为用户 ID，值为该好友的特定配置
	#[serde(default, skip_serializing_if = "HashMap::is_empty")]
	friend: HashMap<String, FriendOption>,
}

impl FriendConfig {
	/// 获取好友配置实例
	///
	/// # 返回值
	///
	/// 返回当前的好友配置副本
	pub fn get() -> FriendConfig {
		FRIEND_CONFIG.read().expect("Failed to acquire lock").clone()
	}

	/// 获取全局好友配置
	///
	/// # 返回值
	///
	/// 返回全局好友配置的引用，作为所有好友的默认配置
	pub fn global(&self) -> &FriendOption {
		&self.global
	}

	/// 获取指定好友的配置
	///
	/// # 参数
	///
	/// - `user_id`: 用户的唯一标识符
	///
	/// # 返回值
	///
	/// 返回对应好友的配置。特定好友配置会自动继承全局配置，
	/// 只有显式设置的字段会覆盖全局配置
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_config::Config;
	///
	/// let friend_config = Config::friend();
	/// // 如果 user_123 只设置了 cd，其他字段会继承全局配置
	/// let friend_option = friend_config.friend("user_123");
	/// println!("好友 CD: {}", friend_option.cd());
	/// ```
	pub fn friend(&self, user_id: &str) -> FriendOption {
		self.friend
			.get(user_id)
			.map(|specific| specific.merge_with(&self.global))
			.unwrap_or_else(|| self.global.clone())
	}

	/// 获取所有好友配置列表
	///
	/// # 返回值
	///
	/// 返回包含所有好友配置的 Vec，每个配置都已与全局配置合并
	pub fn list(&self) -> Vec<FriendOption> {
		self.friend.values().map(|specific| specific.merge_with(&self.global)).collect()
	}
}
