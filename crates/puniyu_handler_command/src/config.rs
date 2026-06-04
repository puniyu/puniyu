use puniyu_core::config::{ReactiveMode, bot_config};

pub fn get_bot_alias(bot_id: &str) -> Vec<String> {
	bot_config().bot(bot_id).alias().into_iter().map(str::to_string).collect()
}

pub fn get_bot_reactive_mode(bot_id: &str) -> ReactiveMode {
	bot_config().bot(bot_id).mode()
}
