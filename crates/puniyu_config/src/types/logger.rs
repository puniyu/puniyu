use log::Level;
use serde::{Deserialize, Serialize};

const fn default_file_enable() -> bool {
	true
}

const fn default_logger_level() -> Level {
	Level::Info
}

fn deserialize_level<'de, D>(deserializer: D) -> Result<Level, D::Error>
where
	D: serde::Deserializer<'de>,
{
	let level = String::deserialize(deserializer)?;
	level.parse().map_err(serde::de::Error::custom)
}

fn serialize_level<S>(level: &Level, serializer: S) -> Result<S::Ok, S::Error>
where
	S: serde::Serializer,
{
	serializer.serialize_str(level.as_str())
}

const fn default_retention_days() -> u8 {
	8
}

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
	#[serde(
		default = "default_logger_level",
		deserialize_with = "deserialize_level",
		serialize_with = "serialize_level"
	)]
	level: Level,

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

impl LoggerConfig {
	/// 是否启用文件日志记录。
	pub fn enable_file(&self) -> bool {
		self.enable_file
	}

	/// 获取日志等级。
	pub fn level(&self) -> Level {
		self.level
	}

	/// 获取日志文件保留天数。
	pub fn retention_days(&self) -> u8 {
		self.retention_days
	}
}
