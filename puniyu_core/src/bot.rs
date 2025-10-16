use puniyu_bot::{Bot, BotId, BotRegistry};

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
