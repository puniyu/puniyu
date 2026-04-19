use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

use puniyu_error::registry::Error;

use crate::types::IpcProcess;

static IPC_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Default)]
pub(crate) struct IpcStore(Arc<RwLock<HashMap<u64, Arc<IpcProcess>>>>);

impl IpcStore {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn insert(&self, process: IpcProcess) -> Result<u64, Error> {
		let mut map = self.0.write().expect("Failed to acquire lock");
		if map.values().any(|item| item.name() == process.name()) {
			return Err(Error::Exists("IpcPluginProcess".to_string()));
		}

		let id = IPC_ID.fetch_add(1, Ordering::SeqCst);
		map.insert(id, Arc::new(process));
		Ok(id)
	}

	pub fn all(&self) -> Vec<Arc<IpcProcess>> {
		let map = self.0.read().expect("Failed to acquire lock");
		map.values().cloned().collect()
	}

	pub fn raw(&self) -> Arc<RwLock<HashMap<u64, Arc<IpcProcess>>>> {
		Arc::clone(&self.0)
	}
}
