use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, LazyLock};

use puniyu_registry::{Entry, Registry};

use crate::{Plugin, PluginId};





pub struct PluginRegistry {
	inner: Arc<Registry<Arc<dyn Plugin>>>,
	next_id: Arc<AtomicU64>,
}

impl Clone for PluginRegistry {
	fn clone(&self) -> Self {
		Self {
			inner: self.inner.clone(),
			next_id: self.next_id.clone(),
		}
	}
}

impl Default for PluginRegistry {
	fn default() -> Self {
		static INNER: LazyLock<Arc<Registry<Arc<dyn Plugin>>>> =
			LazyLock::new(|| Arc::new(Registry::new()));
		static NEXT_ID: LazyLock<Arc<AtomicU64>> =
			LazyLock::new(|| Arc::new(AtomicU64::new(0)));
		Self {
			inner: INNER.clone(),
			next_id: NEXT_ID.clone(),
		}
	}
}

impl PluginRegistry {
	pub fn new() -> Self {
		Self::default()
	}

	/// 注册插件，返回分配的 ID。
	pub fn insert(&self, plugin: Arc<dyn Plugin>) -> u64 {
		loop {
			let id = self.next_id.fetch_add(1, Ordering::Relaxed);
			match self.inner.entry(id) {
				Entry::Occupied(_) => continue,
				Entry::Vacant(entry) => {
					entry.insert_entry(plugin);
					return id;
				}
			}
		}
	}

	/// 按 ID 或名称查找插件。
	pub fn get(&self, id: PluginId) -> Option<Arc<dyn Plugin>> {
		match id {
			PluginId::Index(index) => self.inner.get(index),
			PluginId::Name(name) => {
				let mut plugin = None;
				self.inner.for_each(|_, p| {
					if p.name() == name {
						plugin = Some(p.clone());
					}
				});
				plugin
			}
		}
	}

	/// 按 ID 或名称移除插件。
	pub fn remove(&self, id: PluginId) -> Option<Arc<dyn Plugin>> {
		match id {
			PluginId::Index(index) => self.inner.remove(index),
			PluginId::Name(name) => {
				let mut key = None;
				self.inner.for_each(|k, p| {
					if p.name() == name {
						key = Some(k);
					}
				});
				key.and_then(|k| self.inner.remove(k))
			}
		}
	}

	/// 获取所有插件。
	pub fn values(&self) -> Vec<Arc<dyn Plugin>> {
		self.inner.values()
	}
}
