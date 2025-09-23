use crate::bot::Bot;
use crate::logger::info;
use puniyu_utils::adapter::{AccountInfo, AdapterInfo};
use std::sync::{
	Arc, LazyLock, RwLock,
	atomic::{AtomicU64, Ordering},
};

static BOT_REGISTRY: LazyLock<BotRegistry> =
	LazyLock::new(|| BotRegistry { bots: Arc::new(RwLock::new(Vec::new())) });
static BOT_INDEX: AtomicU64 = AtomicU64::new(0);

pub struct BotRegistry {
	pub bots: Arc<RwLock<Vec<Bot>>>,
}

impl BotRegistry {
	pub fn get_all() -> Vec<Bot> {
		let bots = BOT_REGISTRY.bots.read().unwrap();
		bots.clone()
	}

	pub fn get(index: u64) -> Option<Bot> {
		let bots = BOT_REGISTRY.bots.read().unwrap();
		bots.iter().find(|bot| bot.index == index).cloned()
	}

	pub fn get_with_self_id(self_id: &str) -> Option<Bot> {
		let bots = BOT_REGISTRY.bots.read().unwrap();
		bots.iter().find(|bot| bot.account.self_id == self_id).cloned()
	}

	pub fn register(adapter: AdapterInfo, account: AccountInfo) -> u64 {
		let index = BOT_INDEX.fetch_add(1, Ordering::Relaxed);
		let bot = Bot { index, adapter, account };
		BOT_REGISTRY.bots.write().unwrap().push(bot);
		info!("[Bot: {}] 注册成功", index);
		index
	}

	pub fn unregister(index: u64) -> bool {
		let mut bots = BOT_REGISTRY.bots.write().unwrap();
		let is_unregistered =
			bots.iter().position(|bot| bot.index == index).map(|pos| bots.remove(pos)).is_some();
		if is_unregistered {
			info!("[Bot: {}] 卸载成功", index);
		} else {
			info!("[Bot: {}] 卸载失败", index);
		}
		is_unregistered
	}

	pub fn unregister_with_id(id: &str) -> bool {
		let mut bots = BOT_REGISTRY.bots.write().unwrap();
		let is_unregistered = bots
			.iter()
			.position(|bot| bot.account.self_id == id)
			.map(|pos| bots.remove(pos))
			.is_some();
		if is_unregistered {
			info!("[Bot: {}] 卸载成功", id);
		} else {
			info!("[Bot: {}] 卸载失败", id);
		}
		is_unregistered
	}
}
