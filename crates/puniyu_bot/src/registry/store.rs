use crate::Bot;
use std::{
	collections::HashMap,
	sync::{Arc, RwLock},
};

#[derive(Default)]
pub(crate) struct BotStore(Arc<RwLock<HashMap<u64, Arc<Bot>>>>);

impl BotStore {
	pub fn new() -> Self {
		Self::default()
	}

	pub(crate) fn raw(&self) -> &Arc<RwLock<HashMap<u64, Arc<Bot>>>> {
		&self.0
	}
}
