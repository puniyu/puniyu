use crate::Command;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Default)]
pub(crate) struct CommandStore(Arc<RwLock<HashMap<u64, Arc<dyn Command>>>>);

impl CommandStore {
	pub fn new() -> Self {
		Self::default()
	}

	pub(crate) fn raw(&self) -> &Arc<RwLock<HashMap<u64, Arc<dyn Command>>>> {
		&self.0
	}
}
