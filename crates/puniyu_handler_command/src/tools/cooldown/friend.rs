use puniyu_config::friend::FriendConfig;
use puniyu_cooldown::{CooldownRegistry, CooldownScope};
use puniyu_error::registry::Error;
use std::time::Duration;

pub(crate) fn set_cooldown(user_id: &str, bot_id: &str) -> Result<(), Error> {
	let friend_config = FriendConfig::get();
	let scope = CooldownScope::Friend { bot_id, user_id };
	let global_cd = friend_config.global().cd();
	if global_cd > 0 {
		CooldownRegistry::set_cooldown(&scope, Duration::from_millis(global_cd))?;
	}
	let friend_cd = friend_config.friend(user_id).cd();
	if friend_cd > 0 {
		CooldownRegistry::set_cooldown(&scope, Duration::from_millis(friend_cd))?;
	}

	Ok(())
}
