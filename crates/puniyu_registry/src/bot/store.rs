use crate::{Error, Result};
use puniyu_types::bot::Bot;
use std::{
	collections::HashMap,
	sync::{
		Arc, RwLock,
		atomic::{AtomicU64, Ordering},
	},
};
static BOT_INDEX: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Default)]
pub(crate) struct BotStore(Arc<RwLock<HashMap<u64, Bot>>>);

impl BotStore {
	pub fn new() -> Self {
		Self::default()
	}
	pub fn insert(&self, bot: Bot) -> Result<u64> {
		let index = BOT_INDEX.fetch_add(1, Ordering::SeqCst);
		let mut map = self.0.write().expect("Failed to acquire lock");
		if map.values().any(|v| v == &bot) {
			return Err(Error::Exists("Bot".to_string()));
		}
		map.insert(index, bot);
		Ok(index)
	}

	pub(crate) fn raw(&self) -> Arc<RwLock<HashMap<u64, Bot>>> {
		self.0.clone()
	}
	

	pub fn all(&self) -> Vec<Bot> {
		let map = self.0.read().expect("Failed to acquire lock");
		map.values().cloned().collect()
	}
}
