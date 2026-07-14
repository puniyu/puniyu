use crate::registry::CooldownRegistry;
use crate::{CooldownScope, CooldownState};
use std::time::Duration;

/// 全局消息冷却器。
///
/// 对外提供稳定的冷却 API，具体状态由内部冷却注册表统一管理。
pub struct Cooldown;

impl Cooldown {
	/// 设置或刷新指定作用域的冷却窗口。
	///
	/// 持续时间为零时移除该作用域的已有记录。
	///
	pub fn set(scope: &CooldownScope, duration: Duration) {
		CooldownRegistry::set(scope, duration);
	}

	/// 查询指定作用域当前是否处于冷却中。
	///
	/// 此方法不会创建或刷新冷却窗口。
	pub fn check(scope: &CooldownScope) -> CooldownState {
		CooldownRegistry::check(scope)
	}

	/// 查询指定作用域的剩余冷却时间。
	///
	/// 不存在有效冷却记录时返回 `None`。
	pub fn remaining(scope: &CooldownScope) -> Option<Duration> {
		CooldownRegistry::remaining(scope)
	}

	/// 移除指定作用域的冷却记录。
	///
	/// 返回是否实际移除了记录。
	pub fn remove(scope: &CooldownScope) -> bool {
		CooldownRegistry::remove(scope)
	}

	/// 原子检查并按需开始指定作用域的固定冷却窗口。
	///
	/// 没有有效记录时返回 [`CooldownState::Ready`] 并开始冷却；冷却期间返回
	/// [`CooldownState::CoolingDown`] 且不会延长当前窗口。持续时间为零时移除
	/// 已有记录并直接返回 [`CooldownState::Ready`]。
	///
	/// # Panics
	///
	/// 当持续时间超出平台 [`std::time::Instant`] 可表示范围时 panic。
	pub fn check_and_set(scope: &CooldownScope, duration: Duration) -> CooldownState {
		CooldownRegistry::check_and_set(scope, duration)
	}
}
