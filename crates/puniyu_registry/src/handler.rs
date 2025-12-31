use crate::store::STORE;
use puniyu_types::handler::Handler;
use std::sync::Arc;

#[derive(Default)]
pub struct HandlerRegistry;

impl HandlerRegistry {
	pub fn register(handler: Arc<dyn Handler>) {
		STORE.handler().register(handler)
	}

	pub fn unregister(name: &str) -> bool {
		STORE.handler().unregister(name)
	}

	pub fn get(name: &str) -> Option<Arc<dyn Handler>> {
		STORE.handler().get(name)
	}

	pub fn handlers() -> Vec<Arc<dyn Handler>> {
		STORE.handler().all()
	}
}
