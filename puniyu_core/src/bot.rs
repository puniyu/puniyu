mod registry;

use puniyu_registry::bot::{Bot, BotId, registry::BotRegistry};

/// 获取Bot实例
///
/// # 参数
///
/// * `id` - Bot的ID
///
/// # 返回值
///
/// * `Option<Bot>` - 如果找到Bot，则返回Bot实例，否则返回None
pub fn get_bot(id: impl Into<BotId>) -> Option<Bot> {
	let bot_id: BotId = id.into();
	match bot_id {
		BotId::Index(index) => BotRegistry::get_with_index(index),
		BotId::SelfId(id) => BotRegistry::get_with_self_id(id.as_str()),
	}
}

/// 注销Bot实例
///
/// # 参数
///
/// * `id` - Bot的ID
///
/// # 返回值
///
/// * `bool` - 如果注销成功，则返回true，否则返回false
pub fn unregister_bot<T: Into<BotId>>(id: T) -> bool {
	let bot_id: BotId = id.into();
	match bot_id {
		BotId::Index(index) => BotRegistry::unregister(index),
		BotId::SelfId(id) => BotRegistry::unregister_with_id(id.as_str()),
	}
}
