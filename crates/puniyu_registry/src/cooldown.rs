mod store;
mod types;
pub use types::CooldownScope;

use crate::Result;
use chrono::Utc;
use std::sync::LazyLock;
use std::time::Duration;
use store::CooldownStore;

static STORE: LazyLock<CooldownStore> = LazyLock::new(CooldownStore::new);

pub struct CooldownRegistry;

impl CooldownRegistry {
	/// 检查是否处于冷却期
	#[inline]
	pub fn is_cooling_down(scope: &CooldownScope) -> bool {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		let key = scope.make_key();
		if let Some(&expire_time) = map.get(&key) { timestamp() < expire_time } else { false }
	}

	/// 设置冷却记录
	#[inline]
	pub fn set_cooldown(scope: &CooldownScope, duration: Duration) -> Result<()> {
		let time = duration.as_millis() as u64;
		if time == 0 {
			return Ok(());
		}
		let key = scope.make_key();
		let expire_time = timestamp() + time;
		STORE.insert(key, expire_time)
	}

	/// 清除冷却记录
	#[inline]
	pub fn clear_cooldown(scope: &CooldownScope) -> Result<()> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		let key = scope.make_key();
		map.remove(&key);
		Ok(())
	}

	/// 清理所有过期的冷却记录
	#[inline]
	pub fn cleanup_expired() {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		let time = timestamp();
		map.retain(|_, &mut expire_time| expire_time > time)
	}
}

pub(crate) fn timestamp() -> u64 {
	Utc::now().timestamp_millis() as u64
}
