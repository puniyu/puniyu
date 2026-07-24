mod plugin;
pub use plugin::PluginContext;
mod adapter;
pub use adapter::AdapterContext;
mod service;
pub use service::ServiceContext;

use crate::Error;
use crate::depot::Depot;
use std::any::Any;
use std::sync::Arc;

pub struct AppContext {
	pub(crate) depot: Arc<Depot>,
}

impl Default for AppContext {
	fn default() -> Self {
		Self { depot: Arc::new(Depot::new()) }
	}
}

impl AppContext {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn insert<V: Any + Send + Sync>(&self, value: V) -> Result<(), Error> {
		self.depot.insert(value)
	}

	pub fn get<V: Any + Send + Sync + Clone>(&self) -> Option<V> {
		self.depot.get()
	}

	pub fn contains<V: Any + Send + Sync>(&self) -> bool {
		self.depot.contains::<V>()
	}

	pub fn remove<V: Any + Send + Sync>(&self) -> Option<V> {
		self.depot.remove()
	}
}
