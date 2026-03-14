mod store;

use crate::types::{ConfigId, ConfigInfo};
use puniyu_error::registry::Error;
use std::{path::Path, sync::LazyLock};
use std::sync::Arc;
use store::ConfigStore;
use toml::Value;
use crate::Config;

/// 全局配置存储实例
static STORE: LazyLock<ConfigStore> = LazyLock::new(ConfigStore::new);

/// 配置注册表
///
/// 提供全局配置管理功能，支持注册、查询和更新配置。
///
/// # 功能特性
///
/// - 配置注册：将实现了 `Config` trait 的配置注册到全局存储
/// - 多种查询方式：支持通过索引或路径查询配置
/// - 配置更新：支持动态更新已注册的配置
/// - 配置列表：获取所有已注册的配置
/// - 线程安全：所有操作都是线程安全的
///
/// # 使用示例
///
/// ```rust,ignore
/// use puniyu_config_core::{Config, ConfigRegistry};
/// use std::path::Path;
/// use std::sync::Arc;
/// use toml::Value;
///
/// // 实现 Config trait
/// struct MyConfig;
///
/// impl Config for MyConfig {
///     fn name(&self) -> &'static str {
///         "my_config"
///     }
///
///     fn path(&self) -> &'static Path {
///         Path::new("config/my_config.toml")
///     }
///
///     fn config(&self) -> Value {
///         toml::toml! {
///             enabled = true
///             timeout = 30
///         }
///     }
/// }
///
/// // 注册配置
/// let config = Arc::new(MyConfig);
/// let id = ConfigRegistry::register(config)?;
///
/// // 通过索引查询
/// if let Some(value) = ConfigRegistry::get(id) {
///     println!("配置: {:?}", value);
/// }
///
/// // 通过路径查询
/// if let Some(value) = ConfigRegistry::get(Path::new("config/my_config.toml")) {
///     println!("配置: {:?}", value);
/// }
///
/// // 更新配置
/// let new_value = toml::toml! {
///     enabled = false
///     timeout = 60
/// };
/// ConfigRegistry::update(id, new_value)?;
/// ```
pub struct ConfigRegistry;

impl ConfigRegistry {
	/// 注册配置
	///
	/// 将实现了 `Config` trait 的配置注册到全局存储中。
	///
	/// # 参数
	///
	/// - `config`: 要注册的配置，必须是 `Arc<dyn Config>` 类型
	///
	/// # 返回值
	///
	/// 成功时返回配置的唯一索引 ID，失败时返回错误
	///
	/// # 错误
	///
	/// - 如果配置已存在，返回 `Error::Exists`
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_config_core::{Config, ConfigRegistry};
	/// use std::sync::Arc;
	///
	/// let config = Arc::new(MyConfig);
	/// let id = ConfigRegistry::register(config)?;
	/// ```
	pub fn register(config: Arc<dyn Config>) -> Result<u64, Error> {
		STORE.insert(config)
	}

	/// 获取配置
	///
	/// 通过索引或路径获取配置值。
	///
	/// # 参数
	///
	/// - `id`: 配置标识符，可以是 `u64` 索引或 `&Path` 路径
	///
	/// # 返回值
	///
	/// 返回配置的 TOML 值，如果配置不存在则返回 `None`
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use std::path::Path;
	///
	/// // 通过索引获取
	/// let value = ConfigRegistry::get(0u64);
	///
	/// // 通过路径获取
	/// let value = ConfigRegistry::get(Path::new("config/app.toml"));
	/// ```
	pub fn get<'c, C>(id: C) -> Option<Value>
	where
		C: Into<ConfigId<'c>>,
	{
		let id = id.into();
		match id {
			ConfigId::Index(id) => Self::get_with_index(id),
			ConfigId::Path(path) => Self::get_with_path(path),
		}
	}

	/// 通过索引获取配置
	///
	/// # 参数
	///
	/// - `id`: 配置的索引 ID
	///
	/// # 返回值
	///
	/// 返回配置的 TOML 值，如果配置不存在则返回 `None`
	pub fn get_with_index(id: u64) -> Option<Value> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		Some(map.get(&id).cloned()?.value)
	}

	/// 通过路径获取配置
	///
	/// # 参数
	///
	/// - `path`: 配置文件的路径
	///
	/// # 返回值
	///
	/// 返回配置的 TOML 值，如果配置不存在则返回 `None`
	pub fn get_with_path<P>(path: P) -> Option<Value>
	where
		P: AsRef<Path>,
	{
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		Some(map.values().find(|v| v.path == path.as_ref())?.value.clone())
	}

	/// 更新配置
	///
	/// 通过索引或路径更新配置值。
	///
	/// # 参数
	///
	/// - `id`: 配置标识符，可以是 `u64` 索引或 `&Path` 路径
	/// - `value`: 新的配置值
	///
	/// # 返回值
	///
	/// 成功时返回 `Ok(())`，失败时返回错误
	///
	/// # 错误
	///
	/// - 如果配置不存在，返回 `Error::NotFound`
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use std::path::Path;
	///
	/// // 通过索引更新
	/// ConfigRegistry::update(0u64, new_value)?;
	///
	/// // 通过路径更新
	/// ConfigRegistry::update(Path::new("config/app.toml"), new_value)?;
	/// ```
	pub fn update<'c, C>(id: C, value: Value) -> Result<(), Error>
	where
		C: Into<ConfigId<'c>>,
	{
		let id = id.into();
		match id {
			ConfigId::Index(id) => Self::update_with_index(id, value),
			ConfigId::Path(path) => Self::update_with_path(path, value),
		}
	}

	/// 通过索引更新配置
	///
	/// # 参数
	///
	/// - `id`: 配置的索引 ID
	/// - `value`: 新的配置值
	///
	/// # 返回值
	///
	/// 成功时返回 `Ok(())`，如果配置不存在则返回 `Error::NotFound`
	pub fn update_with_index(id: u64, value: Value) -> Result<(), Error> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		let config = map.get_mut(&id).ok_or(Error::NotFound("Config".to_string()))?;
		config.value = value;
		Ok(())
	}

	/// 通过路径更新配置
	///
	/// # 参数
	///
	/// - `path`: 配置文件的路径
	/// - `value`: 新的配置值
	///
	/// # 返回值
	///
	/// 成功时返回 `Ok(())`
	///
	/// # 注意
	///
	/// 如果有多个配置使用相同路径，所有匹配的配置都会被更新。
	pub fn update_with_path<P>(path: P, value: Value) -> Result<(), Error>
	where
		P: AsRef<Path>,
	{
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		map.values_mut()
			.filter(|config| config.path == path.as_ref())
			.for_each(|config| config.value = value.clone());
		Ok(())
	}

	/// 获取所有配置
	///
	/// # 返回值
	///
	/// 返回所有已注册配置的列表
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let all_configs = ConfigRegistry::all();
	/// for config in all_configs {
	///     println!("路径: {:?}, 值: {:?}", config.path, config.value);
	/// }
	/// ```
	pub fn all() -> Vec<ConfigInfo> {
		STORE.all()
	}
}
