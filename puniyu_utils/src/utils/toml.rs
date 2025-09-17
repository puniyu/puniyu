use super::utils::delete_nested_node;
use crate::error::Config as Error;
use serde::{Serialize, de::DeserializeOwned};
use std::{fs, path::Path};
use toml::{Value, from_str, to_string_pretty};

/// 读取toml配置文件
///
/// # 参数
///
/// `path` 配置文件路径
/// `name` 配置文件名, 不包含后缀
pub fn read_config<D>(path: &Path, name: &str) -> Result<D, Error>
where
	D: DeserializeOwned,
{
	let full_path = path.join(format!("{}.toml", name));
	if !full_path.exists() {
		return Err(Error::NotFound);
	}
	let config_str = fs::read_to_string(full_path).map_err(|_| Error::Read)?;
	let config: D = from_str(&config_str).map_err(|_| Error::Parse)?;
	Ok(config)
}
/// 写入toml配置文件
///
/// # 参数
///
/// `path` 配置文件路径
/// `name` 配置文件名, 不包含后缀
pub fn write_config<C>(path: &Path, name: &str, config: &C) -> Result<(), Error>
where
	C: Serialize,
{
	let full_path = path.join(format!("{}.toml", name));
	if !full_path.exists() {
		return Err(Error::NotFound);
	}
	write_to_file(&full_path, config)
}

/// 更新toml配置文件
///
/// # 参数
///
/// `path` 配置文件路径
/// `name` 配置文件名, 不包含后缀
///
/// # 示例
///
/// ```
/// use serde::{Deserialize, Serialize};
/// use std::path::Path;
/// use puniyu_utils::utils::toml::update_config;
///
/// #[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// pub struct Config {
///     field1: String,
///     field2: u32,
/// }
///
/// update_config::<Config>(Path::new("config"), "config", |config| {
///     config.field1 = "new value".to_string();
///     config.field2 = 42;
/// })?;
/// ```
///
pub fn update_config<C>(path: &Path, name: &str, updater: impl FnOnce(&mut C)) -> Result<(), Error>
where
	C: Serialize + DeserializeOwned + Default,
{
	let full_path = path.join(format!("{}.toml", name));
	if !full_path.exists() {
		return Err(Error::NotFound);
	}

	let mut config = read_config(path, name).unwrap_or_else(|_| C::default());

	updater(&mut config);

	write_to_file(&full_path, &config)
}

/// 合并两个TOML配置结构体并写入文件
///
/// # 参数
///
/// `path` 配置文件路径
/// `name` 配置文件名, 不包含后缀
/// `config1` 第一个配置结构体
/// `config2` 第二个配置结构体
///
pub fn merge_config<C1, C2>(
	path: &Path,
	name: &str,
	user_config: &C1,
	default_config: &C2,
) -> Result<(), Error>
where
	C1: Serialize,
	C2: Serialize,
{
	let user_config_str =
		serde_json::to_string_pretty(user_config).map_err(|_| Error::Serialize)?;
	let default_config_str =
		serde_json::to_string_pretty(default_config).map_err(|_| Error::Serialize)?;

	let mut user_config_value: Value = from_str(&user_config_str).map_err(|_| Error::Parse)?;
	let default_config_value: Value = from_str(&default_config_str).map_err(|_| Error::Parse)?;

	merge_toml_values(&mut user_config_value, default_config_value);

	let full_path = path.join(format!("{}.toml", name));
	if !full_path.exists() {
		return Err(Error::NotFound);
	}
	write_to_file(&full_path, &user_config_value)
}

/// 递归合并两个 TOML 值
fn merge_toml_values(base: &mut Value, fill: Value) {
	if let (Value::Table(base_table), Value::Table(fill_table)) = (base, fill) {
		for (key, fill_value) in fill_table {
			if !base_table.contains_key(&key) {
				base_table.insert(key, fill_value);
			}
		}
	}
}

/// 删除toml配置文件中的指定节点
///
/// # 参数
///
/// `path` 配置文件路径
/// `name` 配置文件名, 不包含后缀
/// `node_path` 要删除的节点路径，支持嵌套节点，如 "parent.child"
///
/// # 示例
/// ```
/// use std::path::Path;
/// use puniyu_utils::utils::toml::delete_config;
/// let config_path = Path::new("./config");
/// delete_config(config_path, "bot", "field").unwrap();
/// ```
pub fn delete_config(path: &Path, name: &str, node_path: &str) -> Result<(), Error> {
	let full_path = path.join(format!("{}.toml", name));

	if !full_path.exists() {
		return Err(Error::NotFound);
	}

	let config_str = fs::read_to_string(&full_path).map_err(|e| match e.kind() {
		std::io::ErrorKind::PermissionDenied => Error::PermissionDenied,
		_ => Error::Read,
	})?;
	let mut toml_value: Value = from_str(&config_str).map_err(|_| Error::Parse)?;
	let node_keys: Vec<&str> = node_path.split('.').collect();

	delete_nested_node(
		&mut toml_value,
		&node_keys,
		|value, key| {
			if let Value::Table(table) = value { table.get_mut(key) } else { None }
		},
		|value, key| {
			if let Value::Table(table) = value {
				table.remove(key);
			}
		},
	)?;
	write_to_file(&full_path, &toml_value)
}

fn write_to_file<T: Serialize>(full_path: &Path, data: &T) -> Result<(), Error> {
	use std::io::ErrorKind;
	if let Some(parent) = full_path.parent() {
		fs::create_dir_all(parent).map_err(|e| match e.kind() {
			ErrorKind::PermissionDenied => Error::PermissionDenied,
			_ => Error::Write,
		})?;
	}
	let config_str = to_string_pretty(data).map_err(|_| Error::Serialize)?;
	fs::write(full_path, config_str).map_err(|e| match e.kind() {
		ErrorKind::PermissionDenied => Error::PermissionDenied,
		_ => Error::Write,
	})?;
	Ok(())
}
