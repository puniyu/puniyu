use log::Level;
use serde::{Deserialize, Serialize};

const fn default_file_enable() -> bool {
	true
}

const fn default_logger_level() -> Level {
	Level::Info
}

fn deserialize_level<'de, D>(d: D) -> Result<Level, D::Error>
where
	D: serde::Deserializer<'de>,
{
	let s = String::deserialize(d)?;
	s.parse::<Level>().map_err(serde::de::Error::custom)
}

fn serialize_level<S>(level: &Level, s: S) -> Result<S::Ok, S::Error>
where
	S: serde::Serializer,
{
	s.serialize_str(&level.to_string())
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