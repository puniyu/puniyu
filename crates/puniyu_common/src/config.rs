use super::Error;
use serde::{Serialize, de::DeserializeOwned};
use std::{
	fs,
	path::{Path, PathBuf},
};
use toml::{Value, from_str, to_string_pretty};

/// 构建配置文件的完整路径
fn build_config_path<P: AsRef<Path>>(path: P, name: &str) -> PathBuf {
	if name.ends_with(".toml") {
		return path.as_ref().join(name);
	}
	path.as_ref().join(format!("{}.toml", name))
}

/// 读取配置文件
///
/// # 参数
///
/// * `path` - 配置文件所在目录路径
/// * `name` - 配置文件名
///
///
/// # 示例
///
/// ```rust, ignore
/// use serde::Deserialize;
/// use std::path::Path;
/// use puniyu_common::config::read_config;
///
/// #[derive(Deserialize)]
/// struct Config {
///     name: String,
///     port: u16,
/// }
///
/// let config: Config = read_config(Path::new("./config"), "app").unwrap();
/// ```
pub fn read_config<D>(path: &Path, name: &str) -> Result<D, Error>
where
	D: DeserializeOwned,
{
	let full_path = build_config_path(path, name);
	let config_str = fs::read_to_string(&full_path)?;
	Ok(from_str(&config_str)?)
}
/// 写入配置文件
///
/// # 参数
///
/// * `path` - 配置文件所在目录路径
/// * `name` - 配置文件名（不包含 .toml 扩展名）
/// * `config` - 要写入的配置对象引用
///
///
/// # 示例
///
/// ```rust
/// use serde::Serialize;
/// use std::path::Path;
/// use puniyu_common::config::write_config;
///
/// #[derive(Serialize)]
/// struct Config {
///     name: String,
///     port: u16,
/// }
///
/// let config = Config {
///     name: "MyApp".to_string(),
///     port: 8080,
/// };
///
/// write_config(Path::new("./config"), "app", &config).unwrap();
/// ```
pub fn write_config<C>(path: &Path, name: &str, config: &C) -> Result<(), Error>
where
	C: Serialize,
{
	let full_path = build_config_path(path, name);
	write_to_file(&full_path, config)
}

/// 更新配置文件
///
///
/// # 参数
///
/// * `path` - 配置文件所在目录路径
/// * `name` - 配置文件名（不包含 .toml 扩展名）
/// * `updater` - 用于修改配置的闭包函数，接收可变配置引用
///
///
/// # 示例
///
/// ```rust
/// use serde::{Deserialize, Serialize};
/// use std::path::Path;
/// use puniyu_common::config::update_config;
///
/// #[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// pub struct Config {
///     field1: String,
///     field2: u32,
/// }
///
/// // 更新配置文件
/// update_config::<Config, _>(Path::new("config"), "app", |config| {
///     config.field1 = "new value".to_string();
///     config.field2 = 42;
/// }).unwrap();
/// ```
pub fn update_config<C, P>(path: P, name: &str, updater: impl FnOnce(&mut C)) -> Result<(), Error>
where
	P: AsRef<Path>,
	C: Serialize + DeserializeOwned + Default,
{
	let mut config = read_config(path.as_ref(), name).unwrap_or_else(|_| C::default());
	updater(&mut config);
	let full_path = build_config_path(path.as_ref(), name);
	write_to_file(&full_path, &config)
}

/// 合并配置文件
///
///
/// # 参数
///
/// * `path` - 配置文件所在目录路径
/// * `name` - 配置文件名（不包含 .toml 扩展名）
/// * `default_config` - 基础配置对象，提供默认值
/// * `user_config` - 用户配置对象，用于覆盖默认值
///
/// # 示例
///
/// ```rust
/// use serde::Serialize;
/// use std::path::Path;
/// use puniyu_common::config::merge_config;
///
/// #[derive(Serialize)]
/// struct DefaultConfig {
///     name: String,
///     port: u16,
///     debug: bool,
/// }
///
/// #[derive(Serialize)]
/// struct UserConfig {
///     port: u16,
/// }
///
/// let default = DefaultConfig {
///     name: "MyApp".to_string(),
///     port: 3000,
///     debug: false,
/// };
///
/// let user = UserConfig {
///     port: 8080,
/// };
///
/// // 合并后的配置将包含: name="MyApp", port=8080, debug=false
/// merge_config(Path::new("./config"), "app", &default, &user).unwrap();
/// ```
pub fn merge_config<P, C1, C2>(
	path: P,
	name: &str,
	default_config: &C1,
	user_config: &C2,
) -> Result<(), Error>
where
	P: AsRef<Path>,
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

