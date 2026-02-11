use puniyu_config::Config;
use puniyu_config::ReactiveMode;

pub fn get_bot_alias(bot_id: &str) -> Vec<String> {
	let bot_config = Config::bot();
	let bot_option = bot_config.bot(bot_id);
	let aliases = bot_option.alias();
	if aliases.is_empty() { bot_config.global().alias() } else { aliases }
}

pub fn get_bot_reactive_mode(bot_id: &str) -> ReactiveMode {
	let bot_config = Config::bot();
	let bot_option = bot_config.bot(bot_id);
	bot_option.mode()
}
