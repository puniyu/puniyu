use actix_ws::Session;
use std::sync::{Arc, LazyLock};
use tokio::sync::Mutex;
use super::store::ConnectionStore;

static CONNECTION_STORE: LazyLock<ConnectionStore> = LazyLock::new(ConnectionStore::new);

pub struct ConnectionManager;

impl ConnectionManager {

	pub fn add(bot_name: String, session: Arc<Mutex<Session>>) {
		CONNECTION_STORE.add(bot_name, session);
	}

	pub fn remove(bot_name: &str) {
		CONNECTION_STORE.remove(bot_name);
	}

	pub fn get(bot_name: &str) -> Option<Arc<Mutex<Session>>> {
		CONNECTION_STORE.get(bot_name)
	}

	pub fn get_all() -> Vec<(String, Arc<Mutex<Session>>)> {
		CONNECTION_STORE.get_all()
	}

}

