mod store;
mod types;

use store::BotStore;

use crate::{Error, Result};
use puniyu_logger::{info, owo_colors::OwoColorize, warn};
pub use puniyu_types::bot::Bot;
use std::sync::LazyLock;
use puniyu_types::adapter::Plugin;
use types::BotId;

static STORE: LazyLock<BotStore> = LazyLock::new(BotStore::new);

pub struct BotRegistry;

impl<B: Plugin> BotRegistry {
	pub fn register(bot: Bot) -> Result<u64> {
		let self_id = bot.account().uin.clone();
		let adapter_name = bot.adapter().name.clone();
		let index = STORE.insert(bot);
		info!(
			"[{}] [{}] 注册成功",
			format!("Bot: {}", self_id).fg_rgb::<221, 160, 221>(),
			format!("adapter:{}", adapter_name).fg_rgb::<255, 182, 193>()
		);
		index
	}
	pub fn unregister<B>(bot_id: B) -> Result<()>
	where
		B: Into<BotId>,
	{
		let bot_id = bot_id.into();
		match bot_id {
			BotId::Index(id) => Self::unregister_with_index(id),
			BotId::SelfId(id) => Self::unregister_with_bot_id(id),
		}
	}

	pub fn unregister_with_index(index: u64) -> Result<()> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		if let Some(bot) = map.remove(&index) {
			let self_id = bot.account().uin;
			let adapter_info = bot.adapter();
			let adapter_name = adapter_info.name;
			let adapter_version = adapter_info.version;

			info!(
				"[Bot: index-{}][adapter:{} v{}] 卸载成功",
				self_id, adapter_name, adapter_version
			);
			Ok(())
		} else {
			Err(Error::NotFound("Bot".to_string()))
		}
	}

	pub fn unregister_with_bot_id(bot_id: &str) -> Result<()> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		let indices = map
			.iter()
			.filter_map(|(k, v)| if v.account().uin == bot_id { Some(*k) } else { None })
			.collect::<Vec<u64>>();
		if indices.is_empty() {
			warn!("[Bot: {}] 卸载失败未找到指定的Bot", bot_id);
			return Err(Error::NotFound("Bot".to_string()));
		}
		indices.into_iter().for_each(|idx| {
			if let Some(bot) = map.remove(&idx) {
				let self_id = bot.account().uin;
				let adapter_info = bot.adapter();
				let adapter_name = adapter_info.name;
				let adapter_version = adapter_info.version;
				info!(
					"[Bot: {}][adapter:{} v{}] 卸载成功",
					self_id, adapter_name, adapter_version
				);
			}
		});

		Ok(())
	}

	pub fn get<T>(bot_id: T) -> Vec<Bot>
	where
		T: Into<BotId>
	{
		let bot_id = bot_id.into();
		match bot_id {
			BotId::Index(index) => Self::get_with_index(index).into_iter().collect(),
			BotId::SelfId(self_id) => Self::get_with_bot_id(self_id),
		}
	}
	pub fn get_with_index(index: u64) -> Option<Bot> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.get(&index).cloned()
	}

	pub fn get_with_bot_id(self_id: &str) -> Vec<Bot> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.values().filter(|bot| bot.account().uin == self_id).cloned().collect()
	}

	pub fn all() -> Vec<Bot> {
		STORE.all()
	}
}

/// 注册Bot实例
///
/// # 参数
///
/// * `adapter` - 适配器信息
/// * `account` - 账号信息
/// * `api` - 适配器API
///
/// ## 示例
/// ```rust, ignore
/// use puniyu_registry::register_bot;
/// use puniyu_registry::bot::BotRegistry;
/// use puniyu_types::bot::Bot;
/// use puniyu_types::adapter::{AdapterInfo, AdapterApi};
/// use puniyu_types::account::AccountInfo;
///
/// register_bot!(adapter: adapter, account: account, api: api);
/// ```
#[cfg(feature = "bot")]
#[macro_export]
macro_rules! register_bot {
	($adapter:expr, $account:expr) => {
		let bot = $crate::bot::Bot::new($adapter, $account)
		$crate::bot::BotRegistry::register(bot)
	};
	(adapter: $adapter:expr, account: $account:expr) => {
		let bot = $crate::bot::Bot::new($adapter, $account)
		$crate::bot::BotRegistry::register(bot)
	};
}

/// 卸载Bot实例
///
/// # 参数
///
/// * `id` - Bot的ID，可以是索引或self_id
///
/// ## 示例
/// ```rust
/// use puniyu_registry::unregister_bot;
/// use puniyu_registry::bot::BotRegistry;
///
/// unregister_bot!("123");
/// unregister_bot!(id: "123");
/// unregister_bot!(index: 0);
/// ```
#[cfg(feature = "bot")]
#[macro_export]
macro_rules! unregister_bot {
	($id:expr) => {
		match $id {
			id if id.is::<u64>() => BotRegistry::unregister_with_index(id),
			_ => BotRegistry::unregister_with_bot_id($id.to_string()),
		}
	};
	(index: $index:expr) => {
		BotRegistry::unregister_with_index($index)
	};
	(id: $id:expr) => {
		BotRegistry::unregister_with_bot_id($id)
	};
}

/// 获取Bot实例
///
/// # 参数
///
/// * `id` - Bot的ID
///
/// # 返回值
///
/// * `Option<Bot>` - 如果找到Bot，则返回Bot实例，否则返回None
///
/// ## 示例
/// ```rust
/// use puniyu_registry::bot::get_bot;
/// assert!(get_bot("123").is_none());
/// ```
///
///
pub fn get_bot(id: impl Into<BotId>) -> Option<Bot> {
	let bot_id: BotId = id.into();
	match bot_id {
		BotId::Index(index) => BotRegistry::get_with_index(index),
		BotId::SelfId(self_id) => {
			let raw = STORE.raw();
			let map = raw.read().expect("Failed to acquire lock");
			map.values().find(|v| v.account().uin == self_id).cloned()
		}
	}
}

/// 获取Bot数量
///
/// # 返回值
///
/// * `u64` - Bot数量
/// ## 示例
/// ```rust
/// use puniyu_registry::bot::get_bot_count;
/// assert_eq!(get_bot_count(), 0);
/// ```
///
pub fn get_bot_count() -> u64 {
	BotRegistry::all().len() as u64
}
