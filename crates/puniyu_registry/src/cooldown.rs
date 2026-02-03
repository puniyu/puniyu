mod store;
mod types;
use store::CooldownStore;

use std::sync::LazyLock;
use std::time::Duration;

pub use types::CooldownScope;

static STORE: LazyLock<CooldownStore> = LazyLock::new(CooldownStore::new);
pub struct CooldownRegistry;

impl CooldownRegistry {
	/// 检查是否处于冷却期
	pub fn is_cooling_down(scope: &CooldownScope) -> bool {
		STORE.is_cooling_down(scope)
	}

	/// 设置冷却记录
	pub fn set_cooldown(scope: &CooldownScope, duration: Duration) {
		STORE.set_cooldown(scope, duration);
	}

	/// 清除冷却记录
	pub fn clear_cooldown(scope: &CooldownScope) {
		STORE.clear_cooldown(scope);
	}

	/// 清理所有过期的冷却记录
	pub fn cleanup_expired() {
		STORE.cleanup_expired();
	}
}
