mod store;
mod matchers;
pub use matchers::command::CommandMatcher;
use puniyu_types::matcher::Matcher;
use store::MatcherStore;
use std::sync::{Arc, LazyLock};

static MATCHER_STORE: LazyLock<MatcherStore> = LazyLock::new(MatcherStore::default);

pub struct MatcherRegistry;

impl MatcherRegistry {
	/// 注册事件匹配器
	pub fn register(matcher: Arc<dyn Matcher>) {
		MATCHER_STORE.insert(matcher);
	}

	/// 卸载事件匹配器
	pub fn unregister(&mut self, name: &str) {
		MATCHER_STORE.remove(name);
	}

	pub fn get_all(&self) -> Vec<Arc<dyn Matcher>> {
		MATCHER_STORE.get_all()
	}

	pub fn get_with_name(&self, name: &str) -> Option<Arc<dyn Matcher>> {
		MATCHER_STORE.get_with_name(name)
	}
}
