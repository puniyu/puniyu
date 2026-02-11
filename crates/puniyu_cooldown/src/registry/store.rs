use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use puniyu_error::registry::Error;

#[derive(Default)]
pub(crate) struct CooldownStore(Arc<RwLock<HashMap<String, u64>>>);

impl CooldownStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&self, cooldown: String, time: u64) -> Result<(), Error> {
        let mut map = self.0.write().expect("Failed to acquire lock");
        if map.get(&cooldown).is_some() {
            return Err(Error::Exists("Cooldown".to_string()));
        }
        map.insert(cooldown, time);
        Ok(())
    }

    pub(crate) fn raw(&self) -> Arc<RwLock<HashMap<String, u64>>> {
        self.0.clone()
    }
}
