use std::sync::Arc;
use puniyu_types::hook::HookBuilder;
use crate::store::STORE;

pub struct HookInfo {
    pub index: u64,
    pub builder: Arc<dyn HookBuilder>,
}

pub struct HookRegistry;

impl HookRegistry {
    pub fn register(hook: Arc<dyn HookBuilder>) {
        STORE.hook().insert(hook);
    }
    pub fn unregister(index: u64) {
        STORE.hook().remove_with_index(index)
    }
    pub fn get(name: &str) -> Option<HookInfo> {
        STORE.hook().get(name)
    }
    pub fn all() -> Vec<HookInfo> {
        STORE.hook().all()
    }
}