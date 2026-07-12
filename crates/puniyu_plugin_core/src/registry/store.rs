use crate::Plugin;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Clone, Default)]
pub(crate) struct PluginStore(Arc<RwLock<HashMap<u64, Arc<dyn Plugin>>>>);

impl PluginStore {
	pub fn new() -> Self {
		Self::default()
	}

	pub(crate) fn raw(&self) -> &Arc<RwLock<HashMap<u64, Arc<dyn Plugin>>>> {
		&self.0
	}
}
