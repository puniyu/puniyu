use crate::{Error, ServerId, ServerInfo, SourceType};
use salvo::Router;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicU64, Ordering};
mod store;
use store::ServerStore;

static STORE: LazyLock<ServerStore> = LazyLock::new(ServerStore::new);
static SERVER_ID: AtomicU64 = AtomicU64::new(0);

pub struct ServerRegistry;

impl ServerRegistry {
	pub fn register(source: SourceType, router: Router) -> Result<u64, Error> {
		let mut map = STORE.raw().write().expect("Failed to acquire lock");
		if map.values().any(|v| v.source == source) {
			return Err(Error::Exists("Server".to_string()));
		}
		let id = SERVER_ID.fetch_add(1, Ordering::Relaxed);
		map.insert(id, ServerInfo { source, router });
		Ok(id)
	}

	pub fn get(source: SourceType) -> Option<ServerInfo> {
		let mut map = STORE.raw().write().expect("Failed to acquire lock");
		let key = map.iter().find(|(_, v)| v.source == source).map(|(k, _)| *k);
		key.and_then(|k| map.remove(&k))
	}

	pub fn take_all() -> Vec<ServerInfo> {
		let mut map = STORE.raw().write().expect("Failed to acquire lock");
		map.drain().map(|(_, v)| v).collect()
	}

	pub fn unregister(server: impl Into<ServerId>) -> Result<(), Error> {
		let mut map = STORE.raw().write().expect("Failed to acquire lock");
		match server.into() {
			ServerId::Index(index) => {
				map.remove(&index).map(|_| ()).ok_or_else(|| Error::NotFound("Server".to_string()))
			}
			ServerId::Source(source) => {
				let count = map.len();
				map.retain(|_, v| v.source != source);
				if map.len() == count { Err(Error::NotFound("Server".to_string())) } else { Ok(()) }
			}
		}
	}
}
