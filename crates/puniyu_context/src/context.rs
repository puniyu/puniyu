mod plugin;
pub use plugin::PluginContext;
mod adapter;
pub use adapter::AdapterContext;
mod service;
pub use service::ServiceContext;

use crate::Depot;
use std::sync::Arc;

#[derive(Default, Clone)]
pub struct Context {
	pub(crate) depot: Arc<Depot>,
}


impl Context {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn service(&self) -> ServiceContext {
		ServiceContext::new(Arc::new(self.clone()))
	}

	
}
