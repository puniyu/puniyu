mod logger;
mod server;
mod adapter;
mod group;
mod friend;

use puniyu_common::path::CONFIG_DIR;
use puniyu_common::toml;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, LazyLock, RwLock};
use logger::LoggerConfig;
use server::ServerConfig;
use adapter::AdapterConfig;
use group::GroupConfig;
use friend::FriendConfig;

pub(crate) static APP_CONFIG: LazyLock<Arc<RwLock<AppConfig>>> = LazyLock::new(|| {
	Arc::new(RwLock::new(
		toml::read_config::<AppConfig>(CONFIG_DIR.as_path(), "app").unwrap_or_default(),
	))
});


fn default_master() -> Vec<String> {
	vec!["console".to_string()]
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AppConfig {
	/// 日志配置
	#[serde(default)]
	logger: LoggerConfig,
	/// 服务器配置
	#[serde(default)]
	server: ServerConfig,
	/// 适配器配置
	#[serde(default)]
	adapter: AdapterConfig,
	/// 群组配置
	#[serde(default)]
	group: GroupConfig,
	/// 好友配置
	#[serde(default)]
	friend: FriendConfig,
	/// Bot主人列表
	#[serde(default = "default_master")]
	masters: Vec<String>,
}

impl AppConfig {
	pub fn get() -> Self {
		APP_CONFIG.read().unwrap().clone()
	}
	/// 获取日志配置
	pub fn logger(&self) -> LoggerConfig {
		let config = APP_CONFIG.read().unwrap();
		config.logger.clone()
	}
	/// 获取服务器配置
	pub fn server(&self) -> ServerConfig {
		let config = APP_CONFIG.read().unwrap();
		config.server.clone()
	}
	/// 获取适配器配置
	pub fn adapter(&self) -> AdapterConfig {
		let config = APP_CONFIG.read().unwrap();
		config.adapter.clone()
	}
	
	/// 获取bot主人列表
	pub fn masters() -> Vec<String> {
		let config = APP_CONFIG.read().unwrap();
		config.masters.clone()
	}
	
	/// 获取应用群组配置
	///
	/// 包含群聊黑白名单
	pub fn group(&self) -> GroupConfig {
		let config = APP_CONFIG.read().unwrap();
		config.group.clone()
	}
	
	/// 获取应用好友配置
	///
	/// 包含好友黑白名单
	pub fn friend(&self) -> FriendConfig {
		let config = APP_CONFIG.read().unwrap();
		config.friend.clone()
	}
}