/// 递归合并两个 TOML 值
///
/// 将 `fill` 值合并到 `base` 值中。对于表类型会进行递归合并，
/// 对于其他类型则直接替换。
///
/// # 参数
///
/// * `base` - 基础 TOML 值的可变引用，合并结果会存储在这里
/// * `fill` - 用于填充的 TOML 值，会被消费
///
/// # 合并行为
///
/// - 如果两个值都是表（Table），则递归合并各个字段
/// - 如果类型不匹配或其中一个不是表，则 `fill` 完全替换 `base`
/// - 对于表的合并，`fill` 中的字段会覆盖 `base` 中的同名字段
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

/// 删除 TOML 配置文件中的指定节点
///
/// 从现有配置文件中删除指定路径的节点，支持嵌套节点的删除。
/// 如果指定的节点不存在，操作会静默成功。
///
/// # 参数
///
/// * `path` - 配置文件所在目录路径
/// * `name` - 配置文件名（不包含 .toml 扩展名）
/// * `node_path` - 要删除的节点路径，使用点号分隔嵌套层级，如 "parent.child"
///
/// # 示例
///
/// ```rust, ignore
/// use std::path::Path;
/// use puniyu_common::toml::delete_config;
///
/// // 删除顶级字段
/// delete_config(Path::new("config"), "app", "debug").unwrap();
///
/// // 删除嵌套字段
/// delete_config(Path::new("config"), "app", "database.host").unwrap();
///
/// // 删除更深层的嵌套字段
/// delete_config(Path::new("config"), "app", "server.ssl.enabled").unwrap();
/// ```
pub fn delete_config<P: AsRef<Path>>(path: P, name: &str, node_path: &str) -> Result<(), Error> {
	let full_path = build_config_path(path, name);
	let config_str = fs::read_to_string(&full_path)?;
	let mut toml_value: Value = from_str(&config_str)?;

	delete_toml_node(&mut toml_value, node_path);
	write_to_file(&full_path, &toml_value)
}

/// 写入配置文件
fn write_to_file<T: Serialize>(full_path: &Path, data: &T) -> Result<(), Error> {
	if let Some(parent) = full_path.parent() {
		fs::create_dir_all(parent)?;
	}
	let config_str = to_string_pretty(data)?;
	fs::write(full_path, config_str)?;
	Ok(())
}

/// 删除 TOML 值中的嵌套节点
///
/// 根据点分隔的路径字符串，在 TOML 值结构中定位并删除指定的节点。
/// 支持任意深度的嵌套路径。
///
/// # 参数
///
/// * `value` - TOML 值的可变引用，删除操作会在此结构上进行
/// * `node_path` - 节点路径，使用点号分隔层级，如 "parent.child.grandchild"
///
/// # 行为说明
///
/// - 如果路径为空或无效，函数会提前返回，不进行任何操作
/// - 只有当完整路径存在且最终节点是表中的键时，才会执行删除
/// - 如果路径中的任何中间节点不存在或不是表类型，删除操作会静默失败
/// - 删除操作只影响目标节点，不会影响其他节点
///
/// # 示例路径
///
/// - `"field"` - 删除根级别的 field 字段
/// - `"database.host"` - 删除 database 表中的 host 字段  
/// - `"server.ssl.enabled"` - 删除 server.ssl 表中的 enabled 字段
fn delete_toml_node(value: &mut Value, node_path: &str) {
	let keys: Vec<&str> = node_path.split('.').collect();
	if keys.is_empty() {
		return;
	}

	let target_table =
		keys.iter().take(keys.len() - 1).try_fold(value, |current, &key| match current {
			Value::Table(table) => table.get_mut(key),
			_ => None,
		});

	if let (Some(Value::Table(table)), Some(last_key)) = (target_table, keys.last()) {
		table.remove(*last_key);
	}
}
