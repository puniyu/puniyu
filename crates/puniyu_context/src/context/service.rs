use crate::{Context, Error};
use std::{any::Any, sync::Arc};

pub struct ServiceContext {
	inner: Arc<Context>,
}

impl ServiceContext {
	pub fn new(context: Arc<Context>) -> Self {
		Self { inner: context }
	}

	pub fn insert<V: Any + Send + Sync>(&self, value: V) -> Result<(), Error> {
		self.inner.depot.insert(value)
	}

	pub fn get<V: Any + Send + Sync + Clone>(&self) -> Option<V> {
		self.inner.depot.get()
	}


	pub fn contains<V: Any + Send + Sync>(&self) -> bool {
		self.inner.depot.contains::<V>()
	}
}
