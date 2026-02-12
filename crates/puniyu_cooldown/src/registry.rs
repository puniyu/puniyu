mod store;

use crate::CooldownScope;
use chrono::Utc;
use puniyu_error::registry::Error;
use std::sync::LazyLock;
use std::time::Duration;
use store::CooldownStore;

static STORE: LazyLock<CooldownStore> = LazyLock::new(CooldownStore::new);

/// 冷却注册表
///
/// 提供冷却时间的管理功能，包括设置、检查和清理冷却记录。
///
/// # 功能
///
/// - 检查是否处于冷却期
/// - 设置冷却时间
/// - 清除冷却记录
/// - 清理过期记录
///
/// # 示例
///
/// ```rust
/// use puniyu_cooldown::{CooldownRegistry, CooldownScope};
/// use std::time::Duration;
///
/// let scope = CooldownScope::Global;
///
/// // 检查冷却状态
/// if !CooldownRegistry::is_cooling_down(&scope) {
///     // 执行操作
///     println!("执行操作");
///     
///     // 设置冷却
///     CooldownRegistry::set_cooldown(&scope, Duration::from_secs(60)).unwrap();
/// }
/// ```
pub struct CooldownRegistry;

impl CooldownRegistry {
	/// 检查是否处于冷却期
	///
	/// # 参数
	///
	/// - `scope` - 冷却范围
	///
	/// # 返回值
	///
	/// 如果处于冷却期返回 `true`，否则返回 `false`。
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_cooldown::{CooldownRegistry, CooldownScope};
	///
	/// let scope = CooldownScope::Global;
	/// if CooldownRegistry::is_cooling_down(&scope) {
	///     println!("正在冷却中");
	/// }
	/// ```
	pub fn is_cooling_down(scope: &CooldownScope) -> bool {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		let key = scope.to_string();
		if let Some(&expire_time) = map.get(&key) { timestamp() < expire_time } else { false }
	}

	/// 设置冷却记录
	///
	/// 为指定范围设置冷却时间。如果持续时间为 0，则不设置冷却。
	///
	/// # 参数
	///
	/// - `scope` - 冷却范围
	/// - `duration` - 冷却持续时间
	///
	/// # 返回值
	///
	/// 成功返回 `Ok(())`，失败返回错误信息。
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_cooldown::{CooldownRegistry, CooldownScope};
	/// use std::time::Duration;
	///
	/// let scope = CooldownScope::Friend {
	///     bot_id: "123456",
	///     user_id: "789012",
	/// };
	///
	/// // 设置 30 秒冷却
	/// CooldownRegistry::set_cooldown(&scope, Duration::from_secs(30)).unwrap();
	/// ```
	pub fn set_cooldown(scope: &CooldownScope, duration: Duration) -> Result<(), Error> {
		let time = duration.as_millis() as u64;
		if time == 0 {
			return Ok(());
		}
		let key = scope.to_string();
		let expire_time = timestamp() + time;
		STORE.insert(key, expire_time)
	}

	/// 清除冷却记录
	///
	/// 立即移除指定范围的冷却记录。
	///
	/// # 参数
	///
	/// - `scope` - 冷却范围
	///
	/// # 返回值
	///
	/// 成功返回 `Ok(())`，失败返回错误信息。
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_cooldown::{CooldownRegistry, CooldownScope};
	///
	/// let scope = CooldownScope::Global;
	/// CooldownRegistry::clear_cooldown(&scope).unwrap();
	/// ```
	pub fn clear_cooldown(scope: &CooldownScope) -> Result<(), Error> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		let key = scope.to_string();
		map.remove(&key);
		Ok(())
	}

	/// 清理所有过期的冷却记录
	///
	/// 遍历所有冷却记录，移除已过期的记录以释放内存。
	/// 建议定期调用此方法进行清理。
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_cooldown::CooldownRegistry;
	///
	/// // 定期清理过期记录
	/// CooldownRegistry::cleanup_expired();
	/// ```
	pub fn cleanup_expired() {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		let time = timestamp();
		map.retain(|_, &mut expire_time| expire_time > time)
	}
}

/// 获取当前时间戳（毫秒）
///
/// # 返回值
///
/// 返回当前 UTC 时间的毫秒时间戳。
pub(crate) fn timestamp() -> u64 {
	Utc::now().timestamp_millis() as u64
}
