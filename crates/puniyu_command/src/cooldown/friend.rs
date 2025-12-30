use puniyu_config::Config;
use puniyu_registry::cooldown::{CooldownRegistry, CooldownScope};
use std::time::Duration;

pub(crate) fn set_cooldown(user_id: &str, bot_id: &str) {
	let friend_config = Config::friend();
	let global_config = friend_config.global();
	if global_config.cd() > 0 {
		CooldownRegistry::set_cooldown(
			&CooldownScope::Friend { bot_id, user_id },
			Duration::from_millis(global_config.cd()),
		);
	}

	let friend_config = friend_config.friend(user_id);
	if friend_config.cd() > 0 {
		CooldownRegistry::set_cooldown(
			&CooldownScope::Friend { bot_id, user_id },
			Duration::from_millis(friend_config.cd()),
		);
	}
}
