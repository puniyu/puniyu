mod store;

use puniyu_types::server::ServerType;
use std::sync::LazyLock;


static SERVER_STORE: LazyLock<store::ServerStore> = LazyLock::new(store::ServerStore::default);

#[derive(Debug, Default)]
pub struct ServerRegistry;

impl ServerRegistry {
	pub fn insert(name: impl Into<String>, server: ServerType) {
		SERVER_STORE.insert(name, server);
	}

	pub fn get(name: &str) -> Option<ServerType> {
		SERVER_STORE.get(name)
	}

	pub fn get_all() -> Vec<ServerType> {
		SERVER_STORE.get_all()
	}

	pub fn remove(name: &str) {
		SERVER_STORE.remove(name);
	}
}