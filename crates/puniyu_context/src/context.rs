mod scoped;
pub use scoped::ScopedContext;
mod plugin;
pub use plugin::PluginContext;
mod adapter;
pub use adapter::AdapterContext;

use crate::Error;
use crate::depot::Depot;
use crate::scope::ScopeId;
use std::any::Any;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

pub struct AppContext {
	pub(crate) depot: Arc<Depot>,
	scope_id: ScopeId,
	next_scope_id: AtomicU64,
}

impl Default for AppContext {
	fn default() -> Self {
		Self {
			depot: Arc::new(Depot::new()),
			scope_id: ScopeId::root(),
			next_scope_id: AtomicU64::new(1),
		}
	}
}

impl AppContext {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn scope_id(&self) -> ScopeId {
		self.scope_id
	}

	pub fn new_scope(&self) -> ScopeId {
		ScopeId::new(self.next_scope_id.fetch_add(1, Ordering::Relaxed))
	}

	pub fn insert<V: Any + Send + Sync>(&self, value: V) -> Result<(), Error> {
		self.depot.insert(self.scope_id, value)
	}

	pub fn get<V: Any + Send + Sync + Clone>(&self) -> Option<V> {
		self.depot.get()
	}

	pub fn contains<V: Any + Send + Sync>(&self) -> bool {
		self.depot.contains::<V>()
	}

	pub fn remove<V: Any + Send + Sync>(&self) -> Option<V> {
		self.depot.remove(self.scope_id)
	}

	pub fn remove_scope(&self, scope_id: ScopeId) {
		self.depot.remove_scope(scope_id);
	}
}
