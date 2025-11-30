use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

/// 冷却作用域
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CooldownScope<'a> {
	/// 好友消息冷却
	Friend { bot_id: &'a str, user_id: &'a str },
	/// 群组冷却
	Group { bot_id: &'a str, group_id: &'a str },
	/// 群组内用户冷却
	GroupUser { bot_id: &'a str, group_id: &'a str, user_id: &'a str },
}

impl CooldownScope<'_> {
	pub fn make_key(&self) -> String {
		match self {
			Self::Friend { bot_id, user_id } => format!("bot:{}:friend:{}", bot_id, user_id),
			Self::Group { bot_id, group_id } => format!("bot:{}:group:{}", bot_id, group_id),
			Self::GroupUser { bot_id, group_id, user_id } => {
				format!("bot:{}:group:{}:user:{}", bot_id, group_id, user_id)
			}
		}
	}
}

#[derive(Default)]
pub(crate) struct CooldownStore(Arc<Mutex<HashMap<String, u64>>>);

impl CooldownStore {
	fn now() -> u64 {
		SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
	}

	/// 检查是否处于冷却期
	pub fn is_cooling_down(&self, scope: &CooldownScope) -> bool {
		let key = scope.make_key();
		let store = self.0.lock().unwrap();
		if let Some(&expire_time) = store.get(&key) {
			Self::now() < expire_time
		} else {
			false
		}
	}

	/// 设置冷却时间
	pub fn set_cooldown(&self, scope: &CooldownScope, duration: u64) {
		if duration == 0 {
			return;
		}
		let key = scope.make_key();
		let mut store = self.0.lock().unwrap();
		let expire_time = Self::now() + duration;
		store.insert(key, expire_time);
	}

	/// 清除指定键的冷却状态
	pub fn clear_cooldown(&self, scope: &CooldownScope) {
		let key = scope.make_key();
		let mut store = self.0.lock().unwrap();
		store.remove(&key);
	}

	/// 清理所有过期的冷却记录
	pub fn cleanup_expired(&self) {
		let mut store = self.0.lock().unwrap();
		let now = Self::now();
		store.retain(|_, &mut expire_time| expire_time > now);
	}
}
