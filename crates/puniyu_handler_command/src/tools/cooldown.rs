mod bot;
mod friend;
mod group;

use puniyu_context::MessageContext;
use puniyu_cooldown::{CooldownRegistry, CooldownScope};
use puniyu_error::registry::Error;
use puniyu_event::EventBase;

/// 检查消息是否处于冷却期
pub fn is_cooling_down(event: &MessageContext) -> bool {
	let bot_id = event.self_id();

	if CooldownRegistry::is_cooling_down(&CooldownScope::Global) {
		return true;
	}

	if CooldownRegistry::is_cooling_down(&CooldownScope::Bot { bot_id }) {
		return true;
	}

	if let Some(event) = event.as_group() {
		let group_id = event.group_id();
		let user_id = event.user_id();

		[
			CooldownScope::Group { bot_id, group_id },
			CooldownScope::GroupMember { bot_id, group_id, user_id },
		]
		.iter()
		.any(|scope| CooldownRegistry::is_cooling_down(scope))
	} else {
		let user_id = event.user_id();
		let scope = CooldownScope::Friend { bot_id, user_id };
		CooldownRegistry::is_cooling_down(&scope)
	}
}

/// 设置消息冷却时间
pub fn set_cooldown(event: &MessageContext) -> Result<(), Error> {
	let bot_id = event.self_id();
	bot::set_cooldown(event.self_id())?;
	if let Some(event) = event.as_group() {
		group::set_cooldown(event.group_id(), event.user_id(), bot_id)?;
	} else {
		friend::set_cooldown(event.user_id(), bot_id)?;
	}
	Ok(())
}
