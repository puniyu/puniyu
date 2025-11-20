mod command;
pub use command::CommandHandler;
use super::Handler;
use super::store::HandlerStore;
use std::sync::{Arc, LazyLock};

static MATCHER_STORE: LazyLock<HandlerStore> = LazyLock::new(HandlerStore::default);

pub struct HandlerRegistry;

impl HandlerRegistry {
	/// 注册事件匹配器
	pub fn register(handler: Arc<dyn Handler>) {
		MATCHER_STORE.insert(handler);
	}

	/// 卸载事件匹配器
	pub fn unregister(&mut self, name: &str) {
		MATCHER_STORE.remove(name);
	}

	pub fn get_all(&self) -> Vec<Arc<dyn Handler>> {
		MATCHER_STORE.get_all()
	}

	pub fn get_with_name(&self, name: &str) -> Option<Arc<dyn Handler>> {
		MATCHER_STORE.get_with_name(name)
	}
}
