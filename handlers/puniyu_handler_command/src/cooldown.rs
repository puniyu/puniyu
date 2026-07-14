use puniyu_cooldown::{Cooldown, CooldownScope, CooldownState};
use puniyu_event::message::MessageEvent;
use std::time::Duration;

pub(crate) fn check(message: &MessageEvent<'_>, duration: Duration) -> CooldownState {
	let scope = if let Some(group) = message.as_group() {
		CooldownScope::group_member(message.self_id(), group.group_id(), message.user_id())
	} else if let Some(group) = message.as_group_temp() {
		CooldownScope::group_member(message.self_id(), group.group_id(), message.user_id())
	} else if let Some(guild) = message.as_guild() {
		CooldownScope::group_member(message.self_id(), guild.guild_id(), message.user_id())
	} else {
		CooldownScope::friend(message.self_id(), message.user_id())
	};

	Cooldown::check_and_set(&scope, duration)
}
