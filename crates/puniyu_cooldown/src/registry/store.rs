use crate::CooldownScope;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Instant;

#[derive(Default)]
pub(super) struct CooldownStore(Arc<RwLock<HashMap<CooldownScope, Instant>>>);

impl CooldownStore {
	pub(super) fn new() -> Self {
		Self::default()
	}

	pub(super) fn raw(&self) -> &Arc<RwLock<HashMap<CooldownScope, Instant>>> {
		&self.0
	}
}
