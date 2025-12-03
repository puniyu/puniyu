use puniyu_registry::cooldown::CooldownRegistry;
use puniyu_types::event::{EventBase, message::MessageEvent};

/// 检查消息是否处于冷却期
pub fn is_cooling_down(event: &MessageEvent) -> bool {
	match event {
		MessageEvent::Friend(m) => {
			CooldownRegistry::is_friend_cooling_down(m.self_id(), m.user_id())
		}
		MessageEvent::Group(m) => {
			CooldownRegistry::is_group_cooling_down(m.self_id(), m.group_id())
				|| CooldownRegistry::is_group_user_cooling_down(
					m.self_id(),
					m.group_id(),
					m.user_id(),
				)
		}
	}
}
