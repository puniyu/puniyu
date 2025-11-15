use puniyu_common::path::CONFIG_DIR;
use puniyu_common::toml;
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;
use std::sync::{Arc, LazyLock, RwLock};

pub(crate) static APP_CONFIG: LazyLock<Arc<RwLock<AppConfig>>> = LazyLock::new(|| {
	Arc::new(RwLock::new(
		toml::read_config::<AppConfig>(CONFIG_DIR.as_path(), "app").unwrap_or_default(),
	))
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggerConfig {
	/// 是否启用文件日志记录
	#[serde(default = "default_logger_file_enable")]
	enable_file: bool,
	/// 日志等级
	#[serde(default = "default_logger_level")]
	level: String,
	/// 日志保留天数
	#[serde(default = "default_logger_retention_days")]
	retention_days: u8,
}

impl Default for LoggerConfig {
	#[inline]
	fn default() -> Self {
		Self {
			enable_file: default_logger_file_enable(),
			level: default_logger_level(),
			retention_days: default_logger_retention_days(),
		}
	}
}

fn default_logger_file_enable() -> bool {
	true
}
fn default_logger_level() -> String {
	String::from("info")
}

fn default_logger_retention_days() -> u8 {
	7
}

impl LoggerConfig {
	/// 是否启用文件日志记录
	pub fn enable_file(&self) -> bool {
		self.enable_file
	}
	/// 日志等级
	pub fn level(&self) -> &str {
		self.level.as_str()
	}
	/// 日志保留天数
	pub fn retention_days(&self) -> u8 {
		self.retention_days
	}
}

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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
	/// 服务器主机地址
	#[serde(default = "default_server_host")]
	host: String,
	/// 服务器端口号
	#[serde(default = "default_server_port")]
	port: u16,
}

fn default_server_host() -> String {
	Ipv4Addr::new(127, 0, 0, 1).to_string()
}
fn default_server_port() -> u16 {
	33720
}

impl Default for ServerConfig {
	#[inline]
	fn default() -> Self {
		Self { host: default_server_host(), port: default_server_port() }
	}
}

impl ServerConfig {
	/// 获取服务器主机地址
	pub fn host(&self) -> &str {
		self.host.as_str()
	}
	/// 获取服务器端口
	pub fn port(&self) -> u16 {
		self.port
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
	/// 加载配置
	#[serde(default)]
	load: LoadConfig,
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

	/// 获取加载配置
	pub fn load(&self) -> LoadConfig {
		let config = APP_CONFIG.read().unwrap();
		config.load.clone()
	}
}
