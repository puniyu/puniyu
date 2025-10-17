use crate::Bot;
use puniyu_adapter_builder::{AccountInfo, AdapterInfo};
use puniyu_logger::info;
use std::sync::{
	Arc, LazyLock, RwLock,
	atomic::{AtomicU64, Ordering},
};

static BOT_REGISTRY: LazyLock<BotRegistry> = LazyLock::new(BotRegistry::default);
static BOT_INDEX: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Default)]
pub struct BotRegistry(Arc<RwLock<Vec<Bot>>>);

impl BotRegistry {
	pub fn get_all() -> Vec<Bot> {
		let bots = BOT_REGISTRY.0.read().unwrap();
		bots.clone()
	}

	pub fn get_with_index(index: u64) -> Option<Bot> {
		let bots = BOT_REGISTRY.0.read().unwrap();
		bots.iter().find(|bot| bot.index == index).cloned()
	}

	pub fn get_with_self_id(self_id: &str) -> Option<Bot> {
		let bots = BOT_REGISTRY.0.read().unwrap();
		bots.iter().find(|bot| bot.account.self_id == self_id).cloned()
	}

	pub fn register(adapter: AdapterInfo, account: AccountInfo) {
		let index = BOT_INDEX.fetch_add(1, Ordering::Relaxed);
		let self_id = account.self_id.clone();
		let bot = Bot { index, adapter, account };
		BOT_REGISTRY.0.write().unwrap().push(bot);
		info!("[Bot: {}] 注册成功", self_id);
	}

	pub fn unregister(index: u64) {
		let mut bots = BOT_REGISTRY.0.write().unwrap();
		let removed_bot =
			bots.iter().position(|bot| bot.index == index).map(|pos| bots.remove(pos));
		if let Some(bot) = removed_bot {
			info!("[Bot: {}] 卸载成功", bot.account.self_id);
		}
	}

	pub fn unregister_with_id(id: &str) -> bool {
		let mut bots = BOT_REGISTRY.0.write().unwrap();
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

/// 注册Bot实例
///
/// # 参数
///
/// * `adapter` - 适配器信息
/// * `account` - 账号信息
///
/// ## 示例
/// ```rust
/// register_bot!(adapter_info, account_info);
/// register_bot!(adapter: adapter_info, account: account_info);
/// ```
#[macro_export]
macro_rules! register_bot {
	($adapter:expr, $account:expr) => {
		BotRegistry::register($adapter, $account)
	};
	(adapter: $adapter:expr, account: $account:expr) => {
		BotRegistry::register($adapter, $account)
	};
}

/// 卸载Bot实例
///
/// # 参数
///
/// * `id` - Bot的ID，可以是索引或self_id
///
/// ## 实例
/// ```rust
/// unregister_bot!("self_id")
/// unregister_bot!(index: 1)
/// nregister_bot!(id: "self_id")
/// ```
#[macro_export]
macro_rules! unregister_bot {
	($id:expr) => {
		BotRegistry::unregister_with_id($id)
	};
	(index: $index:expr) => {{ BotRegistry::unregister($index) }};
	(id: $id:expr) => {
		BotRegistry::unregister_with_id($id)
	};
}
