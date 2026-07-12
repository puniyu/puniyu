mod store;

use crate::Adapter;
use crate::Error;
use crate::types::AdapterId;
use std::sync::{Arc, LazyLock};
use store::AdapterStore;

static STORE: LazyLock<AdapterStore> = LazyLock::new(AdapterStore::new);

pub struct AdapterRegistry;

impl<'a> AdapterRegistry {
	pub fn register(adapter: impl Into<Arc<dyn Adapter>>) -> Result<u64, Error> {
		STORE.insert(adapter.into())
	}

	pub fn unregister<A>(adapter: A) -> Result<(), Error>
	where
		A: Into<AdapterId<'a>>,
	{
		let adapter = adapter.into();
		match adapter {
			AdapterId::Index(index) => Self::unregister_with_index(index),
			AdapterId::Name(name) => Self::unregister_with_adapter_name(name.as_ref()),
		}
	}

	pub fn unregister_with_index(index: u64) -> Result<(), Error> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		if map.get(&index).is_none() {
			return Err(Error::NotFound("Adapter".to_string()));
		}
		map.remove(&index);
		Ok(())
	}

	pub fn unregister_with_adapter_name(name: &str) -> Result<(), Error> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		if !map.values().any(|v| v.adapter_info().name == name) {
			return Err(Error::NotFound("Adapter".to_string()));
		}
		map.retain(|_, v| v.adapter_info().name != name);
		Ok(())
	}

	pub fn get<A>(adapter: A) -> Option<Arc<dyn Adapter>>
	where
		A: Into<AdapterId<'a>>,
	{
		let adapter = adapter.into();
		match adapter {
			AdapterId::Index(index) => Self::get_with_index(index),
			AdapterId::Name(name) => Self::get_with_adapter_name(name.as_ref()),
		}
	}

	pub fn get_with_index(index: u64) -> Option<Arc<dyn Adapter>> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.get(&index).cloned()
	}

	pub fn get_with_adapter_name(name: &str) -> Option<Arc<dyn Adapter>> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.values().find(|v| v.adapter_info().name == name).cloned()
	}

	pub fn all() -> Vec<Arc<dyn Adapter>> {
		STORE.all()
	}
}
