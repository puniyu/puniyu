use crate::error::Config as Error;
use serde::{de::DeserializeOwned, Serialize};
use std::{fs, path::{Path, PathBuf}};
use toml::{from_str, to_string_pretty, Value};

fn build_config_path(path: &Path, name: &str) -> PathBuf {
	path.join(format!("{}.toml", name))
}

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
	let full_path = build_config_path(path, name);
	let config_str = fs::read_to_string(&full_path)?;
	Ok(from_str(&config_str)?)
}
/// 写入toml配置文件(如果目录不存在会自动创建)
///
/// # 参数
///
/// `path` 配置文件路径
/// `name` 配置文件名, 不包含后缀
pub fn write_config<C>(path: &Path, name: &str, config: &C) -> Result<(), Error>
where
	C: Serialize,
{
	let full_path = build_config_path(path, name);
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
/// use puniyu_common::toml::update_config;
///
/// #[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// pub struct Config {
///     field1: String,
///     field2: u32,
/// }
///
///
/// update_config::<Config>(Path::new("config"), "config", |config| {
///     config.field1 = "new value".to_string();
///     config.field2 = 42;
///    });
/// ```
///
pub fn update_config<C>(path: &Path, name: &str, updater: impl FnOnce(&mut C)) -> Result<(), Error>
where
	C: Serialize + DeserializeOwned + Default,
{
	let mut config = read_config(path, name).unwrap_or_else(|_| C::default());
	updater(&mut config);
	let full_path = build_config_path(path, name);
	write_to_file(&full_path, &config)
}

/// 合并两个TOML配置结构体并写入文件
///
/// # 参数
///
/// `path` 配置文件路径
/// `name` 配置文件名, 不包含后缀
/// `default_config` 第一个配置结构体
/// `user_config` 第二个配置结构体
///
pub fn merge_config<C1, C2>(
	path: &Path,
	name: &str,
	default_config: &C1,
	user_config: &C2,
) -> Result<(), Error>
where
	C1: Serialize,
	C2: Serialize,
{
	fn to_value<T: Serialize>(config: &T) -> Result<Value, Error> {
		let s = to_string_pretty(config)?;
		Ok(from_str(&s)?)
	}

	let mut base_value = to_value(default_config)?;
	let fill_value = to_value(user_config)?;

	merge_toml_values(&mut base_value, fill_value);

	let full_path = build_config_path(path, name);
	write_to_file(&full_path, &base_value)
}

fn merge_toml_values(base: &mut Value, fill: Value) {
	match (base, fill) {
		(Value::Table(base_table), Value::Table(fill_table)) => {
			for (key, fill_value) in fill_table {
				match base_table.get_mut(&key) {
					Some(base_value) => merge_toml_values(base_value, fill_value),
					None => {
						base_table.insert(key, fill_value);
					}
				}
			}
		}
		(base, fill) => *base = fill,
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
/// use puniyu_common::toml::delete_config;
/// let config_path = Path::new("./config");
/// delete_config(config_path, "bot", "field").unwrap();
/// ```
pub fn delete_config(path: &Path, name: &str, node_path: &str) -> Result<(), Error> {
	let full_path = build_config_path(path, name);
	let config_str = fs::read_to_string(&full_path)?;
	let mut toml_value: Value = from_str(&config_str)?;
	
	delete_toml_node(&mut toml_value, node_path);
	write_to_file(&full_path, &toml_value)
}

fn write_to_file<T: Serialize>(full_path: &Path, data: &T) -> Result<(), Error> {
	if let Some(parent) = full_path.parent() {
		fs::create_dir_all(parent)?;
	}
	let config_str = to_string_pretty(data)?;
	fs::write(full_path, config_str)?;
	Ok(())
}

/// 删除TOML值中的嵌套节点
///
/// # 参数
///
/// * `value` - TOML值的可变引用
/// * `node_path` - 节点路径,使用点号分隔,如 "parent.child"
fn delete_toml_node(value: &mut Value, node_path: &str) {
	let keys: Vec<&str> = node_path.split('.').collect();
	if keys.is_empty() {
		return;
	}

	let (last_key, parent_keys) = keys.split_last().unwrap();
	
	let mut current = value;
	for &key in parent_keys {
		if let Value::Table(table) = current {
			if let Some(next) = table.get_mut(key) {
				current = next;
			} else {
				return;
			}
		} else {
			return;
		}
	}
	
	if let Value::Table(table) = current {
		table.remove(*last_key);
	}
}
