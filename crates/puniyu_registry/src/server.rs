use crate::store::STORE;
use puniyu_types::server::ServerType;

#[derive(Debug, Default)]
pub struct ServerRegistry;

impl ServerRegistry {
	pub fn insert(name: impl Into<String>, server: ServerType) {
		STORE.server().insert(name, server);
	}

	pub fn get(name: &str) -> Option<ServerType> {
		STORE.server().get(name)
	}

	pub fn get_all() -> Vec<ServerType> {
		STORE.server().get_all()
	}

	pub fn remove(name: &str) {
		STORE.server().remove(name);
	}
}