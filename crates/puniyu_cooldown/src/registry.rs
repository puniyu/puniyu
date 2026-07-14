mod store;

use crate::registry::store::CooldownStore;
use crate::{CooldownScope, CooldownState};
use std::collections::HashMap;
use std::sync::LazyLock;
use std::time::{Duration, Instant};

static STORE: LazyLock<CooldownStore> = LazyLock::new(CooldownStore::new);

pub(crate) struct CooldownRegistry;

impl CooldownRegistry {
	pub(crate) fn set(scope: &CooldownScope, duration: Duration) {
		let mut map = STORE.raw().write().expect("poisoned lock");
		if duration.is_zero() {
			map.remove(scope);
		} else {
			let now = Instant::now();
			map.insert(scope.clone(), deadline(now, duration));
		}
	}

	pub(crate) fn check(scope: &CooldownScope) -> CooldownState {
		match Self::remaining(scope) {
			Some(remaining) => CooldownState::CoolingDown { remaining },
			None => CooldownState::Ready,
		}
	}

	pub(crate) fn remaining(scope: &CooldownScope) -> Option<Duration> {
		let mut map = STORE.raw().write().expect("poisoned lock");
		Self::remaining_in(&mut map, scope, Instant::now())
	}

	pub(crate) fn remove(scope: &CooldownScope) -> bool {
		STORE.raw().write().expect("poisoned lock").remove(scope).is_some()
	}

	pub(crate) fn check_and_set(scope: &CooldownScope, duration: Duration) -> CooldownState {
		let mut map = STORE.raw().write().expect("poisoned lock");
		if duration.is_zero() {
			map.remove(scope);
			return CooldownState::Ready;
		}

		let now = Instant::now();
		if let Some(remaining) = Self::remaining_in(&mut map, scope, now) {
			return CooldownState::CoolingDown { remaining };
		}

		map.insert(scope.clone(), deadline(now, duration));
		CooldownState::Ready
	}

	fn remaining_in(
		map: &mut HashMap<CooldownScope, Instant>,
		scope: &CooldownScope,
		now: Instant,
	) -> Option<Duration> {
		let remaining = map.get(scope).copied().and_then(|deadline| {
			let remaining = deadline.saturating_duration_since(now);
			(!remaining.is_zero()).then_some(remaining)
		});
		if remaining.is_none() {
			map.remove(scope);
		}
		remaining
	}
}

fn deadline(now: Instant, duration: Duration) -> Instant {
	now.checked_add(duration).expect("cooldown duration exceeds supported Instant range")
}
