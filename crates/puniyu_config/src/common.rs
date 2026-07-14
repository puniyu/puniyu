use std::{fs, path::Path};

use crate::Error;
use serde::Deserialize;

/// 读取配置文件
///
/// # 示例
///
/// ```rust, ignore
/// use serde::Deserialize;
/// use std::path::Path;
/// use puniyu_config::common::read_config;
///
/// #[derive(Deserialize)]
/// struct Config {
///     name: String,
///     port: u16,
/// }
///
/// let config: Config = read_config(Path::new("./config/app")).unwrap();
/// ```
pub fn read_config<D>(path: impl AsRef<Path>) -> Result<D, Error>
where
	D: for<'de> Deserialize<'de>,
{
	let config_str = fs::read_to_string(path)?;
	Ok(toml::from_str(&config_str)?)
}

pub(crate) trait MergeWith {
	type Value;
	fn merge_with(&self, global: &Self::Value) -> Self::Value;
}
