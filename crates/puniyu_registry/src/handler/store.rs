use crate::{Error, Result};
use puniyu_types::handler::Handler;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

static HANDLER_INDEX: AtomicU64 = AtomicU64::new(0);

#[derive(Default)]
pub(crate) struct HandlerStore(pub(crate) Arc<RwLock<HashMap<u64, Arc<dyn Handler>>>>);

impl HandlerStore {
	pub fn new() -> Self {
		Self::default()
	}
	pub fn insert(&self, handler: Arc<dyn Handler>) -> Result<u64> {
		let mut map = self.0.write().expect("Failed to acquire lock");
		if map.values().any(|v| v == handler) {
			return Err(Error::Exists("Handler".to_string()));
		}
		let index = HANDLER_INDEX.fetch_add(1, Ordering::Relaxed);
		map.insert(index, handler);
		Ok(index)
	}

	pub fn get(&self, name: &str) -> Option<Arc<dyn Handler>> {
		let handlers = self.0.read().unwrap();
		handlers.values().find(|handler| handler.name() == name).cloned()
	}

	pub fn all(&self) -> Vec<Arc<dyn Handler>> {
		let handlers = self.0.read().unwrap();
		handlers.values().cloned().collect()
	}

	pub(crate) fn raw(&self) -> Arc<RwLock<HashMap<u64, Arc<dyn Handler>>>> {
		self.0.clone()
	}
}
