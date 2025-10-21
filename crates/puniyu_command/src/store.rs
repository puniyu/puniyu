use crate::Command;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

static COMMAND_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Default)]
pub struct CommandStore(Arc<Mutex<HashMap<u64, Arc<Command>>>>);

impl CommandStore {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn insert(&self, builder: Command) {
		let id = COMMAND_ID.fetch_add(1, Ordering::Relaxed);
		self.0.lock().unwrap().insert(id, Arc::from(builder));
	}

	pub fn remove_with_id(&self, key: u64) {
		self.0.lock().unwrap().remove(&key);
	}

	pub fn remove_with_name(&self, name: &str) {
		let mut map = self.0.lock().unwrap();
		if let Some(key) =
			map.iter().find_map(|(k, v)| if v.builder.name() == name { Some(*k) } else { None })
		{
			map.remove(&key);
		}
	}

	pub fn remove_with_plugin_name(&self, plugin_name: &str) {
		let mut map = self.0.lock().unwrap();
		let keys_to_remove: Vec<u64> = map
			.iter()
			.filter(|(_, registry)| registry.plugin_name == plugin_name)
			.map(|(key, _)| *key)
			.collect();

		for key in keys_to_remove {
			map.remove(&key);
		}
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
