mod logger;
mod server;
mod adapter;

use puniyu_common::path::CONFIG_DIR;
use puniyu_common::toml;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, LazyLock, RwLock};
use logger::LoggerConfig;
use server::ServerConfig;
use adapter::AdapterConfig;

pub(crate) static APP_CONFIG: LazyLock<Arc<RwLock<AppConfig>>> = LazyLock::new(|| {
	Arc::new(RwLock::new(
		toml::read_config::<AppConfig>(CONFIG_DIR.as_path(), "app").unwrap_or_default(),
	))
});


/// TODO: 此部分交给插件处理
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadConfig {
	/// 强制加载abi版本不同的插件,
	#[serde(default = "default_force_plugin")]
	force_plugin: bool,
}

fn default_force_plugin() -> bool {
	false
}

impl Default for LoadConfig {
	#[inline]
	fn default() -> Self {
		Self { force_plugin: default_force_plugin() }
	}
}

impl LoadConfig {
	/// 是否强制加载abi版本不同的插件
	pub fn force_plugin(&self) -> bool {
		self.force_plugin
	}
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
}
