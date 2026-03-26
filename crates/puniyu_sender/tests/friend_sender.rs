use puniyu_sender::{FriendSender, Sender, Sex};

#[test]
fn test_creation() {
	let sender =
		FriendSender { user_id: "123456", nick: Some("Alice"), sex: Sex::Female, age: Some(25) };

	assert_eq!(sender.user_id, "123456");
	assert_eq!(sender.nick, Some("Alice"));
	assert_eq!(sender.sex, Sex::Female);
	assert_eq!(sender.age, Some(25));
}

#[test]
fn test_trait_methods() {
	let sender =
		FriendSender { user_id: "123456", nick: Some("Alice"), sex: Sex::Female, age: Some(25) };

	assert_eq!(sender.user_id(), "123456");
	assert_eq!(sender.name(), Some("Alice"));
	assert_eq!(sender.sex(), &Sex::Female);
	assert_eq!(sender.age(), Some(25));
}

#[test]
fn test_clone_and_equality() {
	let sender1 =
		FriendSender { user_id: "123456", nick: Some("Alice"), sex: Sex::Female, age: Some(25) };
	let sender2 = sender1.clone();

	assert_eq!(sender1, sender2);
}
