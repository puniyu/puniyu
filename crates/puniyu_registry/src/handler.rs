mod store;
mod types;
pub use types::*;

use store::HandlerStore;

use crate::Result;
use puniyu_types::handler::Handler;
use std::sync::{Arc, LazyLock};

static STORE: LazyLock<HandlerStore> = LazyLock::new(|| HandlerStore::new());

pub struct HandlerRegistry;

impl HandlerRegistry {
	pub fn register(handler: Arc<dyn Handler>) -> Result<u64> {
		STORE.insert(handler)
	}

	pub fn unregister<H>(handler: H) -> Result<()>
	where
		H: Into<HandlerId>,
	{
		let handler = handler.into();
		match handler {
			HandlerId::Index(index) => Self::unregister_with_index(index),
			HandlerId::Name(name) => Self::unregister_with_handler_name(&name),
		}
	}
	pub fn unregister_with_index(index: u64) -> Result<()> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		map.remove(&index);
		Ok(())
	}

	pub fn unregister_with_handler_name(name: &str) -> Result<()> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		map.retain(|_, handler| handler.name() != name);
		Ok(())
	}

	pub fn get<H>(handler: H) -> Option<Arc<dyn Handler>>
	where
		H: Into<HandlerId>,
	{
		let handler = handler.into();
		match handler {
			HandlerId::Index(index) => Self::get_with_index(index),
			HandlerId::Name(name) => Self::get_with_handler_name(&name),
		}
	}
	pub fn get_with_index(index: u64) -> Option<Arc<dyn Handler>> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.get(&index).cloned()
	}

	pub fn get_with_handler_name(name: &str) -> Option<Arc<dyn Handler>> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.values().find(|handler| handler.name() == name).cloned()
	}

	pub fn all() -> Vec<Arc<dyn Handler>> {
		STORE.all()
	}
}
