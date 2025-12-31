use puniyu_types::handler::Handler;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

pub(crate) struct HandlerStore(pub(crate) Arc<RwLock<BTreeMap<u32, Arc<dyn Handler>>>>);

impl HandlerStore {
	pub fn register(&self, handler: Arc<dyn Handler>) {
		let mut handlers = self.0.write().unwrap();
		handlers.insert(handler.rank(), handler);
	}

	pub fn unregister(&self, name: &str) -> bool {
		let mut handlers = self.0.write().unwrap();
		let original_len = handlers.len();

		handlers.retain(|_, handler| handler.name() != name);

		handlers.len() != original_len
	}

	pub fn get(&self, name: &str) -> Option<Arc<dyn Handler>> {
		let handlers = self.0.read().unwrap();
		handlers.values().find(|handler| handler.name() == name).cloned()
	}

	pub fn all(&self) -> Vec<Arc<dyn Handler>> {
		let handlers = self.0.read().unwrap();
		handlers.values().cloned().collect()
	}
}
