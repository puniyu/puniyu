use puniyu_registry::cooldown::{CooldownRegistry, CooldownScope};
use puniyu_types::event::{EventBase, message::MessageEvent};

/// 检查消息是否处于冷却期
pub fn is_cooling_down(event: &MessageEvent) -> bool {
	match event {
		MessageEvent::Friend(m) => {
			let scope = CooldownScope::Friend { bot_id: m.self_id(), user_id: m.user_id() };
			CooldownRegistry::is_cooling_down(&scope)
		}
		MessageEvent::Group(m) => {
			let group_scope = CooldownScope::Group { bot_id: m.self_id(), group_id: m.group_id() };
			let group_member_scope = CooldownScope::GroupMember {
				bot_id: m.self_id(),
				group_id: m.group_id(),
				user_id: m.user_id(),
			};
			CooldownRegistry::is_cooling_down(&group_scope)
				|| CooldownRegistry::is_cooling_down(&group_member_scope)
		}
	}
}
