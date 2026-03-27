mod store;
use crate::Bot;
use crate::types::BotId;
use puniyu_error::registry::Error;
use std::sync::LazyLock;
use store::BotStore;

static STORE: LazyLock<BotStore> = LazyLock::new(BotStore::new);

/// 全局机器人注册表。
///
/// # 示例
///
/// ```rust
/// use puniyu_account::AccountInfo;
/// use puniyu_adapter_api::AdapterApi;
/// use puniyu_adapter_types::{AdapterInfo, AdapterPlatform, AdapterProtocol};
/// use puniyu_bot::{Bot, BotRegistry};
///
/// let mut adapter = AdapterInfo::default();
/// adapter.name = "console".to_string();
/// adapter.platform = AdapterPlatform::QQ;
/// adapter.protocol = AdapterProtocol::Console;
///
/// let account = AccountInfo {
///     uin: "123456789".to_string(),
///     name: "Puniyu".to_string(),
///     avatar: Default::default(),
/// };
///
/// let bot = Bot::new(adapter, AdapterApi::default(), account);
/// let index = BotRegistry::register(bot.clone()).unwrap();
///
/// assert_eq!(BotRegistry::get(index), Some(bot));
/// BotRegistry::unregister(index).unwrap();
/// ```
pub struct BotRegistry;

impl<'b> BotRegistry {
	/// 将机器人注册到全局注册表，并返回分配的索引。
	pub fn register(bot: Bot) -> Result<u64, Error> {
		STORE.insert(bot)
	}

	/// 按索引或 UIN 从注册表移除机器人。
	pub fn unregister<B>(bot_id: B) -> Result<(), Error>
	where
		B: Into<BotId<'b>>,
	{
		let bot_id = bot_id.into();
		match bot_id {
			BotId::Index(id) => Self::unregister_with_index(id),
			BotId::SelfId(id) => Self::unregister_with_bot_id(id),
		}
	}

	/// 按注册表索引移除机器人。
	pub fn unregister_with_index(index: u64) -> Result<(), Error> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		if map.remove(&index).is_none() {
			return Err(Error::NotFound("Bot".to_string()));
		}
		Ok(())
	}

	/// 按机器人 UIN 移除所有匹配的机器人。
	pub fn unregister_with_bot_id(bot_id: &str) -> Result<(), Error> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		let indices = map
			.iter()
			.filter_map(|(k, v)| if v.account().uin == bot_id { Some(*k) } else { None })
			.collect::<Vec<u64>>();
		if indices.is_empty() {
			return Err(Error::NotFound("Bot".to_string()));
		}
		for idx in indices {
			if map.remove(&idx).is_none() {
				return Err(Error::NotFound("Bot".to_string()));
			}
		}

		Ok(())
	}

	/// 按索引或 UIN 查询机器人。
	pub fn get<T>(bot_id: T) -> Option<Bot>
	where
		T: Into<BotId<'b>>,
	{
		let bot_id = bot_id.into();
		match bot_id {
			BotId::Index(index) => Self::get_with_index(index),
			BotId::SelfId(self_id) => Self::get_with_bot_id(self_id),
		}
	}

	/// 按注册表索引查询机器人。
	pub fn get_with_index(index: u64) -> Option<Bot> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.get(&index).cloned()
	}

	/// 按机器人 UIN 查询第一个匹配的机器人。
	pub fn get_with_bot_id(self_id: &str) -> Option<Bot> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.values().find(|bot| bot.account().uin == self_id).cloned()
	}

	/// 返回所有已注册的机器人副本。
	pub fn all() -> Vec<Bot> {
		STORE.all()
	}
}
