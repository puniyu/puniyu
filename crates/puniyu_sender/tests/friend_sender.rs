use puniyu_sender::{FriendSender, Sender, Sex};

#[test]
fn test_friend_sender_creation() {
	let sender =
		FriendSender { user_id: "123456", nick: Some("Alice"), sex: Sex::Female, age: Some(25) };

	assert_eq!(sender.user_id, "123456");
	assert_eq!(sender.nick, Some("Alice"));
	assert_eq!(sender.sex, Sex::Female);
	assert_eq!(sender.age, Some(25));
}

#[test]
fn test_friend_sender_minimal() {
	let sender = FriendSender { user_id: "123456", nick: None, sex: Sex::Unknown, age: None };

	assert_eq!(sender.user_id, "123456");
	assert_eq!(sender.nick, None);
	assert_eq!(sender.age, None);
}

#[test]
fn test_friend_sender_trait_methods() {
	let sender =
		FriendSender { user_id: "123456", nick: Some("Alice"), sex: Sex::Female, age: Some(25) };

	assert_eq!(sender.user_id(), "123456");
	assert_eq!(sender.name(), Some("Alice"));
	assert_eq!(sender.sex(), &Sex::Female);
	assert_eq!(sender.age(), Some(25));
}

#[test]
fn test_friend_sender_clone() {
	let sender1 =
		FriendSender { user_id: "123456", nick: Some("Alice"), sex: Sex::Female, age: Some(25) };
	let sender2 = sender1.clone();

	assert_eq!(sender1, sender2);
}

#[test]
fn test_friend_sender_debug() {
	let sender =
		FriendSender { user_id: "123456", nick: Some("Alice"), sex: Sex::Female, age: Some(25) };
	let debug_str = format!("{:?}", sender);

	assert!(debug_str.contains("FriendSender"));
	assert!(debug_str.contains("123456"));
}

#[test]
fn test_friend_sender_equality() {
	let sender1 =
		FriendSender { user_id: "123456", nick: Some("Alice"), sex: Sex::Female, age: Some(25) };
	let sender2 =
		FriendSender { user_id: "123456", nick: Some("Alice"), sex: Sex::Female, age: Some(25) };
	let sender3 =
		FriendSender { user_id: "789012", nick: Some("Bob"), sex: Sex::Male, age: Some(30) };

	assert_eq!(sender1, sender2);
	assert_ne!(sender1, sender3);
}

#[test]
fn test_friend_sender_different_sex() {
	let male = FriendSender { user_id: "123456", nick: Some("Bob"), sex: Sex::Male, age: Some(30) };
	let female =
		FriendSender { user_id: "789012", nick: Some("Alice"), sex: Sex::Female, age: Some(25) };
	let unknown =
		FriendSender { user_id: "111222", nick: Some("Unknown"), sex: Sex::Unknown, age: None };

	assert_eq!(male.sex(), &Sex::Male);
	assert_eq!(female.sex(), &Sex::Female);
	assert_eq!(unknown.sex(), &Sex::Unknown);
}

#[test]
fn test_friend_sender_unicode_nick() {
	let sender =
		FriendSender {
			user_id: "123456", nick: Some("爱丽丝"), sex: Sex::Female, age: Some(25)
		};

	assert_eq!(sender.name(), Some("爱丽丝"));
}

#[test]
fn test_friend_sender_special_characters() {
	let sender =
		FriendSender { user_id: "user@123", nick: Some("User #1"), sex: Sex::Unknown, age: None };

	assert_eq!(sender.user_id(), "user@123");
	assert_eq!(sender.name(), Some("User #1"));
}
