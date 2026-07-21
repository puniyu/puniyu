use crate::{AppContext, Error, ScopeId};
use smol_str::SmolStr;
use std::{any::Any, sync::Arc};

use super::ScopedContext;

#[derive(Clone)]
pub struct PluginContext {
	scoped: ScopedContext,
	plugin_name: SmolStr,
}

impl PluginContext {
	pub fn new(app: Arc<AppContext>, plugin_name: impl Into<SmolStr>) -> Self {
		let scope_id = app.depot.new_scope();
		Self { scoped: ScopedContext::new(app, scope_id), plugin_name: plugin_name.into() }
	}

	pub fn scoped(&self) -> &ScopedContext {
		&self.scoped
	}

	pub fn scope_id(&self) -> ScopeId {
		self.scoped.scope_id()
	}

	pub fn plugin_name(&self) -> &str {
		self.plugin_name.as_str()
	}

	pub fn provide<V: Any + Send + Sync>(&self, value: V) -> Result<(), Error> {
		self.scoped.insert(value)
	}

	pub fn get<V: Any + Send + Sync + Clone>(&self) -> Option<V> {
		self.scoped.get()
	}

	pub fn require<V: Any + Send + Sync + Clone>(&self) -> Result<V, Error> {
		self.get().ok_or_else(|| Error::Missing {
			requester: self.plugin_name.clone(),
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
