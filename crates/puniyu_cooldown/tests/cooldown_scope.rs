use puniyu_cooldown::CooldownScope;
use std::str::FromStr;

#[test]
fn test_scope_display_global() {
	let scope = CooldownScope::Global;
	assert_eq!(scope.to_string(), "global");
}

#[test]
fn test_scope_display_bot() {
	let scope = CooldownScope::Bot { bot_id: "123456" };
	assert_eq!(scope.to_string(), "bot:123456");
}

#[test]
fn test_scope_display_friend() {
	let scope = CooldownScope::Friend { bot_id: "123456", user_id: "789012" };
	assert_eq!(scope.to_string(), "bot:123456:userId:789012");
}

#[test]
fn test_scope_display_group() {
	let scope = CooldownScope::Group { bot_id: "123456", group_id: "456789" };
	assert_eq!(scope.to_string(), "bot:123456:groupId:456789");
}

#[test]
fn test_scope_display_group_member() {
	let scope =
		CooldownScope::GroupMember { bot_id: "123456", group_id: "456789", user_id: "789012" };
	assert_eq!(scope.to_string(), "bot:123456:groupId:456789:userId:789012");
}

#[test]
fn test_scope_clone() {
	let scope1 = CooldownScope::Friend { bot_id: "123456", user_id: "789012" };
	let scope2 = scope1.clone();

	assert_eq!(scope1, scope2);
}

#[test]
fn test_scope_equality() {
	let scope1 = CooldownScope::Friend { bot_id: "123456", user_id: "789012" };
	let scope2 = CooldownScope::Friend { bot_id: "123456", user_id: "789012" };
	let scope3 = CooldownScope::Friend { bot_id: "123456", user_id: "999999" };

	assert_eq!(scope1, scope2);
	assert_ne!(scope1, scope3);
}

#[test]
fn test_scope_default() {
	let scope: CooldownScope = Default::default();
	assert_eq!(scope, CooldownScope::Global);
}

#[test]
fn test_scope_debug() {
	let scope = CooldownScope::Friend { bot_id: "123456", user_id: "789012" };
	let debug_str = format!("{:?}", scope);

	assert!(debug_str.contains("Friend"));
	assert!(debug_str.contains("123456"));
	assert!(debug_str.contains("789012"));
}

#[test]
fn test_different_scope_types() {
	let global = CooldownScope::Global;
	let bot = CooldownScope::Bot { bot_id: "123" };
	let friend = CooldownScope::Friend { bot_id: "123", user_id: "456" };

	assert_ne!(global, bot);
	assert_ne!(bot, friend);
	assert_ne!(global, friend);
}

#[test]
fn test_scope_from_str() {
	let global = CooldownScope::from_str("global").unwrap();
	assert_eq!(global, CooldownScope::Global);

	assert!(CooldownScope::from_str("bot:123456").is_err());
	assert!(CooldownScope::from_str("bot:123:groupId:456:userId:789").is_err());
}

#[test]
fn test_scope_serde_roundtrip() {
	let scope = CooldownScope::GroupMember { bot_id: "123", group_id: "456", user_id: "789" };
	let json = serde_json::to_string(&scope).unwrap();
	let decoded: CooldownScope = serde_json::from_str(&json).unwrap();

	assert_eq!(scope, decoded);
}
