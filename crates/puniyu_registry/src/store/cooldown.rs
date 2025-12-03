use std::{
	collections::HashMap,
	sync::{Arc, RwLock},
	time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum CooldownScope<'a> {
	Friend { bot_id: &'a str, user_id: &'a str },
	Group { bot_id: &'a str, group_id: &'a str },
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

pub(crate) struct CooldownStore(pub(crate) Arc<RwLock<HashMap<String, u64>>>);

impl CooldownStore {
	fn now() -> u64 {
		SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
	}

	pub fn is_cooling_down(&self, scope: &CooldownScope) -> bool {
		let key = scope.make_key();
		if let Some(&expire_time) = self.0.read().unwrap().get(&key) {
			Self::now() < expire_time
		} else {
			false
		}
	}

	pub fn set_cooldown(&self, scope: &CooldownScope, duration: u64) {
		if duration == 0 {
			return;
		}
		let key = scope.make_key();
		let expire_time = Self::now() + duration;
		self.0.write().unwrap().insert(key, expire_time);
	}

	pub fn clear_cooldown(&self, scope: &CooldownScope) {
		let key = scope.make_key();
		self.0.write().unwrap().remove(&key);
	}

	pub fn cleanup_expired(&self) {
		let now = Self::now();
		self.0.write().unwrap().retain(|_, &mut expire_time| expire_time > now);
	}
}
