use super::Handler;
use std::cmp::Reverse;
use std::sync::{Arc, RwLock};

#[derive(Default)]
pub struct HandlerRegistry(RwLock<Vec<Arc<dyn Handler>>>);

impl HandlerRegistry {
	pub fn new() -> Self {
		Self::default()
	}

	/// 注册事件处理器
	pub fn register(&mut self, handler: Arc<dyn Handler>) {
		let mut vec = self.0.write().unwrap();
		vec.push(handler);
		vec.sort_by_key(|h| Reverse(h.rank()));
	}

	/// 卸载事件处理器
	pub fn unregister(&mut self, name: &str) -> Option<Arc<dyn Handler>> {
		let mut vec = self.0.write().unwrap();
		if let Some(pos) = vec.iter().position(|h| h.name() == name) {
			Some(vec.remove(pos))
		} else {
			None
		}
	}

	pub fn get_all(&self) -> Vec<Arc<dyn Handler>> {
		let vec = self.0.read().unwrap();
		vec.iter().cloned().collect()
	}

	pub fn get_with_name(&self, name: &str) -> Option<Arc<dyn Handler>> {
		let vec = self.0.read().unwrap();
		vec.iter().find(|h| h.name() == name).cloned()
	}
}
