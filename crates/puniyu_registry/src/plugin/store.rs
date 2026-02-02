use crate::plugin::PluginInfo;
use std::{
    collections::HashMap,
    sync::{
        Arc, RwLock,
        atomic::{AtomicU64, Ordering},
    },
};

static PLUGIN_INDEX: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Default)]
pub(crate) struct PluginStore(pub(crate) Arc<RwLock<HashMap<u64, PluginInfo>>>);

impl PluginStore {
	pub fn new() -> Self {
    	Self::default()
	}
    pub fn insert(&self, plugin: PluginInfo) {
        let mut plugins = self.0.write().unwrap();
        let exists = plugins.values().any(|p| p.name == plugin.name);
        if !exists {
            let index = PLUGIN_INDEX.fetch_add(1, Ordering::Relaxed);
            plugins.insert(index, plugin);
        }
    }

    pub fn all(&self) -> HashMap<u64, PluginInfo> {
        self.0.read().unwrap().clone()
    }

    pub fn get_plugin_with_index(&self, index: u64) -> Option<PluginInfo> {
        self.0.read().unwrap().get(&index).cloned()
    }

    pub fn get_plugin_with_name(&self, name: &str) -> Option<PluginInfo> {
        self.0.read().unwrap().values().find(|plugin| plugin.name == name).cloned()
    }

    pub fn remove(&self, index: u64) -> Option<PluginInfo> {
        self.0.write().unwrap().remove(&index)
    }

    pub fn get_index(&self, name: &str) -> Option<u64> {
        self.0.read().unwrap().iter().find(|(_, plugin)| plugin.name == name).map(|(idx, _)| *idx)
    }
}
