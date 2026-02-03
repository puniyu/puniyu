mod store;

use puniyu_types::server::ServerType;
use std::sync::LazyLock;
use store::ServerStore;

static STORE: LazyLock<ServerStore> = LazyLock::new(ServerStore::new);
#[derive(Debug, Default)]
pub struct ServerRegistry;

impl ServerRegistry {
	pub fn insert(name: impl Into<String>, server: impl Into<ServerType>) {
		STORE.insert(name, server.into());
	}

	pub fn get(name: &str) -> Option<ServerType> {
		STORE.get(name)
	}

	pub fn get_all() -> Vec<ServerType> {
		STORE.all()
	}

	pub fn remove(name: &str) {
		STORE.remove(name);
	}
}
