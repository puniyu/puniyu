use crate::store::STORE;
use puniyu_logger::{info, owo_colors::OwoColorize, warn};
pub use puniyu_types::bot::Bot;
pub use puniyu_types::bot::BotId;

pub struct BotRegistry;

impl BotRegistry {
	pub fn get_all() -> Vec<Bot> {
		STORE.bot().all()
	}

	pub fn get_with_index(index: u64) -> Option<Bot> {
		STORE.bot().get_with_index(index)
	}

	pub fn get_with_self_id(self_id: &str) -> Option<Bot> {
		STORE.bot().get_with_self_id(self_id)
	}

	pub fn register(bot: Bot) -> u64 {
		let self_id = bot.account.uin.clone();
		let adapter = bot.adapter.clone();
		let index = STORE.bot().insert(bot);
		info!(
			"[{}] [{}] 注册成功",
			format!("Bot: {}", self_id).fg_rgb::<221, 160, 221>(),
			format!("adapter:{}", adapter.name).fg_rgb::<255, 182, 193>()
		);
		index
	}

	pub fn unregister_with_index(index: u64) -> bool {
		if let Some(bot) = STORE.bot().remove_with_index(index) {
			info!(
				"[Bot: index-{}][adapter:{} v{}] 卸载成功",
				bot.account.uin, bot.adapter.name, bot.adapter.version
			);
			return true;
		}
		warn!("[Bot: index-{}] 卸载失败未找到指定的Bot", index);
		false
	}

	pub fn unregister_with_id(id: impl Into<String>) -> bool {
		let bot_id = id.into();
		if let Some(bot) = STORE.bot().remove_with_self_id(bot_id.as_str()) {
			info!(
				"[Bot: {}][adapter:{} v{}] 卸载成功",
				bot.account.uin, bot.adapter.name, bot.adapter.version
			);
			return true;
		}
		warn!("[Bot: id-{}] 卸载失败未找到指定的Bot", bot_id);
		false
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
	($adapter:expr, $account:expr, $api:expr) => {
		let bot = $crate::bot::Bot { adapter: $adapter, api: $api, account: $account };
		$crate::bot::BotRegistry::register(bot)
	};
	(adapter: $adapter:expr, account: $account:expr, api: $api:expr) => {
		let bot = $crate::bot::Bot { adapter: $adapter, api: $api, account: $account };
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
		BotRegistry::unregister_with_id($id)
	};
	(index: $index:expr) => {
		BotRegistry::unregister_with_index($index)
	};
	(id: $id:expr) => {
		BotRegistry::unregister_with_id($id)
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
		BotId::SelfId(id) => BotRegistry::get_with_self_id(id.as_str()),
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
	BotRegistry::get_all().len() as u64
}
