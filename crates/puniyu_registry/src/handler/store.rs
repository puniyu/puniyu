use puniyu_types::handler::Handler;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicU64, Ordering};

static HANDLER_INDEX: AtomicU64 = AtomicU64::new(0);

#[derive( Default)]
pub(crate) struct HandlerStore(pub(crate) Arc<RwLock<HashMap<u64, Arc<dyn Handler>>>>);

impl HandlerStore {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn register(&self, handler: Arc<dyn Handler>) {
        let mut handlers = self.0.write().unwrap();
        let index = HANDLER_INDEX.fetch_add(1, Ordering::Relaxed);
        handlers.insert(index, handler);
    }

    pub fn unregister(&self, name: &str) -> bool {
        let mut handlers = self.0.write().unwrap();
        let original_len = handlers.len();

        handlers.retain(|_, handler| handler.name() != name);

        handlers.len() != original_len
    }

    pub fn get(&self, name: &str) -> Option<Arc<dyn Handler>> {
        let handlers = self.0.read().unwrap();
        handlers.values().find(|handler| handler.name() == name).cloned()
    }

    pub fn all(&self) -> Vec<Arc<dyn Handler>> {
        let handlers = self.0.read().unwrap();
        handlers.values().cloned().collect()
    }
}
