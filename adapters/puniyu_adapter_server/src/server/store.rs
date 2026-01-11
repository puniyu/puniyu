use actix_ws::Session;
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ConnectionStore {
	connections: DashMap<String, Arc<Mutex<Session>>>,
}

impl ConnectionStore {
	pub fn new() -> Self {
		Self { connections: DashMap::new() }
	}

	pub fn add(&self, bot_name: String, session: Arc<Mutex<Session>>) {
		self.connections.insert(bot_name, session);
	}

	pub fn remove(&self, bot_name: &str) {
		self.connections.remove(bot_name);
	}

	pub fn get(&self, bot_name: &str) -> Option<Arc<Mutex<Session>>> {
		self.connections.get(bot_name).map(|v| Arc::clone(&v))
	}

	pub fn get_all(&self) -> Vec<(String, Arc<Mutex<Session>>)> {
		self.connections
			.iter()
			.map(|entry| (entry.key().to_string(), Arc::clone(entry.value())))
			.collect()
	}
}
