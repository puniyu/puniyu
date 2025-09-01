use super::utils::delete_nested_node;
use serde::{Serialize, de::DeserializeOwned};
use std::{error::Error, fs, path::Path};
use toml::{from_str, to_string_pretty};

/// 读取toml配置文件
///
/// # 参数
///
/// `path` 配置文件路径
/// `name` 配置文件名, 不包含后缀
pub fn read_config<D>(path: &Path, name: &str) -> Result<D, Box<dyn Error>>
where
    D: DeserializeOwned,
{
    let full_path = path.join(format!("{}.toml", name));
    let config_str = fs::read_to_string(full_path)?;
    let config: D = from_str(&config_str)?;
    Ok(config)
}
/// 写入toml配置文件
///
/// # 参数
///
/// `path` 配置文件路径
/// `name` 配置文件名, 不包含后缀
pub fn write_config<C>(path: &Path, name: &str, config: &C) -> Result<(), Box<dyn Error>>
where
    C: Serialize,
{
    let full_path = path.join(format!("{}.toml", name));
    if let Some(parent) = full_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let config_str = to_string_pretty(config)?;
    fs::write(full_path, config_str)?;
    Ok(())
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
/// use puniyu_utils::config::toml::update_config;
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
pub fn update_config<C>(
    path: &Path,
    name: &str,
    updater: impl FnOnce(&mut C),
) -> Result<(), Box<dyn Error>>
where
    C: Serialize + DeserializeOwned + Default,
{
    let full_path = path.join(format!("{}.toml", name));

    let mut config = read_config(path, name).unwrap_or_else(|_| C::default());

    updater(&mut config);

    if let Some(parent) = full_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let config_str = to_string_pretty(&config)?;
    fs::write(full_path, config_str)?;
    Ok(())
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
/// use puniyu_utils::config::toml::delete_config;
/// let config_path = Path::new("./config");
/// delete_config(config_path, "bot", "field").unwrap();
/// ```
pub fn delete_config(path: &Path, name: &str, node_path: &str) -> Result<(), Box<dyn Error>> {
    let full_path = path.join(format!("{}.toml", name));

    if !full_path.exists() {
        return Ok(());
    }

    let config_str = fs::read_to_string(&full_path)?;
    let mut toml_value: toml::Value = from_str(&config_str)?;
    let node_keys: Vec<&str> = node_path.split('.').collect();

    delete_nested_node(
        &mut toml_value,
        &node_keys,
        |value, key| {
            if let toml::Value::Table(table) = value {
                table.get_mut(key)
            } else {
                None
            }
        },
        |value, key| {
            if let toml::Value::Table(table) = value {
                table.remove(key);
            }
        },
    )?;

    if let Some(parent) = full_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let config_str = to_string_pretty(&toml_value)?;
    fs::write(full_path, config_str)?;

    Ok(())
}
