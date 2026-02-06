use super::HookInfo;
use crate::{Error, Result, SourceType};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

static HOOK_INDEX: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Default)]
pub(crate) struct HookStore(Arc<RwLock<HashMap<u64, HookInfo>>>);

impl HookStore {
	pub fn new() -> Self {
		Self::default()
	}
	pub fn insert(&self, hook: HookInfo) -> Result<u64> {
		let mut map = self.0.write().expect("Failed to acquire lock");
		if map.values().any(|v| v == &hook) {
			return Err(Error::Exists("Hook".to_string()));
		}
		let index = HOOK_INDEX.fetch_add(1, Ordering::Relaxed);
		map.insert(index, hook);
		Ok(index)
	}

	pub fn get(&self, source: SourceType) -> Option<HookInfo> {
		let map = &self.0.read().expect("Failed to acquire lock");
		map.values().find(|h| h.source == source).cloned()
	}

	pub fn all(&self) -> Vec<HookInfo> {
		let map = &self.0.read().expect("Failed to acquire lock");
		map.values().cloned().collect()
	}

	pub(crate) fn raw(&self) -> Arc<RwLock<HashMap<u64, HookInfo>>> {
		self.0.clone()
	}
}
