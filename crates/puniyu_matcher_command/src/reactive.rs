use puniyu_config::{Config, ReactiveMode};
use puniyu_types::event::{EventBase, message::MessageEvent};

use crate::message;

/// 获取 bot 配置信息（别名列表和响应模式）
pub fn get_bot_config(event: &MessageEvent) -> (Vec<String>, ReactiveMode) {
	let bot_id = match event {
		MessageEvent::Friend(m) => m.self_id().to_string(),
		MessageEvent::Group(m) => m.self_id().to_string(),
	};
	let bot_config = Config::bot();
	let bot_option = bot_config.bot(&bot_id);
	let aliases = bot_option.alias();
	let mode = bot_option.mode();
	let aliases = if aliases.is_empty() { bot_config.global().alias() } else { aliases };
	(aliases, mode)
}

/// 尝试移除 bot 别名前缀，返回 (移除后的文本, 是否匹配到别名)
pub fn strip_bot_alias(text: &str, aliases: &[String]) -> (String, bool) {
	for alias in aliases {
		if !alias.is_empty()
			&& let Some(stripped) = text.strip_prefix(alias)
		{
			return (stripped.trim_start().to_string(), true);
		}
	}
	(text.to_string(), false)
}

/// 根据响应模式检查是否应该响应消息
pub fn check_mode(event: &MessageEvent, mode: &ReactiveMode, has_alias: bool) -> bool {
	match mode {
		ReactiveMode::All => true,
		ReactiveMode::AtBot => message::is_at_bot(event),
		ReactiveMode::Alias => has_alias,
		ReactiveMode::AtOrAlias => message::is_at_bot(event) || has_alias,
		ReactiveMode::Master => event.is_master(),
	}
}
