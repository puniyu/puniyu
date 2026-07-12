mod store;

use crate::Bot;
use crate::error::Error;
use crate::types::BotId;
use std::sync::{Arc, LazyLock};
use std::sync::atomic::{AtomicU64, Ordering};
use store::BotStore;

static BOT_INDEX: AtomicU64 = AtomicU64::new(0);
static STORE: LazyLock<BotStore> = LazyLock::new(BotStore::new);

/// 全局机器人注册表。
pub struct BotRegistry;

impl BotRegistry {
	/// 将机器人注册到全局注册表，返回分配的索引。
	pub fn register(bot: impl Into<Arc<Bot>>) -> Result<u64, Error> {
		let mut map = STORE.raw().write().expect("poisoned lock");
		let bot = bot.into();
		if map.values().any(|v| Arc::ptr_eq(v, &bot) || v.id == bot.id) {
			return Err(Error::Exists);
		}
		let index = BOT_INDEX.fetch_add(1, Ordering::Relaxed);
		map.insert(index, bot);
		Ok(index)
	}

	/// 按索引或 UIN 从注册表卸载并销毁机器人。
	pub fn unregister<'b>(bot_id: impl Into<BotId<'b>>) -> Result<(), Error> {
		match bot_id.into() {
			BotId::Index(id) => Self::unregister_with_index(id),
			BotId::SelfId(id) => Self::unregister_with_bot_id(id.as_ref()),
		}
	}

	/// 按注册表索引卸载并销毁机器人。
	pub fn unregister_with_index(index: u64) -> Result<(), Error> {
		let mut map = STORE.raw().write().expect("poisoned lock");
		map.remove(&index).ok_or(Error::NotFound)?;
		Ok(())
	}

	/// 按机器人 UIN 卸载并销毁所有匹配的机器人。
	pub fn unregister_with_bot_id(bot_id: &str) -> Result<(), Error> {
		let mut map = STORE.raw().write().expect("poisoned lock");
		let keys: Vec<u64> = map
			.iter()
			.filter_map(|(k, v)| if v.id == bot_id { Some(*k) } else { None })
			.collect();
		if keys.is_empty() {
			return Err(Error::NotFound);
		}
		for key in keys {
			map.remove(&key);
		}
		Ok(())
	}

	/// 按索引或 UIN 查询机器人。
	pub fn get<'b>(bot_id: impl Into<BotId<'b>>) -> Option<Arc<Bot>> {
		match bot_id.into() {
			BotId::Index(index) => Self::get_with_index(index),
			BotId::SelfId(self_id) => Self::get_with_bot_id(self_id.as_ref()),
		}
	}

	/// 按注册表索引查询机器人。
	pub fn get_with_index(index: u64) -> Option<Arc<Bot>> {
		let map = STORE.raw().read().expect("poisoned lock");
		map.get(&index).cloned()
	}

	/// 按机器人 UIN 查询第一个匹配的机器人。
	pub fn get_with_bot_id(self_id: &str) -> Option<Arc<Bot>> {
		let map = STORE.raw().read().expect("poisoned lock");
		map.values().find(|v| v.id == self_id).cloned()
	}

	/// 返回所有已注册的机器人。
	pub fn all() -> Vec<Arc<Bot>> {
		let map = STORE.raw().read().expect("poisoned lock");
		map.values().cloned().collect()
	}
}
