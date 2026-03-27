use puniyu_config::group::GroupConfig;
use puniyu_cooldown::{CooldownRegistry, CooldownScope};
use puniyu_error::registry::Error;
use std::time::Duration;

pub(crate) fn set_cooldown(group_id: &str, user_id: &str, bot_id: &str) -> Result<(), Error> {
	let config = GroupConfig::get();
	let global_config = config.global();
	let group_config = config.group(group_id);
	set_group_cooldown(global_config.cd(), bot_id, group_id)?;
	set_group_cooldown(group_config.cd(), bot_id, group_id)?;
	set_member_cooldown(global_config.user_cd(), bot_id, group_id, user_id)?;
	set_member_cooldown(group_config.user_cd(), bot_id, group_id, user_id)?;

	Ok(())
}

/// 设置群组级别冷却
fn set_group_cooldown(cd: u64, bot_id: &str, group_id: &str) -> Result<(), Error> {
	if cd > 0 {
		CooldownRegistry::set_cooldown(
			&CooldownScope::Group { bot_id, group_id },
			Duration::from_millis(cd),
		)?;
	}
	Ok(())
}

/// 设置群成员级别冷却
fn set_member_cooldown(cd: u64, bot_id: &str, group_id: &str, user_id: &str) -> Result<(), Error> {
	if cd > 0 {
		CooldownRegistry::set_cooldown(
			&CooldownScope::GroupMember { bot_id, group_id, user_id },
			Duration::from_millis(cd),
		)?;
	}
	Ok(())
}
