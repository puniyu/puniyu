use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, LazyLock};

use puniyu_registry::{Entry, Registry};

use crate::{Action, ActionId};

pub struct ActionRegistry {
	inner: Arc<Registry<Arc<dyn Action>>>,
	next_id: Arc<AtomicU64>,
}

impl Clone for ActionRegistry {
	fn clone(&self) -> Self {
		Self {
			inner: self.inner.clone(),
			next_id: self.next_id.clone(),
		}
	}
}

impl Default for ActionRegistry {
	fn default() -> Self {
		static INNER: LazyLock<Arc<Registry<Arc<dyn Action>>>> =
			LazyLock::new(|| Arc::new(Registry::new()));
		static NEXT_ID: LazyLock<Arc<AtomicU64>> =
			LazyLock::new(|| Arc::new(AtomicU64::new(0)));
		Self {
			inner: INNER.clone(),
			next_id: NEXT_ID.clone(),
		}
	}
}

impl ActionRegistry {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn insert(&self, action: Arc<dyn Action>) -> u64 {
		loop {
			let id = self.next_id.fetch_add(1, Ordering::Relaxed);
			match self.inner.entry(id) {
				Entry::Occupied(_) => continue,
				Entry::Vacant(entry) => {
					entry.insert_entry(action);
					return id;
				}
			}
		}
	}

	pub fn get(&self, id: ActionId) -> Option<Arc<dyn Action>> {
		match id {
			ActionId::Id(index) => self.inner.get(index),
			ActionId::Name(name) => {
				let mut action = None;
				self.inner.for_each(|_, a| {
					if a.name() == &*name {
						action = Some(a.clone());
					}
				});
				action
			}
		}
	}

	pub fn remove(&self, id: ActionId) -> Option<Arc<dyn Action>> {
		match id {
			ActionId::Id(index) => self.inner.remove(index),
			ActionId::Name(name) => {
				let mut key = None;
				self.inner.for_each(|k, a| {
					if a.name() == &*name {
						key = Some(k);
					}
				});
				key.and_then(|k| self.inner.remove(k))
			}
		}
	}

	pub fn values(&self) -> Vec<Arc<dyn Action>> {
		self.inner.values()
	}
}
