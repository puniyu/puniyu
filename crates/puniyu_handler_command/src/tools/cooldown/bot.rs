use puniyu_config::bot_config;
use puniyu_cooldown::{CooldownRegistry, CooldownScope};
use puniyu_error::registry::Error;
use std::time::Duration;

pub(crate) fn set_cooldown(bot_id: &str) -> Result<(), Error> {
	let config = bot_config();

	let global_cd = config.global().cd();
	if global_cd > 0 {
		CooldownRegistry::set_cooldown(&CooldownScope::Global, Duration::from_millis(global_cd))?;
	}

	let bot_cd = config.bot(bot_id).cd();
	if bot_cd > 0 {
		CooldownRegistry::set_cooldown(
			&CooldownScope::Bot { bot_id },
			Duration::from_millis(bot_cd),
		)?;
	}

	Ok(())
}
