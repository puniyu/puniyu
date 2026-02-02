use crate::bot::BotRegistry;
use crate::adapter::AdapterInfo;
use std::{
    collections::HashMap,
    sync::{
        Arc, RwLock,
        atomic::{AtomicU64, Ordering},
    },
};

static ADAPTER_INDEX: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Default)]
pub(crate) struct AdapterStore(pub(crate) Arc<RwLock<HashMap<u64, AdapterInfo>>>);

impl AdapterStore {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert(&self, adapter: AdapterInfo) {
        let mut adapters = self.0.write().unwrap();
        let exists = adapters.values().any(|a| a.name == adapter.name);
        if !exists {
            let id = ADAPTER_INDEX.fetch_add(1, Ordering::Relaxed);
            adapters.insert(id, adapter);
        }
    }

    pub fn get(&self, name: &str) -> Option<AdapterInfo> {
        let adapters = self.0.read().unwrap();
        adapters.values().find(|a| a.name == name).cloned()
    }

    pub fn all(&self) -> HashMap<u64, AdapterInfo> {
        self.0.read().unwrap().clone()
    }

    pub fn remove(&self, name: &str) {
        let mut adapters = self.0.write().unwrap();

        BotRegistry::all().into_iter().filter(|bot| bot.adapter.name == name).for_each(|bot| {
            BotRegistry::unregister_with_id(bot.account.uin);
        });
        adapters.retain(|_, adapter| adapter.name != name);
    }
}
