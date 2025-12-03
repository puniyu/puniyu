mod adapter;
mod friend;
mod group;
mod logger;
mod server;

use adapter::AdapterConfig;
use friend::FriendConfig;
use group::GroupConfig;
use logger::LoggerConfig;
use puniyu_common::path::CONFIG_DIR;
use puniyu_common::toml;
use serde::{Deserialize, Serialize};
use server::ServerConfig;
use std::sync::{Arc, LazyLock, RwLock};

pub(crate) static APP_CONFIG: LazyLock<Arc<RwLock<AppConfig>>> = LazyLock::new(|| {
	Arc::new(RwLock::new(
		toml::read_config::<AppConfig>(CONFIG_DIR.as_path(), "app").unwrap_or_default(),
	))
});

fn default_master() -> Vec<String> {
	vec!["console".to_string()]
}

fn default_prefix() -> String {
	String::from("!")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
	/// 全局前缀
	#[serde(default = "default_prefix")]
	prefix: String,
}

impl Default for AppConfig {
	fn default() -> Self {
		AppConfig {
			logger: LoggerConfig::default(),
			server: ServerConfig::default(),
			adapter: AdapterConfig::default(),
			group: GroupConfig::default(),
			friend: FriendConfig::default(),
			masters: default_master(),
			prefix: default_prefix(),
		}
	}
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
	pub fn masters(&self) -> Vec<String> {
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
	/// 获取前缀
	pub fn prefix(&self) -> String {
		let config = APP_CONFIG.read().unwrap();
		config.prefix.clone()
	}
}
