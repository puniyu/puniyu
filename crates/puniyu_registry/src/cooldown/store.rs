use crate::cooldown::CooldownScope;
use chrono::Utc;
use std::{
	collections::HashMap,
	sync::{Arc, RwLock},
	time::Duration,
};

#[derive(Default)]
pub(crate) struct CooldownStore(pub(crate) Arc<RwLock<HashMap<String, u64>>>);

impl CooldownStore {
	pub fn new() -> Self {
		Self::default()
	}
	fn now() -> u64 {
		Utc::now().timestamp_millis() as u64
	}

	pub fn is_cooling_down(&self, scope: &CooldownScope) -> bool {
		let key = scope.make_key();
		if let Some(&expire_time) = self.0.read().unwrap().get(&key) {
			Self::now() < expire_time
		} else {
			false
		}
	}

	pub fn set_cooldown(&self, scope: &CooldownScope, duration: Duration) {
		let time = duration.as_millis() as u64;
		if time == 0 {
			return;
		}
		let key = scope.make_key();
		let expire_time = Self::now() + time;
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
