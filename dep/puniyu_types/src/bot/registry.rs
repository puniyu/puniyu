use super::Bot;
use puniyu_logger::info;
use std::collections::HashMap;
use std::sync::{
	Arc, LazyLock, RwLock,
	atomic::{AtomicU64, Ordering},
};

static BOT_REGISTRY: LazyLock<BotRegistry> = LazyLock::new(BotRegistry::default);
static BOT_INDEX: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Default)]
pub struct BotRegistry(Arc<RwLock<HashMap<u64, Bot>>>);

impl BotRegistry {
	pub fn get_all() -> Vec<Bot> {
		let bots = BOT_REGISTRY.0.read().unwrap();
		bots.values().cloned().collect()
	}

	pub fn get_with_index(index: u64) -> Option<Bot> {
		let bots = BOT_REGISTRY.0.read().unwrap();
		bots.get(&index).cloned()
	}

	pub fn get_with_self_id(self_id: &str) -> Option<Bot> {
		let bots = BOT_REGISTRY.0.read().unwrap();
		bots.values().find(|bot| bot.account.self_id == self_id).cloned()
	}

	pub fn register(bot: Bot) {
		let index = BOT_INDEX.fetch_add(1, Ordering::Relaxed);
		let self_id = bot.account.self_id.clone();
		BOT_REGISTRY.0.write().unwrap().insert(index, bot);
		info!("[Bot: {}] 注册成功", self_id);
	}

	pub fn unregister_with_index(index: u64) {
		let mut bots = BOT_REGISTRY.0.write().unwrap();
		let removed_bot = bots.remove(&index);
		if let Some(bot) = removed_bot {
			info!("[Bot: {}] 卸载成功", bot.account.self_id);
		}
	}

	pub fn unregister_with_id(id: &str) {
		let mut bots = BOT_REGISTRY.0.write().unwrap();
		let index_to_remove =
			bots.iter().find(|(_, bot)| bot.account.self_id == id).map(|(index, _)| *index);

		if let Some(index) = index_to_remove {
			let removed_bot = bots.remove(&index);
			if let Some(bot) = removed_bot {
				info!("[Bot: {}] 卸载成功", bot.account.self_id);
			}
		}

		info!("[Bot: {}] 卸载失败", id);
	}
}

/// 注册Bot实例
///
/// # 参数
///
/// * `adapter` - 适配器信息
/// * `account` - 账号信息
#[cfg(feature = "bot")]
#[macro_export]
macro_rules! register_bot {
	($adapter:expr, $account:expr, $api:expr) => {
		let bot = Bot { adapter: $adapter, api: $api, account: $account };
		BotRegistry::register(bot)
	};
	(adapter: $adapter:expr, account: $account:expr, api: $api:expr) => {
		let bot = Bot { adapter: $adapter, api: $api, account: $account };
		BotRegistry::register(bot)
	};
}

/// 卸载Bot实例
///
/// # 参数
///
/// * `id` - Bot的ID，可以是索引或self_id
///
#[cfg(feature = "bot")]
#[macro_export]
macro_rules! unregister_bot {
	(index: $index:expr) => {
		BotRegistry::unregister_with_index($index)
	};
	(id: $id:expr) => {
		BotRegistry::unregister_with_id($id)
	};
}
