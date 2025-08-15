use super::utils::delete_nested_node;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, from_str};
use std::{error::Error, fs, path::Path};

/// 读取json配置文件
///
/// # 参数
///
/// `path` 配置文件路径
/// `name` 配置文件名, 不包含后缀
pub fn read_config<D>(path: &Path, name: &str) -> Result<D, Box<dyn Error>>
where
    D: DeserializeOwned,
{
    let full_path = path.join(format!("{}.json", name));
    let config_str = fs::read_to_string(full_path)?;
    let config: D = from_str(&config_str)?;
    Ok(config)
}

/// 写入json配置文件
///
/// # 参数
///
/// `path` 配置文件路径
/// `name` 配置文件名, 不包含后缀
///
pub fn write_config<C>(path: &Path, name: &str, config: &C) -> Result<(), Box<dyn Error>>
where
    C: Serialize,
{
    let full_path = path.join(format!("{}.json", name));
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let config_str = serde_json::to_string_pretty(config)?;
    fs::write(full_path, config_str)?;
    Ok(())
}

/// 更新json配置文件
///
/// # 参数
///
/// `path` 配置文件路径
/// `name` 配置文件名, 不包含后缀
/// `updater` 更新函数, 用于更新配置
///
/// # 示例
///
/// ```
/// use serde::{Deserialize, Serialize};
/// use std::path::Path;
/// use puniyu_utils::config::json::update_config;
///
/// #[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// pub struct Config {
///     pub field: String,
/// }
///
/// let config_path = Path::new("./config");
/// update_config::<Config>(config_path, "bot", |config| {
///     config.field = "loli".to_string()
/// }).unwrap();
/// ```
pub fn update_config<C>(
    path: &Path,
    name: &str,
    updater: impl FnOnce(&mut C),
) -> Result<(), Box<dyn Error>>
where
    C: Serialize + DeserializeOwned + Default,
{
    let full_path = path.join(format!("{}.json", name));
    let mut config = read_config(path, name).unwrap_or_else(|_| C::default());
    updater(&mut config);
    let config_str = serde_json::to_string_pretty(&config)?;
    fs::write(full_path, config_str)?;
    Ok(())
}

/// 删除json配置文件中的指定节点
///
/// # 参数
///
/// `path` 配置文件路径
/// `name` 配置文件名, 不包含后缀
/// `node_path` 要删除的节点路径，支持嵌套节点，如 "parent.child"
///
/// # 示例
///
/// ```
/// use std::path::Path;
/// use puniyu_utils::config::json::delete_config;
///
/// let config_path = Path::new("./config");
/// delete_config(config_path, "bot", "field").unwrap();
/// ```
pub fn delete_config(path: &Path, name: &str, node_path: &str) -> Result<(), Box<dyn Error>> {
    let full_path = path.join(format!("{}.json", name));

    if !full_path.exists() {
        return Ok(());
    }

    let config_str = fs::read_to_string(&full_path)?;
    let mut json_value: Value = from_str(&config_str)?;
    let node_keys: Vec<&str> = node_path.split('.').collect();

    delete_nested_node(
        &mut json_value,
        &node_keys,
        |value, key| {
            if let Value::Object(map) = value {
                map.get_mut(key)
            } else {
                None
            }
        },
        |value, key| {
            if let Value::Object(map) = value {
                map.remove(key);
            }
        },
    )?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let config_str = serde_json::to_string_pretty(&json_value)?;
    fs::write(full_path, config_str)?;
    Ok(())
}
