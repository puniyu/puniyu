use crate::ServerInfo;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Default)]
pub(crate) struct ServerStore(Arc<RwLock<HashMap<u64, ServerInfo>>>);

impl ServerStore {
	pub fn new() -> Self {
		Self::default()
	}

	pub(crate) fn raw(&self) -> &Arc<RwLock<HashMap<u64, ServerInfo>>> {
		&self.0
	}
}
