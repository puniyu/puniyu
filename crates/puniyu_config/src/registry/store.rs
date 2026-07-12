use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub(crate) struct ConfigEntry {
	pub name: smol_str::SmolStr,
	pub path: std::path::PathBuf,
	pub value: toml::Value,
}

impl PartialEq for ConfigEntry {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name && self.path == other.path
	}
}
impl Eq for ConfigEntry {}

#[derive(Default)]
pub(crate) struct ConfigStore(Arc<RwLock<HashMap<u64, ConfigEntry>>>);

impl ConfigStore {
	pub fn new() -> Self {
		Self::default()
	}

	pub(crate) fn raw(&self) -> &Arc<RwLock<HashMap<u64, ConfigEntry>>> {
		&self.0
	}
}
