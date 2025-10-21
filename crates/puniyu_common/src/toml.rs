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
	let default_config_str = to_string_pretty(default_config).map_err(|_| Error::Serialize)?;
	let user_config_str = to_string_pretty(user_config).map_err(|_| Error::Serialize)?;

	let mut default_config_value = from_str(&default_config_str).map_err(|_| Error::Parse)?;
	let user_config_value = from_str(&user_config_str).map_err(|_| Error::Parse)?;

	merge_toml_values(&mut default_config_value, user_config_value);

	let full_path = path.join(format!("{}.toml", name));
	write_to_file(&full_path, &default_config_value)
}

/// 递归合并两个 TOML 值
fn merge_toml_values(base: &mut Value, fill: Value) {
	match (base, fill) {
		(Value::Table(base_table), Value::Table(fill_table)) => {
			for (key, fill_value) in fill_table {
				if let Some(base_value) = base_table.get_mut(&key) {
					merge_toml_values(base_value, fill_value);
				} else {
					base_table.insert(key, fill_value);
				}
			}
		}
		(base, fill) => {
			*base = fill;
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
/// use puniyu_common::toml::delete_config;
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

	delete_node(
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

/// 通用函数，删除嵌套节点
///
/// 该函数提供了一种通用的方法来删除嵌套数据结构中的节点。
///
/// # 参数
///
/// * `value` - 需要操作的数据结构的可变引用
/// * `node_keys` - 表示节点路径的字符串切片数组
/// * `get_mut_mapping` - 闭包函数，用于获取指定键的可变引用
/// * `remove_from_mapping` - 闭包函数，用于从数据结构中删除指定键
///
fn delete_node<V, G, M>(
	value: &mut V,
	node_keys: &[&str],
	get_mut_mapping: G,
	remove_from_mapping: M,
) -> Result<(), Error>
where
	G: for<'a> Fn(&'a mut V, &str) -> Option<&'a mut V>,
	M: Fn(&mut V, &str),
{
	if node_keys.is_empty() {
		return Ok(());
	}

	let (current_key, remaining_keys) = node_keys.split_first().unwrap();

	if remaining_keys.is_empty() {
		remove_from_mapping(value, current_key);
		Ok(())
	} else if let Some(child_value) = get_mut_mapping(value, current_key) {
		delete_node(child_value, remaining_keys, get_mut_mapping, remove_from_mapping)
	} else {
		Ok(())
	}
}
