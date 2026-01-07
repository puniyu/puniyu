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
pub(crate) struct BotStore(pub(crate) Arc<RwLock<HashMap<u64, Bot>>>);

impl BotStore {

	pub fn get_with_index(&self, index: u64) -> Option<Bot> {
		self.0.read().unwrap().get(&index).cloned()
	}

	pub fn all(&self) -> Vec<Bot> {
		self.0.read().unwrap().values().cloned().collect()
	}

	pub fn get_with_self_id(&self, self_id: &str) -> Option<Bot> {
		self.0.read().unwrap().values().find(|bot| bot.account.uin == self_id).cloned()
	}

	pub fn insert(&self, bot: Bot) -> u64 {
		let index = BOT_INDEX.fetch_add(1, Ordering::Relaxed);
		self.0.write().unwrap().insert(index, bot);
		index
	}

	pub fn remove_with_index(&self, index: u64) -> Option<Bot> {
		self.0.write().unwrap().remove(&index)
	}

	pub fn remove_with_self_id(&self, self_id: &str) -> Option<Bot> {
		let mut bots = self.0.write().unwrap();
		let index = bots.iter().find(|(_, bot)| bot.account.uin == self_id).map(|(i, _)| *i);
		index.and_then(|i| bots.remove(&i))
	}
}
