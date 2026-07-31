use std::sync::Arc;

use puniyu_action::Action;
use puniyu_registry::Registry;
use puniyu_session::EventSession;

pub struct ActionConetxt {
	inner: Registry<Arc<dyn Action>>,
}

impl ActionConetxt {
	pub(crate) fn new() -> Self {
		Self { inner: Registry::new() }
	}

	pub fn insert<A: Action + 'static>(&self, action: A) {
		self.inner.insert(Arc::new(action));
	}

	pub fn get(&self, name: &str) -> Option<Arc<dyn Action>> {
		let mut result = None;
		self.inner.for_each(|_, a| {
			if a.name() == name {
				result = Some(a.clone());
			}
		});
		result
	}

	pub fn values(&self) -> Vec<Arc<dyn Action>> {
		self.inner.values()
	}

	pub async fn dispatch(&self, session: &EventSession) {
		let mut actions = self.values();
		actions.sort_by_key(|a| a.priority());

		for action in actions {
			if action.matcher().matches(session).await {
				let _ = action.handler().handle(session.clone()).await;
				if action.block() {
					break;
				}
			}
		}
	}
}
