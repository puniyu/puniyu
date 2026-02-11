mod bot;
mod friend;
mod group;

use puniyu_registry::cooldown::{CooldownRegistry, CooldownScope};
use puniyu_types::event::{EventBase, message::MessageEvent};

/// 检查消息是否处于冷却期
pub fn is_cooling_down(event: &MessageEvent) -> bool {
	let bot_id = &event.self_id();

	if CooldownRegistry::is_cooling_down(&CooldownScope::Global) {
		return true;
	}

	if CooldownRegistry::is_cooling_down(&CooldownScope::Bot { bot_id }) {
		return true;
	}

	match event {
		MessageEvent::Friend(m) => {
			let user_id = &m.user_id().to_string();
			let scope = CooldownScope::Friend { bot_id, user_id };
			CooldownRegistry::is_cooling_down(&scope)
		}
		MessageEvent::Group(m) => {
			let group_id = &m.group_id().to_string();
			let user_id = &m.user_id().to_string();

			[
				CooldownScope::Group { bot_id, group_id },
				CooldownScope::GroupMember { bot_id, group_id, user_id },
			]
			.iter()
			.any(|scope| CooldownRegistry::is_cooling_down(scope))
		}
	}
}

/// 设置消息冷却时间
pub fn set_cooldown(event: &MessageEvent) {
	let bot_id = event.self_id();
	bot::set_cooldown(event.self_id());
	match event {
		MessageEvent::Friend(m) => {
			friend::set_cooldown(m.user_id(), bot_id);
		}
		MessageEvent::Group(m) => {
			group::set_cooldown(m.group_id(), m.user_id(), bot_id);
		}
	}
}
