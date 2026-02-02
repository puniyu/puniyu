mod store;
use store::HandlerStore;

use puniyu_types::handler::Handler;
use std::sync::{Arc, LazyLock};

static STORE: LazyLock<HandlerStore> = LazyLock::new(|| HandlerStore::new());
#[derive(Default)]
pub struct HandlerRegistry;

impl HandlerRegistry {
	pub fn register(handler: Arc<dyn Handler>) {
		STORE.register(handler)
	}

	pub fn unregister(name: &str) -> bool {
		STORE.unregister(name)
	}

	pub fn get(name: &str) -> Option<Arc<dyn Handler>> {
		STORE.get(name)
	}

	pub fn handlers() -> Vec<Arc<dyn Handler>> {
		STORE.all()
	}
}
