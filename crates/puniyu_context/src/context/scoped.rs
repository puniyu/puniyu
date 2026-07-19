use crate::{Error, ScopeId};
use std::{any::Any, sync::Arc};

use super::AppContext;

#[derive(Clone)]
pub struct ScopedContext {
	inner: Arc<AppContext>,
	scope_id: ScopeId,
}

impl ScopedContext {
	pub fn new(app: Arc<AppContext>, scope_id: ScopeId) -> Self {
		Self { inner: app, scope_id }
	}

	pub fn scope_id(&self) -> ScopeId {
		self.scope_id
	}

	pub fn insert<V: Any + Send + Sync>(&self, value: V) -> Result<(), Error> {
		self.inner.depot.insert(self.scope_id, value)
	}

	pub fn get<V: Any + Send + Sync + Clone>(&self) -> Option<V> {
		self.inner.get()
	}

	pub fn contains<V: Any + Send + Sync>(&self) -> bool {
		self.inner.contains::<V>()
	}

	pub fn remove<V: Any + Send + Sync>(&self) -> Option<V> {
		self.inner.depot.remove(self.scope_id)
	}
}
