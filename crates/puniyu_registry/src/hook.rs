mod store;
use puniyu_types::hook::HookBuilder;
use std::sync::{Arc, LazyLock};
use store::HookStore;

static STORE: LazyLock<HookStore> = LazyLock::new(HookStore::new);

pub struct HookInfo {
	pub index: u64,
	pub builder: Arc<dyn HookBuilder>,
}

pub struct HookRegistry;

impl HookRegistry {
	pub fn register(hook: Arc<dyn HookBuilder>) {
		STORE.insert(hook);
	}
	pub fn unregister(index: u64) {
		STORE.remove_with_index(index)
	}
	pub fn get(name: &str) -> Option<HookInfo> {
		STORE.get(name)
	}
	pub fn all() -> Vec<HookInfo> {
		STORE.all()
	}
}
