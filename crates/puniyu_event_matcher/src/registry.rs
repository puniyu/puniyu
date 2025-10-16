use super::Matcher;
use std::sync::{Arc, RwLock};

#[derive(Default)]
pub struct MatcherRegistry(RwLock<Vec<Arc<dyn Matcher>>>);

impl MatcherRegistry {
	pub fn new() -> Self {
		Self::default()
	}

	/// 注册事件匹配器
	pub fn register(&mut self, handler: Arc<dyn Matcher>) {
		let mut vec = self.0.write().unwrap();
		vec.push(handler);
		vec.sort_by_key(|b| std::cmp::Reverse(b.rank()));
	}

	/// 卸载事件匹配器
	pub fn unregister(&mut self, name: &str) -> Option<Arc<dyn Matcher>> {
		let mut vec = self.0.write().unwrap();
		if let Some(pos) = vec.iter().position(|h| h.name() == name) {
			Some(vec.remove(pos))
		} else {
			None
		}
	}

	pub fn get_all(&self) -> Vec<Arc<dyn Matcher>> {
		let vec = self.0.read().unwrap();
		vec.iter().cloned().collect()
	}

	pub fn get_with_name(&self, name: &str) -> Option<Arc<dyn Matcher>> {
		let vec = self.0.read().unwrap();
		vec.iter().find(|h| h.name() == name).cloned()
	}
}
