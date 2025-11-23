use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use toml::Value;

#[derive(Debug, Clone)]
pub struct Config {
	pub path: PathBuf,
	pub value: Arc<RwLock<Value>>,
}

#[derive(Default)]
pub(crate) struct ConfigStore(Vec<Config>);

impl ConfigStore {
	pub fn insert(&mut self, path: &Path, value: Value) {
		self.0.push(Config { path: path.to_path_buf(), value: Arc::new(RwLock::new(value)) });
	}

	pub fn get(&self, path: &Path) -> Option<Value> {
		self.0.iter().find(|c| c.path == path).map(|c| c.value.read().unwrap().clone())
	}

	pub fn update(&mut self, path: &Path, value: Value) {
		if let Some(config) = self.0.iter().find(|c| c.path == path)
			&& let Ok(mut lock) = config.value.write()
		{
			*lock = value;
		}
	}

	pub fn all(&self) -> Vec<Config> {
		self.0.clone()
	}
}
