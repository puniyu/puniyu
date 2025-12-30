use puniyu_config::Config;
use puniyu_registry::cooldown::{CooldownRegistry, CooldownScope};
use std::time::Duration;

pub(crate) fn set_cooldown(group_id: &str, user_id: &str, bot_id: &str) {
	let config = Config::group();
	let global_config = config.global();

	if global_config.cd() > 0 {
		CooldownRegistry::set_cooldown(
			&CooldownScope::Group { bot_id, group_id },
			Duration::from_millis(global_config.cd()),
		);
	}

	if global_config.user_cd() > 0 {
		CooldownRegistry::set_cooldown(
			&CooldownScope::GroupMember { bot_id, group_id, user_id },
			Duration::from_millis(global_config.user_cd()),
		);
	}

	let group_config = config.group(group_id);

	if group_config.cd() > 0 {
		CooldownRegistry::set_cooldown(
			&CooldownScope::Group { bot_id, group_id },
			Duration::from_millis(group_config.cd()),
		);
	}

	if group_config.user_cd() > 0 {
		CooldownRegistry::set_cooldown(
			&CooldownScope::GroupMember { bot_id, group_id, user_id },
			Duration::from_millis(group_config.user_cd()),
		);
	}
}
