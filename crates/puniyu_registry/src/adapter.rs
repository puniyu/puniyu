mod store;
mod types;

use store::AdapterStore;

use crate::{Error, Result};
use convert_case::Casing;
use puniyu_logger::owo_colors::OwoColorize;
use puniyu_types::adapter::Adapter;
use std::future::Future;
use std::sync::{Arc, LazyLock};
use types::AdapterId;

static STORE: LazyLock<AdapterStore> = LazyLock::new(AdapterStore::new);

pub struct AdapterRegistry;

impl AdapterRegistry {
	/// 注册一个适配器
	pub fn register<A>(adapter: A) -> Result<u64>
	where
		A: Into<Arc<dyn Adapter>>,
	{
		let adapter = adapter.into();
		STORE.insert(adapter)
	}

	/// 卸载一个适配器
	pub fn unregister<A>(adapter: A) -> Result<()>
	where
		A: Into<AdapterId>,
	{
		let adapter = adapter.into();
		match adapter {
			AdapterId::Index(index) => Self::unregister_with_index(index),
			AdapterId::Name(name) => Self::unregister_with_adapter_name(&name),
		}
	}

	pub fn unregister_with_index(index: u64) -> Result<()> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		if map.get(&index).is_none() {
			return Err(Error::NotFound("Adapter".to_string()));
		}
		map.remove(&index);
		Ok(())
	}

	pub fn unregister_with_adapter_name(name: &str) -> Result<()> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		if !map.values().any(|v| v.info().name == name) {
			return Err(Error::NotFound("Adapter".to_string()));
		}
		map.retain(|_, v| v.info().name != name);
		Ok(())
	}

	pub fn get<A>(adapter: A) -> Vec<Arc<dyn Adapter>>
	where
		A: Into<AdapterId>,
	{
		let adapter = adapter.into();
		match adapter {
			AdapterId::Index(index) => Self::get_with_index(index).into_iter().collect(),
			AdapterId::Name(name) => Self::get_with_adapter_name(&name),
		}
	}

	pub fn get_with_index(index: u64) -> Option<Arc<dyn Adapter>> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.get(&index).cloned()
	}

	pub fn get_with_adapter_name(name: &str) -> Vec<Arc<dyn Adapter>> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.values().filter(|v| v.info().name == name).collect()
	}

	pub fn all() -> Vec<Arc<dyn Adapter>> {
		STORE.all()
	}
}
