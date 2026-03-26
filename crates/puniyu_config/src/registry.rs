mod store;

use crate::types::{ConfigId, ConfigInfo};
use puniyu_error::registry::Error;
use std::{path::Path, sync::LazyLock};
use store::ConfigStore;
use toml::Value;

/// 全局配置存储实例
static STORE: LazyLock<ConfigStore> = LazyLock::new(ConfigStore::new);

/// 配置注册表
///
/// 提供全局配置管理功能，支持注册、查询和更新配置。
pub struct ConfigRegistry;

impl ConfigRegistry {
	/// 注册配置
	pub fn register(config: ConfigInfo) -> Result<u64, Error> {
		STORE.insert(config)
	}

	/// 获取配置
	pub fn get<C>(id: C) -> Option<Value>
	where
		C: Into<ConfigId>,
	{
		let id = id.into();
		match id {
			ConfigId::Index(id) => Self::get_with_index(id),
			ConfigId::Path(path) => Self::get_with_path(path),
		}
	}

	/// 通过索引获取配置
	pub fn get_with_index(id: u64) -> Option<Value> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		Some(map.get(&id).cloned()?.value)
	}

	/// 通过路径获取配置
	pub fn get_with_path<P>(path: P) -> Option<Value>
	where
		P: AsRef<Path>,
	{
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		Some(map.values().find(|v| v.path == path.as_ref())?.value.clone())
	}

	/// 更新配置
	pub fn update<C>(id: C, value: Value) -> Result<(), Error>
	where
		C: Into<ConfigId>,
	{
		let id = id.into();
		match id {
			ConfigId::Index(id) => Self::update_with_index(id, value),
			ConfigId::Path(path) => Self::update_with_path(path, value),
		}
	}

	/// 通过索引更新配置
	pub fn update_with_index(id: u64, value: Value) -> Result<(), Error> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		let config = map.get_mut(&id).ok_or(Error::NotFound("Config".to_string()))?;
		config.value = value;
		Ok(())
	}

	/// 通过路径更新配置
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
	pub fn all() -> Vec<ConfigInfo> {
		STORE.all()
	}
}
