use puniyu_types::server::ServerType;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Default, Clone)]
pub(crate) struct ServerStore(Arc<Mutex<HashMap<String, ServerType>>>);

impl ServerStore {
	pub fn insert(&self, name: impl Into<String>, server: ServerType) {
		let mut services = self.0.lock().unwrap();
		services.insert(name.into(), server);
	}

	pub fn get(&self, name: &str) -> Option<ServerType> {
		let services = self.0.lock().unwrap();
		services.get(name).cloned()
	}

	pub fn get_all(&self) -> Vec<ServerType> {
		let services = self.0.lock().unwrap();
		services.values().cloned().collect()
	}

	pub fn remove(&self, name: &str) {
		let mut services = self.0.lock().unwrap();
		services.remove(name);
	}
}
