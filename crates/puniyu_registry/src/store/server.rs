use puniyu_types::server::ServerType;
use std::{
	collections::HashMap,
	sync::{Arc, RwLock},
};

#[derive(Clone)]
pub(crate) struct ServerStore(pub(crate) Arc<RwLock<HashMap<String, ServerType>>>);

impl ServerStore {
	pub fn insert(&self, name: impl Into<String>, server: ServerType) {
		self.0.write().unwrap().insert(name.into(), server);
	}

	pub fn get(&self, name: &str) -> Option<ServerType> {
		self.0.read().unwrap().get(name).cloned()
	}

	pub fn get_all(&self) -> Vec<ServerType> {
		self.0.read().unwrap().values().cloned().collect()
	}

	pub fn remove(&self, name: &str) {
		self.0.write().unwrap().remove(name);
	}
}
