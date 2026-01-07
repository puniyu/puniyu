use puniyu_types::hook::HookBuilder;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

static HOOK_INDEX: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Default)]
pub(crate) struct HookStore(pub(crate) Arc<RwLock<HashMap<u64, Arc<dyn HookBuilder>>>>);

impl HookStore {
	pub fn insert(&self, hook: Arc<dyn HookBuilder>) {
		let mut hooks = self.0.write().unwrap();
		let exists = hooks.values().any(|a| a.name() == hook.name());
		if !exists {
			let id = HOOK_INDEX.fetch_add(1, Ordering::Relaxed);
			hooks.insert(id, hook);
		}
	}

	pub fn get(&self, name: &str) -> Option<Arc<dyn HookBuilder>> {
		let adapters = self.0.read().unwrap();
		adapters.values().find(|a| a.name() == name).cloned()
	}

	pub fn all(&self) -> Vec<Arc<dyn HookBuilder>> {
		self.0.read().unwrap().values().cloned().collect()
	}

	pub fn remove_with_id(&self, key: u64) {
		self.0.write().unwrap().remove(&key);
	}

	pub fn remove_with_name(&self, name: &str) {
		let mut map = self.0.write().unwrap();
		if let Some(key) = map.iter().find(|(_, v)| v.name() == name).map(|(k, _)| *k) {
			map.remove(&key);
		}
	}
}
