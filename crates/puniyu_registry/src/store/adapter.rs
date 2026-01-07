use crate::bot::BotRegistry;
use puniyu_types::adapter::Adapter;
use std::{
	collections::HashMap,
	sync::{
		Arc, RwLock,
		atomic::{AtomicU64, Ordering},
	},
};

static ADAPTER_INDEX: AtomicU64 = AtomicU64::new(0);

#[derive(Clone)]
pub(crate) struct AdapterStore(pub(crate) Arc<RwLock<HashMap<u64, Adapter>>>);

impl AdapterStore {
	pub fn insert(&self, adapter: Adapter) {
		let mut adapters = self.0.write().unwrap();
		let exists = adapters.values().any(|a| a.name == adapter.name);
		if !exists {
			let id = ADAPTER_INDEX.fetch_add(1, Ordering::Relaxed);
			adapters.insert(id, adapter);
		}
	}

	pub fn get(&self, name: &str) -> Option<Adapter> {
		let adapters = self.0.read().unwrap();
		adapters.values().find(|a| a.name == name).cloned()
	}

	pub fn all(&self) -> HashMap<u64, Adapter> {
		self.0.read().unwrap().clone()
	}

	pub fn remove(&self, name: &str) {
		let mut adapters = self.0.write().unwrap();

		let ids = adapters
			.iter()
			.filter(|(_, adapter)| adapter.name == name)
			.map(|(id, _)| *id)
			.collect::<Vec<u64>>();

		BotRegistry::get_all().into_iter().filter(|bot| bot.adapter.name == name).for_each(|bot| {
			BotRegistry::unregister_with_id(bot.account.uin);
		});
		for id in ids {
			adapters.remove(&id);
		}
	}
}
