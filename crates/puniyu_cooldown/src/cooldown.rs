use crate::{CooldownScope, CooldownState};
use scc::HashMap;
use std::sync::LazyLock;
use std::time::{Duration, Instant};

static STORE: LazyLock<HashMap<CooldownScope, Instant>> = LazyLock::new(HashMap::new);

/// 全局消息冷却器。
pub struct Cooldown;

impl Cooldown {
	/// 设置或刷新指定作用域的冷却窗口。
	///
	/// 持续时间为零时移除该作用域的已有记录。
	pub fn set(scope: &CooldownScope, duration: Duration) {
		if duration.is_zero() {
			STORE.remove_sync(scope);
		} else {
			let now = Instant::now();
			STORE.insert_sync(scope.clone(), deadline(now, duration)).ok();
		}
	}

	/// 查询指定作用域当前是否处于冷却中。
	///
	/// 此方法不会创建或刷新冷却窗口。
	pub fn check(scope: &CooldownScope) -> CooldownState {
		match Self::remaining(scope) {
			Some(remaining) => CooldownState::CoolingDown { remaining },
			None => CooldownState::Ready,
		}
	}

	/// 查询指定作用域的剩余冷却时间。
	///
	/// 不存在有效冷却记录时返回 `None`。
	pub fn remaining(scope: &CooldownScope) -> Option<Duration> {
		let now = Instant::now();
		let result = STORE.read_sync(scope, |_, &deadline| {
			let remaining = deadline.saturating_duration_since(now);
			(!remaining.is_zero()).then_some(remaining)
		});
		let remaining = result.flatten();
		if remaining.is_none() {
			STORE.remove_sync(scope);
		}
		remaining
	}

	/// 移除指定作用域的冷却记录。
	///
	/// 返回是否实际移除了记录。
	pub fn remove(scope: &CooldownScope) -> bool {
		STORE.remove_sync(scope).is_some()
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
		if duration.is_zero() {
			STORE.remove_sync(scope);
			return CooldownState::Ready;
		}

		let now = Instant::now();
		if let Some(remaining) = remaining_in(scope, now) {
			return CooldownState::CoolingDown { remaining };
		}

		STORE.insert_sync(scope.clone(), deadline(now, duration)).ok();
		CooldownState::Ready
	}
}

fn remaining_in(scope: &CooldownScope, now: Instant) -> Option<Duration> {
	let result = STORE.read_sync(scope, |_, &deadline| {
		let remaining = deadline.saturating_duration_since(now);
		(!remaining.is_zero()).then_some(remaining)
	});
	let remaining = result.flatten();
	if remaining.is_none() {
		STORE.remove_sync(scope);
	}
	remaining
}

fn deadline(now: Instant, duration: Duration) -> Instant {
	now.checked_add(duration).expect("cooldown duration exceeds supported Instant range")
}
