use crate::Adapter;
use crate::Error;
use std::{
	collections::HashMap,
	sync::{
		Arc, RwLock,
		atomic::{AtomicU64, Ordering},
	},
};

static ADAPTER_INDEX: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Default)]
pub(crate) struct AdapterStore(Arc<RwLock<HashMap<u64, Arc<dyn Adapter>>>>);

impl AdapterStore {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn insert(&self, adapter: Arc<dyn Adapter>) -> Result<u64, Error> {
		let mut map = self.0.write().expect("Failed to acquire lock");
		let adapter_name = adapter.adapter_info().name.clone();
		if map.values().any(|v| v.adapter_info().name == adapter_name) {
			return Err(Error::Exists("Adapter".to_string()));
		}
		let index = ADAPTER_INDEX.fetch_add(1, Ordering::Relaxed);
		map.insert(index, adapter);
		Ok(index)
	}

	pub fn all(&self) -> Vec<Arc<dyn Adapter>> {
		let map = self.0.read().expect("Failed to acquire lock");
		map.values().cloned().collect()
	}

	pub fn raw(&self) -> Arc<RwLock<HashMap<u64, Arc<dyn Adapter>>>> {
		self.0.clone()
	}
}
