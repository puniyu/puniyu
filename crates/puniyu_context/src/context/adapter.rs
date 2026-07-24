use crate::{AppContext, Error};
use smol_str::SmolStr;
use std::{any::Any, sync::Arc};

pub struct AdapterContext {
	inner: Arc<AppContext>,
	adapter_name: SmolStr,
}

impl AdapterContext {
	pub fn new(app: Arc<AppContext>, adapter_name: impl Into<SmolStr>) -> Self {
		Self { inner: app, adapter_name: adapter_name.into() }
	}

	pub fn adapter_name(&self) -> &str {
		self.adapter_name.as_str()
	}

	pub fn get<V: Any + Send + Sync + Clone>(&self) -> Option<V> {
		self.inner.get()
	}

	pub fn require<V: Any + Send + Sync + Clone>(&self) -> Result<V, Error> {
		self.get().ok_or_else(|| Error::Missing {
			requester: self.adapter_name.clone(),
			capability: std::any::type_name::<V>(),
		})
	}

	pub fn contains<V: Any + Send + Sync>(&self) -> bool {
		self.inner.contains::<V>()
	}
}
