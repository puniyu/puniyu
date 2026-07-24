use crate::{AppContext, Error};
use smol_str::SmolStr;
use std::{any::Any, sync::Arc};

pub struct PluginContext {
	inner: Arc<AppContext>,
	plugin_name: SmolStr,
}

impl PluginContext {
	pub fn new(app: Arc<AppContext>, plugin_name: impl Into<SmolStr>) -> Self {
		Self { inner: app, plugin_name: plugin_name.into() }
	}

	pub fn plugin_name(&self) -> &str {
		self.plugin_name.as_str()
	}

	pub fn get<V: Any + Send + Sync + Clone>(&self) -> Option<V> {
		self.inner.get()
	}

	pub fn require<V: Any + Send + Sync + Clone>(&self) -> Result<V, Error> {
		self.get().ok_or_else(|| Error::Missing {
			requester: self.plugin_name.clone(),
			capability: std::any::type_name::<V>(),
		})
	}

	pub fn contains<V: Any + Send + Sync>(&self) -> bool {
		self.inner.contains::<V>()
	}
}
