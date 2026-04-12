pub use puniyu_bot::Bot;
pub use puniyu_bot::BotId;
use puniyu_bot::BotRegistry;
use std::sync::Arc;
/// 按索引或 UIN 查询已注册的机器人。
pub fn get_bot<'b>(bot_id: impl Into<BotId<'b>>) -> Option<Arc<dyn Bot>> {
	let bot_id = bot_id.into();
	match bot_id {
		BotId::Index(id) => BotRegistry::get_with_index(id),
		BotId::SelfId(name) => BotRegistry::get_with_bot_id(name.as_ref()),
	}
}

/// 返回当前已注册的机器人数量。
pub fn get_bot_count() -> usize {
	BotRegistry::all().len()
}

/// 返回所有已注册的机器人副本。
pub fn get_all_bot() -> Vec<Arc<dyn Bot>> {
	BotRegistry::all()
}
