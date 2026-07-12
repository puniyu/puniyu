use crate::Handler;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Default)]
pub(crate) struct HandlerStore(Arc<RwLock<HashMap<u64, Arc<dyn Handler>>>>);


impl HandlerStore {
	pub fn new() -> Self {
		Self::default()
	}

	pub(crate) fn raw(&self) -> &Arc<RwLock<HashMap<u64, Arc<dyn Handler>>>> {
		&self.0
	}
}
