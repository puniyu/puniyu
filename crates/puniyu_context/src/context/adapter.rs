use smol_str::SmolStr;
use std::{any::Any, sync::Arc};

use crate::{AppContext, Error, ScopeId, ScopedContext};

#[derive(Clone)]
pub struct AdapterContext {
	scoped: ScopedContext,
	adapter_name: SmolStr,
}

impl AdapterContext {
	pub fn new(app: Arc<AppContext>, scope_id: ScopeId, adapter_name: impl Into<SmolStr>) -> Self {
		Self { scoped: ScopedContext::new(app, scope_id), adapter_name: adapter_name.into() }
	}

	pub fn scoped(&self) -> &ScopedContext {
		&self.scoped
	}

	pub fn scope_id(&self) -> ScopeId {
		self.scoped.scope_id()
	}

	pub fn adapter_name(&self) -> &str {
		self.adapter_name.as_str()
	}

	pub fn provide<V: Any + Send + Sync>(&self, value: V) -> Result<(), Error> {
		self.scoped.insert(value)
	}

	pub fn get<V: Any + Send + Sync + Clone>(&self) -> Option<V> {
		self.scoped.get()
	}

	pub fn require<V: Any + Send + Sync + Clone>(&self) -> Result<V, Error> {
		self.get().ok_or_else(|| Error::Missing {
			requester: self.adapter_name.clone(),
			capability: std::any::type_name::<V>(),
		})
	}

	pub fn contains<V: Any + Send + Sync>(&self) -> bool {
		self.scoped.contains::<V>()
	}

	pub fn remove<V: Any + Send + Sync>(&self) -> Option<V> {
		self.scoped.remove()
	}
}
