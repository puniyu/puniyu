use puniyu_error::registry::Error;
use puniyu_server_core::ServerInfo;
use std::{
	collections::HashMap,
	sync::{
		Arc, RwLock,
		atomic::{AtomicU64, Ordering},
	},
};

static SERVER_ID: AtomicU64 = AtomicU64::new(0);

/// 服务器存储
///
/// 用于存储注册的服务器配置。
#[derive(Clone, Default)]
pub(crate) struct ServerStore(pub(crate) Arc<RwLock<HashMap<u64, ServerInfo>>>);

impl ServerStore {
	pub fn new() -> Self {
		Self::default()
	}

	/// 插入服务器配置
	pub fn insert(&self, server: ServerInfo) -> Result<u64, Error> {
		let mut map = self.0.write().expect("Failed to acquire lock");
		if map.values().any(|v| v == &server) {
			return Err(Error::Exists("Server".to_string()));
		}
		let index = SERVER_ID.fetch_add(1, Ordering::Relaxed);
		map.insert(index, server);
		Ok(index)
	}

	/// 获取所有服务器配置
	///
	/// 返回所有配置的克隆。
	pub fn all(&self) -> Vec<ServerInfo> {
		let map = self.0.read().expect("Failed to acquire lock");
		map.values().cloned().collect()
	}

	pub(crate) fn raw(&self) -> Arc<RwLock<HashMap<u64, ServerInfo>>> {
		self.0.clone()
	}
}
