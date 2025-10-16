use crate::builder::CommandBuilder;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct CommandStore(Arc<Mutex<HashMap<u64, Arc<dyn CommandBuilder>>>>);
impl CommandStore {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn insert(&self, key: u64, builder: Box<dyn CommandBuilder>) {
		self.0.lock().unwrap().insert(key, Arc::from(builder));
	}

	pub fn remove(&self, key: u64) -> Option<Arc<dyn CommandBuilder>> {
		self.0.lock().unwrap().remove(&key)
	}

	pub fn get(&self, key: u64) -> Option<Arc<dyn CommandBuilder>> {
		self.0.lock().unwrap().get(&key).cloned()
	}
}
