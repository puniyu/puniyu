use puniyu_common::CONFIG_DIR;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, RwLock};

mod option;
use option::FriendOption;

pub(crate) static FRIEND_CONFIG: LazyLock<Arc<RwLock<FriendConfig>>> = LazyLock::new(|| {
	Arc::new(RwLock::new(
		puniyu_common::config::read_config(CONFIG_DIR.as_path(), "friend").unwrap_or_default(),
	))
});

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FriendConfig {
	#[serde(default)]
	global: FriendOption,
	#[serde(default, skip_serializing_if = "HashMap::is_empty")]
	friend: HashMap<String, FriendOption>,
}

impl FriendConfig {
	pub fn get() -> FriendConfig {
		FRIEND_CONFIG.read().unwrap().clone()
	}

	/// 获取全局配置
	pub fn global(&self) -> &FriendOption {
		&self.global
	}

	/// 获取好友配置
	pub fn friend(&self, user_id: &str) -> &FriendOption {
		self.friend.get(user_id).unwrap_or(&self.global)
	}

	/// 获取所有好友配置
	pub fn list(&self) -> Vec<&FriendOption> {
		self.friend.values().collect()
	}
}
