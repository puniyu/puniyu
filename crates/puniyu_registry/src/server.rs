mod store;
mod types;

use std::cmp::PartialEq;
pub use types::*;

use crate::{Error, Result};
use puniyu_types::server::ServerFunction;
use std::sync::LazyLock;
use store::ServerStore;
use crate::SourceType;

static STORE: LazyLock<ServerStore> = LazyLock::new(ServerStore::new);
#[derive(Debug, Default)]
pub struct ServerRegistry;

impl ServerRegistry {
	pub fn register(source: SourceType, server: impl Into<ServerFunction>) -> Result<u64> {
		let server = ServerInfo { source, builder: server };
		STORE.insert(server)
	}

	pub fn get(source: SourceType) -> Option<ServerInfo> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.values().find(|server| server.source == source).cloned()
	}

	pub fn get_all() -> Vec<ServerInfo> {
		STORE.all()
	}

	pub fn unregister<S>(server: S) -> Result<()>
	where
		S: Into<ServerId>,
	{
		let server = server.into();
		match server {
			ServerId::Index(index) => Self::unregister_with_index(index),
			ServerId::Source(source) => Self::unregister_with_source(source),
		}
	}

	pub fn unregister_with_index(index: u64) -> Result<()> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		if map.get(&index).is_none() {
			return Err(Error::NotFound("Command".to_string()));
		}
		map.remove(&index);
		Ok(())
	}
	pub fn unregister_with_source(source: SourceType) -> Result<()> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		map.retain(|_, server| server.source != source);
		Ok(())
	}
}
