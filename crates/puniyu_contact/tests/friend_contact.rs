use puniyu_contact::{Contact, FriendContact, SceneType};

#[test]
fn test_friend_contact_creation() {
	let friend = FriendContact { scene: SceneType::Friend, peer: "123456", name: Some("Alice") };

	assert_eq!(friend.peer, "123456");
	assert_eq!(friend.name, Some("Alice"));
}

#[test]
fn test_friend_contact_without_name() {
	let friend = FriendContact { scene: SceneType::Friend, peer: "123456", name: None };

	assert_eq!(friend.peer, "123456");
	assert_eq!(friend.name, None);
}

#[test]
fn test_friend_contact_trait_methods() {
	let friend = FriendContact { scene: SceneType::Friend, peer: "123456", name: Some("Alice") };

	assert_eq!(friend.peer(), "123456");
	assert_eq!(friend.name(), Some("Alice"));
}

#[test]
fn test_friend_contact_clone() {
	let friend1 = FriendContact { scene: SceneType::Friend, peer: "123456", name: Some("Alice") };
	let friend2 = friend1.clone();

	assert_eq!(friend1, friend2);
}

#[test]
fn test_friend_contact_debug() {
	let friend = FriendContact { scene: SceneType::Friend, peer: "123456", name: Some("Alice") };
	let debug_str = format!("{:?}", friend);

	assert!(debug_str.contains("FriendContact"));
	assert!(debug_str.contains("123456"));
}

#[test]
fn test_friend_contact_equality() {
	let friend1 = FriendContact { scene: SceneType::Friend, peer: "123456", name: Some("Alice") };
	let friend2 = FriendContact { scene: SceneType::Friend, peer: "123456", name: Some("Alice") };
	let friend3 = FriendContact { scene: SceneType::Friend, peer: "789012", name: Some("Bob") };

	assert_eq!(friend1, friend2);
	assert_ne!(friend1, friend3);
}

#[test]
fn test_friend_contact_empty_peer() {
	let friend = FriendContact { scene: SceneType::Friend, peer: "", name: Some("NoID") };

	assert_eq!(friend.peer(), "");
	assert_eq!(friend.name(), Some("NoID"));
}

#[test]
fn test_friend_contact_unicode_name() {
	let friend =
		FriendContact { scene: SceneType::Friend, peer: "123456", name: Some("爱丽丝") };

	assert_eq!(friend.name(), Some("爱丽丝"));
}

#[test]
fn test_friend_contact_special_characters() {
	let friend =
		FriendContact { scene: SceneType::Friend, peer: "user@123", name: Some("User #1") };

	assert_eq!(friend.peer(), "user@123");
	assert_eq!(friend.name(), Some("User #1"));
}
