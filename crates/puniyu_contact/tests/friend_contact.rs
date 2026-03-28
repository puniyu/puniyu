use puniyu_contact::{Contact, FriendContact, contact_friend};

#[test]
fn test_creation() {
	let friend = FriendContact { peer: "123456".into(), name: Some("Alice".into()) };

	assert_eq!(friend.peer, "123456");
	assert_eq!(friend.name.as_deref(), Some("Alice"));
}

#[test]
fn test_trait_methods() {
	let friend = FriendContact { peer: "123456".into(), name: Some("Alice".into()) };

	assert_eq!(friend.peer(), "123456");
	assert_eq!(friend.name(), Some("Alice"));
}

#[test]
fn test_clone_and_equality() {
	let friend1 = FriendContact { peer: "123456".into(), name: Some("Alice".into()) };
	let friend2 = friend1.clone();

	assert_eq!(friend1, friend2);
}

#[test]
fn test_macro() {
	let friend = contact_friend!("123456", "Alice");
	assert_eq!(friend.peer(), "123456");
	assert_eq!(friend.name(), Some("Alice"));
}

#[test]
fn test_macro_with_owned_strings() {
	let friend = contact_friend!(String::from("123456"), String::from("Alice"));

	assert_eq!(friend.peer(), "123456");
	assert_eq!(friend.name(), Some("Alice"));
}
