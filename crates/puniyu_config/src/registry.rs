use crate::store::Config;
use crate::store::ConfigStore;
use std::{
	path::Path,
	sync::{LazyLock, RwLock},
};
use toml::Value;

static CONFIG_STORE: LazyLock<RwLock<ConfigStore>> =
	LazyLock::new(|| RwLock::new(ConfigStore::default()));

pub struct ConfigRegistry;

impl ConfigRegistry {
	pub fn register(path: &Path, value: Value) {
		if !path.exists() {
			let _ = std::fs::create_dir_all(path);
		}
		CONFIG_STORE.write().unwrap().insert(path, value);
	}

	pub fn get(path: &Path) -> Option<Value> {
		CONFIG_STORE.read().unwrap().get(path)
	}
	pub fn update(path: &Path, value: Value) {
		CONFIG_STORE.write().unwrap().update(path, value);
	}

	pub fn all() -> Vec<Config> {
		CONFIG_STORE.read().unwrap().all()
	}
}