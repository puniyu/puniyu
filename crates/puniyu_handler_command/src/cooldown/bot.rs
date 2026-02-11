use puniyu_config::Config;
use puniyu_registry::cooldown::{CooldownRegistry, CooldownScope};
use std::time::Duration;

pub(crate) fn set_cooldown(bot_id: &str) {
	let config = Config::bot();
	let global_config = config.global();
	if global_config.cd() > 0 {
		CooldownRegistry::set_cooldown(
			&CooldownScope::Global,
			Duration::from_millis(global_config.cd()),
		);
	}

	let bot_config = config.bot(bot_id);
	if bot_config.cd() > 0 {
		CooldownRegistry::set_cooldown(
			&CooldownScope::Bot { bot_id },
			Duration::from_millis(bot_config.cd()),
		);
	}
}
