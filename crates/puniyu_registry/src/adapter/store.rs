use crate::{Error, Result};
use puniyu_types::adapter::Plugin;
use std::{
	collections::HashMap,
	sync::{
		Arc, RwLock,
		atomic::{AtomicU64, Ordering},
	},
};
static ADAPTER_INDEX: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Default)]
pub(crate) struct AdapterStore(Arc<RwLock<HashMap<u64, Arc<dyn Plugin>>>>);

impl AdapterStore {
	pub fn new() -> Self {
		Self::default()
	}
	pub fn insert(&self, adapter: Arc<dyn Plugin>) -> Result<u64> {
		let index = ADAPTER_INDEX.fetch_add(1, Ordering::SeqCst);
		let mut map = self.0.write().expect("Failed to acquire lock");
		if map.values().any(|v| v == &adapter) {
			return Err(Error::Exists("Adapter".to_string()));
		}
		map.insert(index, adapter);
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
