use puniyu_bot::BotRegistry;
use puniyu_builder::adapter::Adapter;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

static ADAPTER_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Default, Clone)]
pub(crate) struct AdapterStore(Arc<Mutex<HashMap<u64, Adapter>>>);

impl AdapterStore {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn insert_adapter(&self, adapter: Adapter) {
		let mut adapters = self.0.lock().unwrap();
		let exists = adapters.values().any(|a| a.info.name == adapter.info.name);
		if !exists {
			let id = ADAPTER_COUNTER.fetch_add(1, Ordering::Relaxed);
			adapters.insert(id, adapter);
		}
	}

	pub fn get_adapter(&self, name: &str) -> Option<Adapter> {
		let adapters = self.0.lock().unwrap();
		adapters.values().find(|a| a.info.name == name).cloned()
	}

	pub fn get_all_adapters(&self) -> HashMap<u64, Adapter> {
		let adapters = self.0.lock().unwrap();
		adapters.clone()
	}

	pub fn remove_adapter(&self, name: &str) {
		let mut adapters = self.0.lock().unwrap();

		let ids = adapters
			.iter()
			.filter(|(_, adapter)| adapter.info.name == name)
			.map(|(id, _)| *id)
			.collect::<Vec<u64>>();

		BotRegistry::get_all()
			.into_iter()
			.filter(|bot| bot.adapter.name == name)
			.for_each(|bot| BotRegistry::unregister_with_id(bot.account.self_id.as_str()));
		for id in ids {
			adapters.remove(&id);
		}
	}
}
