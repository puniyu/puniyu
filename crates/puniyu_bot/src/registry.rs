use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, LazyLock};

use puniyu_registry::{Entry, Registry};

use crate::{Bot, BotId};

pub struct BotRegistry {
	inner: Registry<Arc<dyn Bot>>,
	next_id: AtomicU64,
}

impl Clone for BotRegistry {
	fn clone(&self) -> Self {
		Self {
			inner: self.inner.clone(),
			next_id: AtomicU64::new(self.next_id.load(Ordering::Relaxed)),
		}
	}
}

impl Default for BotRegistry {
	fn default() -> Self {
		static STORE: LazyLock<Registry<Arc<dyn Bot>>> = LazyLock::new(Registry::new);
		Self {
			inner: STORE.clone(),
			next_id: AtomicU64::new(0),
		}
	}
}

impl BotRegistry {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn insert(&self, bot: Arc<dyn Bot>) -> u64 {
		loop {
			let id = self.next_id.fetch_add(1, Ordering::Relaxed);
			match self.inner.entry(id) {
				Entry::Occupied(_) => continue,
				Entry::Vacant(entry) => {
					entry.insert_entry(bot);
					return id;
				}
			}
		}
	}
	pub fn get(&self, bot_id: BotId) -> Option<Arc<dyn Bot>> {
		match bot_id {
			BotId::Index(index) => self.inner.get(index),
			BotId::SelfId(id) => {
				let mut bot = None;
				self.inner.for_each(|_, b| {
					if b.id() == id {
						bot = Some(b.clone());
					}
				});
				bot
			}
		}
	}

	pub fn values(&self) -> Vec<Arc<dyn Bot>> {
		self.inner.values()
	}

	pub fn remove(&self, bot_id: BotId) -> Option<Arc<dyn Bot>> {
		match bot_id {
			BotId::Index(index) => self.inner.remove(index),
			BotId::SelfId(id) => {
				let mut key = None;
				self.inner.for_each(|k, b| {
					if b.id() == id {
						key = Some(k);
					}
				});
				key.and_then(|k| self.inner.remove(k))
			}
		}
	}
}
