use crate::{Error, Result};
use puniyu_types::plugin::Plugin;
use std::{
	collections::HashMap,
	sync::{
		Arc, RwLock,
		atomic::{AtomicU64, Ordering},
	},
};

static PLUGIN_INDEX: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Default)]
pub(crate) struct PluginStore(Arc<RwLock<HashMap<u64, Arc<dyn Plugin>>>>);

impl PluginStore {
	pub fn new() -> Self {
		Self::default()
	}
	pub fn insert(&self, plugin: Arc<dyn Plugin>) -> Result<u64> {
		let index = PLUGIN_INDEX.fetch_add(1, Ordering::SeqCst);
		let mut map = self.0.write().expect("Failed to acquire lock");
		if map.values().any(|v| v == &plugin) {
			return Err(Error::Exists("Plugin".to_string()));
		}
		map.insert(index, plugin);
		Ok(index)
	}

	pub fn all(&self) -> Vec<Arc<dyn Plugin>> {
		let map = self.0.read().expect("Failed to acquire lock");
		map.values().cloned().collect()
	}

	pub fn raw(&self) -> Arc<RwLock<HashMap<u64, Arc<dyn Plugin>>>> {
		self.0.clone()
	}
}
