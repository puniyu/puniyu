use crate::Matcher;
use std::sync::{Arc, RwLock};

#[derive(Default)]
pub(crate) struct MatcherStore(RwLock<Vec<Arc<dyn Matcher>>>);

impl MatcherStore {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn insert(&self, matcher: Arc<dyn Matcher>) {
		let mut vec = self.0.write().unwrap();
		vec.push(matcher);
		vec.sort_by_key(|b| std::cmp::Reverse(b.rank()));
	}

	pub fn remove(&self, name: &str) {
		let mut vec = self.0.write().unwrap();
		vec.retain(|matcher| matcher.name() != name);
	}

	pub fn get_all(&self) -> Vec<Arc<dyn Matcher>> {
		self.0.read().unwrap().clone()
	}

	pub fn get_with_name(&self, name: &str) -> Option<Arc<dyn Matcher>> {
		self.0.read().unwrap().iter().find(|matcher| matcher.name() == name).cloned()
	}
}
