use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use super::Command;

static COMMAND_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Default)]
pub(crate) struct CommandStore(Arc<Mutex<HashMap<u64, Arc<Command>>>>);

impl CommandStore {
	pub fn insert(&self, builder: Command) {
		let id = COMMAND_ID.fetch_add(1, Ordering::Relaxed);
		self.0.lock().unwrap().insert(id, Arc::from(builder));
	}

	pub fn remove_with_id(&self, key: u64) {
		self.0.lock().unwrap().remove(&key);
	}

	pub fn remove_with_name(&self, name: &str) {
		let mut map = self.0.lock().unwrap();
		if let Some(key) = map.iter().find(|(_, v)| v.builder.name() == name).map(|(k, _)| *k) {
			map.remove(&key);
		}
	}

	pub fn remove_with_plugin_name(&self, plugin_name: &str) {
		self.0.lock().unwrap().retain(|_, registry| registry.plugin_name != plugin_name);
	}
	pub fn get_with_id(&self, key: u64) -> Option<Arc<Command>> {
		self.0.lock().unwrap().get(&key).cloned()
	}

	pub fn get_with_name(&self, name: &str) -> Option<Arc<Command>> {
		let map = self.0.lock().unwrap();
		map.values().find(|registry| registry.builder.name() == name).cloned()
	}

	pub fn get_with_plugin(&self, plugin_name: &str, name: &str) -> Option<Arc<Command>> {
		let map = self.0.lock().unwrap();
		map.values()
			.find(|registry| registry.plugin_name == plugin_name && registry.builder.name() == name)
			.cloned()
	}

	pub fn get_all(&self) -> Vec<Arc<Command>> {
		self.0.lock().unwrap().values().cloned().collect()
	}
}
