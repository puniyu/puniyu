use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicU64, Ordering};
use crate::types::ConfigInfo;
use puniyu_error::registry::Error;

static CONFIG_ID: AtomicU64 = AtomicU64::new(0);
#[derive(Default)]
pub(crate) struct ConfigStore(Arc<RwLock<HashMap<u64, ConfigInfo>>>);

impl ConfigStore {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert(&self, config: ConfigInfo) -> Result<u64, Error> {
        let mut map = self.0.write().expect("Failed to acquire lock");
        if map.values().any(|v| v == &config) {
            return Err(Error::Exists("Config".to_string()));
        }
        let index = CONFIG_ID.fetch_add(1, Ordering::Relaxed);
        map.insert(index, config).ok_or(Error::Exists("Config".to_string()))?;
        Ok(index)
    }
    pub fn all(&self) -> Vec<ConfigInfo> {
        let map = self.0.read().expect("Failed to acquire lock");
        map.values().cloned().collect()
    }
    pub(crate) fn raw(&self) -> Arc<RwLock<HashMap<u64, ConfigInfo>>> {
        self.0.clone()
    }
}
