use std::{fs, path::Path};

use serde::{Deserialize, Serialize};
use crate::Error;

fn write_to_file<T: Serialize>(path: impl AsRef<Path>, data: &T) -> Result<(), Error> {
    let path = path.as_ref();
	if let Some(parent) = path.parent() {
		fs::create_dir_all(parent)?;
	}
	fs::write(path, toml::to_string_pretty(data)?)?;
	Ok(())
}

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