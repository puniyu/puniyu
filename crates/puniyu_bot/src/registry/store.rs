use crate::Bot;
use puniyu_error::registry::Error;
use std::{
	collections::HashMap,
	sync::{
		Arc, RwLock,
		atomic::{AtomicU64, Ordering},
	},
};

static BOT_INDEX: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Default)]
pub(crate) struct BotStore(Arc<RwLock<HashMap<u64, Arc<dyn Bot>>>>);

impl BotStore {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn insert(&self, bot: Arc<dyn Bot>) -> Result<u64, Error> {
		let mut map = self.0.write().expect("Failed to acquire lock");
		if map.values().any(|v| v.as_ref() == bot.as_ref()) {
			return Err(Error::Exists("Bot".to_string()));
		}
		let index = BOT_INDEX.fetch_add(1, Ordering::SeqCst);
		map.insert(index, bot);
		Ok(index)
	}

	pub fn remove(&self, bot_index: u64) -> Option<Arc<dyn Bot>> {
		self.0.write().expect("Failed to acquire lock").remove(&bot_index)
	}

	pub fn raw(&self) -> Arc<RwLock<HashMap<u64, Arc<dyn Bot>>>> {
		self.0.clone()
	}

	pub fn all(&self) -> Vec<Arc<dyn Bot>> {
		self.0.read().expect("Failed to acquire lock").values().cloned().collect()
	}
}
