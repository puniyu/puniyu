use puniyu_registry::cooldown::{CooldownRegistry, CooldownScope};
use puniyu_types::event::{EventBase, message::MessageEvent};

/// 检查消息是否处于冷却期
pub fn is_cooling_down(event: &MessageEvent) -> bool {
	match event {
		MessageEvent::Friend(m) => CooldownRegistry::is_cooling_down(&CooldownScope::Friend {
			bot_id: m.self_id(),
			user_id: m.user_id(),
		}),
		MessageEvent::Group(m) => {
			CooldownRegistry::is_cooling_down(&CooldownScope::Group {
				bot_id: m.self_id(),
				group_id: m.group_id(),
			}) || CooldownRegistry::is_cooling_down(&CooldownScope::GroupUser {
				bot_id: m.self_id(),
				group_id: m.group_id(),
				user_id: m.user_id(),
			})
		}
	}
}
