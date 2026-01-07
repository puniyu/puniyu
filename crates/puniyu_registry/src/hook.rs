use std::sync::Arc;
use puniyu_types::hook::HookBuilder;
use crate::store::STORE;

pub struct HookRegistry;

impl HookRegistry {
    pub fn register(hook: Arc<dyn HookBuilder>) {
        STORE.hook().insert(hook);
    }
    pub fn unregister(name: &str) {
        STORE.hook().remove_with_name(name)
    }
    pub fn get(name: &str) -> Option<Arc<dyn HookBuilder>> {
        STORE.hook().get(name)
    }
    pub fn all() -> Vec<Arc<dyn HookBuilder>> {
        STORE.hook().all()
    }
}