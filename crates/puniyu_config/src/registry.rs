mod store;

use std::path::Path;

use puniyu_error::registry::Error;
use std::sync::LazyLock;
use store::ConfigStore;
use toml::Value;
use crate::types::{ConfigId, ConfigInfo};

static STORE: LazyLock<ConfigStore> = LazyLock::new(ConfigStore::new);

pub struct ConfigRegistry;

impl ConfigRegistry {
	pub fn register(config: ConfigInfo) -> Result<u64, Error> {
		STORE.insert(config)
	}

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
	pub fn get_with_index(id: u64) -> Option<Value> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		Some(map.get(&id).cloned()?.value)
	}

	pub fn get_with_path<P>(path: P) -> Option<Value>
	where
		P: AsRef<Path>,
	{
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		Some(map.values().find(|v| v.path == path.as_ref().to_path_buf())?.value.clone())
	}

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

	pub fn update_with_index(id: u64, value: Value) -> Result<(), Error> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		let config = map.get_mut(&id).ok_or(Error::NotFound("Config".to_string()))?;
		config.value = value;
		Ok(())
	}

	pub fn update_with_path<P>(path: P, value: Value) -> Result<(), Error>
	where
		P: AsRef<Path>,
	{
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		map.values_mut()
			.filter(|config| config.path == path.as_ref().to_path_buf())
			.for_each(|config| config.value = value.clone());
		Ok(())
	}

	pub fn all() -> Vec<ConfigInfo> {
		STORE.all()
	}
}
