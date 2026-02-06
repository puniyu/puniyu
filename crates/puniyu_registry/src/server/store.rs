use super::ServerInfo;
use crate::{Error, Result};
use std::{
	collections::HashMap,
	sync::{
		Arc, RwLock,
		atomic::{AtomicU64, Ordering},
	},
};

static SERVER_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Default)]
pub(crate) struct ServerStore(pub(crate) Arc<RwLock<HashMap<u64, ServerInfo>>>);

impl ServerStore {
	pub fn new() -> Self {
		Self::default()
	}
	pub fn insert(&self, server: ServerInfo) -> Result<u64> {
		let mut map = self.0.write().expect("Failed to acquire lock");
		if map.values().any(|v| v.source == server.source && v.builder == server.builder) {
			return Err(Error::Exists("Server".to_string()));
		}
		let index = SERVER_ID.fetch_add(1, Ordering::Relaxed);
		map.insert(index, server);
		Ok(index)
	}

	pub fn all(&self) -> Vec<ServerInfo> {
		let map = self.0.read().expect("Failed to acquire lock");
		map.values().cloned().collect()
	}

	pub(crate) fn raw(&self) -> Arc<RwLock<HashMap<u64, ServerInfo>>> {
		self.0.clone()
	}
}
