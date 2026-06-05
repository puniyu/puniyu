mod logger;
pub use logger::LoggerConfig;
use puniyu_core::toml;

pub(crate) fn serialize_to_value<T: serde::Serialize>(config: &T) -> toml::Value {
	toml::Value::try_from(config).expect("Failed to serialize config to toml::Value")
}