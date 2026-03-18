mod store;
use crate::Bot;
use crate::types::BotId;
use puniyu_error::registry::Error;
use std::sync::{Arc, LazyLock};
use store::BotStore;

static STORE: LazyLock<BotStore> = LazyLock::new(BotStore::new);

/// 机器人注册表
///
/// 提供全局的机器人实例管理功能，支持注册、注销和查询机器人。
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_bot::{Bot, BotRegistry};
///
/// // 注册机器人
/// let index = BotRegistry::register(bot)?;
///
/// // 通过索引获取
/// let bot = BotRegistry::get_with_index(index);
///
/// // 通过 UIN 获取
/// let bots = BotRegistry::get_with_bot_id("123456");
///
/// // 注销机器人
/// BotRegistry::unregister(index)?;
/// ```
pub struct BotRegistry;

impl<'b> BotRegistry {
	/// 注册机器人到注册表
	///
	/// # 参数
	///
	/// - `bot` - 要注册的机器人实例
	///
	/// # 返回值
	///
	/// 返回分配的索引号，失败时返回错误
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let index = BotRegistry::register(bot)?;
	/// println!("机器人已注册，索引: {}", index);
	/// ```
	pub fn register(bot: Bot) -> Result<u64, Error> {
		STORE.insert(bot)
	}

	/// 从注册表注销机器人
	///
	/// 支持使用索引或 UIN 注销机器人。
	///
	/// # 参数
	///
	/// - `bot_id` - 机器人标识符（索引或 UIN）
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// // 使用索引注销
	/// BotRegistry::unregister(123u64)?;
	///
	/// // 使用 UIN 注销
	/// BotRegistry::unregister("123456")?;
	/// ```
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

	/// 使用索引注销机器人
	///
	/// # 参数
	///
	/// - `index` - 机器人的注册表索引
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// BotRegistry::unregister_with_index(123)?;
	/// ```
	pub fn unregister_with_index(index: u64) -> Result<(), Error> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		if map.remove(&index).is_none() {
			return Err(Error::NotFound("Bot".to_string()));
		}
		Ok(())
	}

	/// 使用 UIN 注销机器人
	///
	/// 会注销所有匹配该 UIN 的机器人实例。
	///
	/// # 参数
	///
	/// - `bot_id` - 机器人的 UIN
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// BotRegistry::unregister_with_bot_id("123456")?;
	/// ```
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

	/// 获取机器人实例
	///
	/// 支持使用索引或 UIN 查询。
	///
	/// # 参数
	///
	/// - `bot_id` - 机器人标识符（索引或 UIN）
	///
	/// # 返回值
	///
	/// 返回匹配的机器人实例列表
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// // 使用索引查询
	/// let bots = BotRegistry::get(123u64);
	///
	/// // 使用 UIN 查询
	/// let bots = BotRegistry::get("123456");
	/// ```
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

	/// 使用索引获取机器人
	///
	/// # 参数
	///
	/// - `index` - 机器人的注册表索引
	///
	/// # 返回值
	///
	/// 返回机器人实例，如果不存在则返回 [`None`]
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// if let Some(bot) = BotRegistry::get_with_index(123) {
	///     println!("找到机器人: {}", bot.account().uin);
	/// }
	/// ```
	pub fn get_with_index(index: u64) -> Option<Bot> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.get(&index).cloned()
	}

	/// 使用 UIN 获取机器人
	///
	/// 返回所有匹配该 UIN 的机器人实例。
	///
	/// # 参数
	///
	/// - `self_id` - 机器人的 UIN
	///
	/// # 返回值
	///
	/// 返回机器人实例，如果不存在则返回 [`None`]
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// if let Some(bot) = BotRegistry::get_with_bot_id(("123456") {
	///     println!("找到机器人: {}", bot.account().uin);
	/// }
	/// ```
	pub fn get_with_bot_id(self_id: &str) -> Option<Bot> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.values().find(|bot| bot.account().uin == self_id).cloned()
	}

	/// 获取所有已注册的机器人
	///
	/// # 返回值
	///
	/// 返回所有机器人实例的列表
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let all_bots = BotRegistry::all();
	/// println!("共有 {} 个机器人", all_bots.len());
	/// ```
	pub fn all() -> Vec<Bot> {
		STORE.all()
	}
}
