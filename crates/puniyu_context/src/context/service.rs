use crate::{AppContext, Error};
use smol_str::SmolStr;
use std::{any::Any, sync::Arc};

pub struct ServiceContext {
	inner: Arc<AppContext>,
	service_name: SmolStr,
}

impl ServiceContext {
	pub fn new(app: Arc<AppContext>, service_name: impl Into<SmolStr>) -> Self {
		Self { inner: app, service_name: service_name.into() }
	}

	pub fn service_name(&self) -> &str {
		self.service_name.as_str()
	}

	pub fn provide<V: Any + Send + Sync>(&self, value: V) -> Result<(), Error> {
		self.inner.depot.insert(value)
	}

	pub fn get<V: Any + Send + Sync + Clone>(&self) -> Option<V> {
		self.inner.get()
	}

	pub fn require<V: Any + Send + Sync + Clone>(&self) -> Result<V, Error> {
		self.get().ok_or_else(|| Error::Missing {
			requester: self.service_name.clone(),
			capability: std::any::type_name::<V>(),
		})
	}

	pub fn contains<V: Any + Send + Sync>(&self) -> bool {
		self.inner.contains::<V>()
	}

	pub fn remove<V: Any + Send + Sync>(&self) -> Option<V> {
		self.inner.depot.remove()
	}
}
