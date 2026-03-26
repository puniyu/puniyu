use serde::{Deserialize, Serialize};

/// 日志配置结构
///
/// 定义日志系统的行为，包括日志级别、文件记录和保留策略。
///
/// # 示例
///
/// ```toml
/// [logger]
/// enable_file = true
/// level = "info"
/// retention_days = 7
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggerConfig {
	/// 是否启用文件日志记录
	///
	/// 当设置为 [`true`] 时，日志会同时输出到文件和控制台
	#[serde(default = "default_file_enable")]
	enable_file: bool,

	/// 日志等级
	///
	/// 可选值: "trace", "debug", "info", "warn", "error"
	#[serde(default = "default_logger_level")]
	level: String,

	/// 日志文件保留天数
	///
	/// 超过指定天数的日志文件会被自动删除
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
	///
	/// # 返回值
	///
	/// 返回 `true` 表示启用文件日志，`false` 表示仅输出到控制台
	pub fn enable_file(&self) -> bool {
		self.enable_file
	}

	/// 获取日志等级
	///
	/// # 返回值
	///
	/// 返回日志等级字符串，如 "info", "debug", "warn" 等
	pub fn level(&self) -> &str {
		self.level.as_str()
	}

	/// 获取日志保留天数
	///
	/// # 返回值
	///
	/// 返回日志文件保留的天数，超过此天数的日志会被自动删除
	pub fn retention_days(&self) -> u8 {
		self.retention_days
	}
}
