use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggerConfig {
	/// 是否启用文件日志记录
	#[serde(default = "default_file_enable")]
	enable_file: bool,
	/// 日志等级
	#[serde(default = "default_logger_level")]
	level: String,
	/// 日志保留天数
	#[serde(default = "default_retention_days")]
	retention_days: u8,
}

impl Default for LoggerConfig {
	#[inline]
	fn default() -> Self {
		Self {
			enable_file: default_file_enable(),
			level: default_logger_level(),
			retention_days: default_retention_days(),
		}
	}
}

fn default_file_enable() -> bool {
	true
}
fn default_logger_level() -> String {
	String::from("info")
}

fn default_retention_days() -> u8 {
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
