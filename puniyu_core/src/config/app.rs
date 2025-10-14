use puniyu_utils::path::CONFIG_DIR;
use puniyu_utils::toml;
use serde::{Deserialize, Serialize};
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
	pub enable_file: bool,
	/// 日志等级
	#[serde(default = "default_logger_level")]
	pub level: String,
	/// 日志路径
	#[serde(default = "default_logger_path")]
	pub path: String,
	/// 日志保留天数
	#[serde(default = "default_logger_retention_days")]
	pub retention_days: u8,
}

impl Default for LoggerConfig {
	fn default() -> Self {
		Self {
			enable_file: default_logger_file_enable(),
			level: default_logger_level(),
			path: default_logger_path(),
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

fn default_logger_path() -> String {
	String::from("logs")
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
	/// 日志路径
	pub fn path(&self) -> &str {
		self.path.as_str()
	}
	/// 日志保留天数
	pub fn retention_days(&self) -> u8 {
		self.retention_days
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
	/// 服务器主机地址
	#[serde(default = "default_server_host")]
	pub host: String,
	/// 服务器端口号
	#[serde(default = "default_server_port")]
	pub port: u16,
}

fn default_server_host() -> String {
	String::from("0.0.0.0")
}
fn default_server_port() -> u16 {
	33720
}

impl Default for ServerConfig {
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
	pub logger: LoggerConfig,
	/// 服务器配置
	#[serde(default)]
	pub server: ServerConfig,
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
}
