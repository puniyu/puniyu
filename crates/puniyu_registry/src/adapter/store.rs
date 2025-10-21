use puniyu_bot::BotRegistry;
use puniyu_builder::adapter::Adapter;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
#[derive(Default, Clone)]
pub(crate) struct AdapterStore(Arc<Mutex<HashMap<String, Adapter>>>);

impl AdapterStore {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn insert_adapter(&self, name: &str, adapter: Adapter) {
		let mut adapters = self.0.lock().unwrap();
		if !adapters.contains_key(name) {
			adapters.insert(name.to_string(), adapter);
		}
	}

	pub fn get_adapter(&self, name: &str) -> Option<Adapter> {
		let adapters = self.0.lock().unwrap();
		adapters.get(name).cloned()
	}

	pub fn get_all_adapters(&self) -> HashMap<String, Adapter> {
		let adapters = self.0.lock().unwrap();
		adapters.clone()
	}

	pub fn remove_adapter(&self, name: &str) {
		let mut adapters = self.0.lock().unwrap();
		BotRegistry::get_all()
			.into_iter()
			.filter(|bot| bot.adapter.name == name)
			.for_each(|bot| BotRegistry::unregister(bot.index));
		adapters.remove(name);
	}
}
