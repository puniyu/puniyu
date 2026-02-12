use puniyu_cooldown::{CooldownRegistry, CooldownScope};
use std::thread;
use std::time::Duration;

#[test]
fn test_global_cooldown() {
	let scope = CooldownScope::Global;

	// 初始状态不应该在冷却中
	assert!(!CooldownRegistry::is_cooling_down(&scope));

	// 设置冷却
	CooldownRegistry::set_cooldown(&scope, Duration::from_millis(100)).unwrap();

	// 应该在冷却中
	assert!(CooldownRegistry::is_cooling_down(&scope));

	// 等待冷却结束
	thread::sleep(Duration::from_millis(150));

	// 不应该在冷却中
	assert!(!CooldownRegistry::is_cooling_down(&scope));
}

#[test]
fn test_bot_cooldown() {
	let scope = CooldownScope::Bot { bot_id: "test_bot_1" };

	assert!(!CooldownRegistry::is_cooling_down(&scope));

	CooldownRegistry::set_cooldown(&scope, Duration::from_millis(100)).unwrap();
	assert!(CooldownRegistry::is_cooling_down(&scope));

	thread::sleep(Duration::from_millis(150));
	assert!(!CooldownRegistry::is_cooling_down(&scope));
}

#[test]
fn test_friend_cooldown() {
	let scope = CooldownScope::Friend { bot_id: "test_bot_2", user_id: "user_123" };

	assert!(!CooldownRegistry::is_cooling_down(&scope));

	CooldownRegistry::set_cooldown(&scope, Duration::from_millis(100)).unwrap();
	assert!(CooldownRegistry::is_cooling_down(&scope));

	thread::sleep(Duration::from_millis(150));
	assert!(!CooldownRegistry::is_cooling_down(&scope));
}

#[test]
fn test_group_cooldown() {
	let scope = CooldownScope::Group { bot_id: "test_bot_3", group_id: "group_456" };

	assert!(!CooldownRegistry::is_cooling_down(&scope));

	CooldownRegistry::set_cooldown(&scope, Duration::from_millis(100)).unwrap();
	assert!(CooldownRegistry::is_cooling_down(&scope));

	thread::sleep(Duration::from_millis(150));
	assert!(!CooldownRegistry::is_cooling_down(&scope));
}

#[test]
fn test_group_member_cooldown() {
	let scope = CooldownScope::GroupMember {
		bot_id: "test_bot_4",
		group_id: "group_789",
		user_id: "user_456",
	};

	assert!(!CooldownRegistry::is_cooling_down(&scope));

	CooldownRegistry::set_cooldown(&scope, Duration::from_millis(100)).unwrap();
	assert!(CooldownRegistry::is_cooling_down(&scope));

	thread::sleep(Duration::from_millis(150));
	assert!(!CooldownRegistry::is_cooling_down(&scope));
}

#[test]
fn test_zero_duration() {
	let scope = CooldownScope::Friend { bot_id: "test_zero", user_id: "user_zero" };

	// 先清除可能存在的冷却
	let _ = CooldownRegistry::clear_cooldown(&scope);

	// 设置 0 时长的冷却应该不生效
	CooldownRegistry::set_cooldown(&scope, Duration::from_secs(0)).unwrap();
	assert!(!CooldownRegistry::is_cooling_down(&scope));
}

#[test]
fn test_clear_cooldown() {
	let scope = CooldownScope::Friend { bot_id: "test_bot_5", user_id: "user_789" };

	// 设置冷却
	CooldownRegistry::set_cooldown(&scope, Duration::from_secs(10)).unwrap();
	assert!(CooldownRegistry::is_cooling_down(&scope));

	// 清除冷却
	CooldownRegistry::clear_cooldown(&scope).unwrap();
	assert!(!CooldownRegistry::is_cooling_down(&scope));
}

#[test]
fn test_multiple_scopes() {
	let scope1 = CooldownScope::Friend { bot_id: "bot_1", user_id: "user_1" };
	let scope2 = CooldownScope::Friend { bot_id: "bot_1", user_id: "user_2" };

	// 设置 scope1 的冷却
	CooldownRegistry::set_cooldown(&scope1, Duration::from_millis(100)).unwrap();

	// scope1 应该在冷却中，scope2 不应该
	assert!(CooldownRegistry::is_cooling_down(&scope1));
	assert!(!CooldownRegistry::is_cooling_down(&scope2));

	// 设置 scope2 的冷却
	CooldownRegistry::set_cooldown(&scope2, Duration::from_millis(100)).unwrap();

	// 两个都应该在冷却中
	assert!(CooldownRegistry::is_cooling_down(&scope1));
	assert!(CooldownRegistry::is_cooling_down(&scope2));
}

#[test]
fn test_cleanup_expired() {
	let scope1 = CooldownScope::Friend { bot_id: "bot_cleanup", user_id: "user_1" };
	let scope2 = CooldownScope::Friend { bot_id: "bot_cleanup", user_id: "user_2" };

	// 设置短期冷却
	CooldownRegistry::set_cooldown(&scope1, Duration::from_millis(50)).unwrap();
	// 设置长期冷却
	CooldownRegistry::set_cooldown(&scope2, Duration::from_millis(200)).unwrap();

	// 等待 scope1 过期
	thread::sleep(Duration::from_millis(100));

	// 清理过期记录
	CooldownRegistry::cleanup_expired();

	// scope1 应该不在冷却中，scope2 应该还在
	assert!(!CooldownRegistry::is_cooling_down(&scope1));
	assert!(CooldownRegistry::is_cooling_down(&scope2));
}
