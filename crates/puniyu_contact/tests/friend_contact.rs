use puniyu_contact::{contact_friend, Contact, FriendContact};

#[test]
fn test_creation() {
	let friend = FriendContact {
		peer: "123456",
		name: Some("Alice"),
	};

	assert_eq!(friend.peer, "123456");
	assert_eq!(friend.name, Some("Alice"));
}

#[test]
fn test_trait_methods() {
	let friend = FriendContact {
		peer: "123456",
		name: Some("Alice"),
	};

	assert_eq!(friend.peer(), "123456");
	assert_eq!(friend.name(), Some("Alice"));
}

#[test]
fn test_clone_and_equality() {
	let friend1 = FriendContact {
		peer: "123456",
		name: Some("Alice"),
	};
	let friend2 = friend1.clone();

	assert_eq!(friend1, friend2);
}

#[test]
fn test_macro() {
	let friend = contact_friend!("123456", "Alice");
	assert_eq!(friend.peer(), "123456");
	assert_eq!(friend.name(), Some("Alice"));
}
