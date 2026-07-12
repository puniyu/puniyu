mod store;

use crate::StoredEntry;
use crate::error::Error;
use crate::types::ConfigId;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicU64, Ordering};
use store::{ConfigEntry, ConfigStore};

static CONFIG_ID: AtomicU64 = AtomicU64::new(0);
static STORE: LazyLock<ConfigStore> = LazyLock::new(ConfigStore::new);

pub struct ConfigRegistry;

impl ConfigRegistry {
	pub fn register<T>(name: &str, path: impl AsRef<Path>, config: T) -> Result<u64, Error>
	where
		T: Serialize + for<'de> Deserialize<'de> + Clone + Send + Sync + 'static,
	{
		let value = toml::Value::try_from(&config)?;
		let mut map = STORE.raw().write().expect("poisoned lock");
		if map.values().any(|v| v.name == name) {
			return Err(Error::Exists);
		}
		let index = CONFIG_ID.fetch_add(1, Ordering::Relaxed);
		map.insert(
			index,
			ConfigEntry { name: name.into(), path: path.as_ref().to_path_buf(), value },
		);

		Ok(index)
	}

	pub fn get<T>(id: impl Into<ConfigId>) -> Option<T>
	where
		T: Serialize + for<'de> Deserialize<'de> + Clone + Send + Sync + 'static,
	{
		let id = id.into();
		match id {
			ConfigId::Index(id) => Self::get_with_index(id),
			ConfigId::Path(path) => Self::get_with_path(path),
		}
	}

	pub fn get_with_index<T>(id: u64) -> Option<T>
	where
		T: Serialize + for<'de> Deserialize<'de> + Clone + Send + Sync + 'static,
	{
		let map = STORE.raw().read().expect("poisoned lock");
		map.get(&id)
			.and_then(|e| toml::to_string(&e.value).ok().and_then(|s| toml::from_str(&s).ok()))
	}

	pub fn get_with_path<T>(path: impl AsRef<Path>) -> Option<T>
	where
		T: Serialize + for<'de> Deserialize<'de> + Clone + Send + Sync + 'static,
	{
		let map = STORE.raw().read().expect("poisoned lock");
		map.values()
			.find(|e| e.path == path.as_ref())
			.and_then(|e| toml::to_string(&e.value).ok().and_then(|s| toml::from_str(&s).ok()))
	}

	pub fn update<T>(id: impl Into<ConfigId>, value: T) -> Result<(), Error>
	where
		T: Serialize + for<'de> Deserialize<'de> + Clone + Send + Sync + 'static,
	{
		let id = id.into();
		match id {
			ConfigId::Index(id) => Self::update_with_index(id, value),
			ConfigId::Path(path) => Self::update_by_path(path, value),
		}
	}

	pub fn update_with_index<T>(id: u64, value: T) -> Result<(), Error>
	where
		T: Serialize + for<'de> Deserialize<'de> + Clone + Send + Sync + 'static,
	{
		let toml_val = toml::Value::try_from(&value)?;
		let mut map = STORE.raw().write().expect("poisoned lock");
		match map.get_mut(&id) {
			Some(entry) => {
				entry.value = toml_val;
				Ok(())
			}
			None => Err(Error::NotFound),
		}
	}

	pub fn update_by_path<T>(path: impl AsRef<Path>, value: T) -> Result<(), Error>
	where
		T: Serialize + for<'de> Deserialize<'de> + Clone + Send + Sync + 'static,
	{
		let toml_val = toml::Value::try_from(&value)?;
		let mut map = STORE.raw().write().expect("poisoned lock");
		match map.values_mut().find(|e| e.path == path.as_ref()) {
			Some(entry) => {
				entry.value = toml_val;
				Ok(())
			}
			None => Err(Error::NotFound),
		}
	}

	pub fn all<T>() -> Vec<StoredEntry<T>>
	where
		T: Serialize + for<'de> Deserialize<'de> + Clone + Send + Sync + 'static,
	{
		let map = STORE.raw().read().expect("poisoned lock");
		map.values()
			.filter_map(|e| {
				toml::to_string(&e.value)
					.ok()
					.and_then(|s| toml::from_str(&s).ok())
					.map(|value| StoredEntry { name: e.name.clone(), path: e.path.clone(), value })
			})
			.collect()
	}
}
