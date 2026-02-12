use puniyu_sender::{FriendSender, GroupSender, Role, Sender, SenderType, Sex};

#[test]
fn test_sender_type_from_friend() {
	let friend =
		FriendSender { user_id: "123456", nick: Some("Alice"), sex: Sex::Female, age: Some(25) };
	let sender = SenderType::from(friend);

	assert!(sender.is_friend());
	assert!(!sender.is_group());
}

#[test]
fn test_sender_type_from_group() {
	let group = GroupSender {
		user_id: "123456",
		nick: Some("Alice"),
		sex: Sex::Female,
		age: Some(25),
		role: Role::Admin,
		card: Some("Group Admin"),
		level: Some(10),
		title: Some("Active Member"),
	};
	let sender = SenderType::from(group);

	assert!(sender.is_group());
	assert!(!sender.is_friend());
}

#[test]
fn test_sender_type_as_friend() {
	let friend =
		FriendSender { user_id: "123456", nick: Some("Alice"), sex: Sex::Female, age: Some(25) };
	let sender = SenderType::Friend(friend);

	let friend_ref = sender.as_friend();
	assert!(friend_ref.is_some());
	assert_eq!(friend_ref.unwrap().user_id(), "123456");
}

#[test]
fn test_sender_type_as_friend_none() {
	let group = GroupSender {
		user_id: "123456",
		nick: Some("Alice"),
		sex: Sex::Female,
		age: Some(25),
		role: Role::Admin,
		card: None,
		level: None,
		title: None,
	};
	let sender = SenderType::Group(group);

	assert!(sender.as_friend().is_none());
}

#[test]
fn test_sender_type_as_group() {
	let group = GroupSender {
		user_id: "123456",
		nick: Some("Alice"),
		sex: Sex::Female,
		age: Some(25),
		role: Role::Admin,
		card: Some("Group Admin"),
		level: Some(10),
		title: Some("Active Member"),
	};
	let sender = SenderType::Group(group);

	let group_ref = sender.as_group();
	assert!(group_ref.is_some());
	assert_eq!(group_ref.unwrap().user_id(), "123456");
}

#[test]
fn test_sender_type_as_group_none() {
	let friend =
		FriendSender { user_id: "123456", nick: Some("Alice"), sex: Sex::Female, age: Some(25) };
	let sender = SenderType::Friend(friend);

	assert!(sender.as_group().is_none());
}

#[test]
fn test_sender_type_is_friend() {
	let friend =
		FriendSender { user_id: "123456", nick: Some("Alice"), sex: Sex::Female, age: Some(25) };
	let sender = SenderType::Friend(friend);

	assert!(sender.is_friend());
}

#[test]
fn test_sender_type_is_group() {
	let group = GroupSender {
		user_id: "123456",
		nick: Some("Alice"),
		sex: Sex::Female,
		age: Some(25),
		role: Role::Admin,
		card: None,
		level: None,
		title: None,
	};
	let sender = SenderType::Group(group);

	assert!(sender.is_group());
}

#[test]
fn test_sender_type_trait_friend() {
	let friend =
		FriendSender { user_id: "123456", nick: Some("Alice"), sex: Sex::Female, age: Some(25) };
	let sender = SenderType::Friend(friend);

	assert_eq!(sender.user_id(), "123456");
	assert_eq!(sender.name(), Some("Alice"));
	assert_eq!(sender.sex(), &Sex::Female);
	assert_eq!(sender.age(), Some(25));
}

#[test]
fn test_sender_type_trait_group() {
	let group = GroupSender {
		user_id: "789012",
		nick: Some("Bob"),
		sex: Sex::Male,
		age: Some(30),
		role: Role::Member,
		card: Some("Member Card"),
		level: Some(5),
		title: None,
	};
	let sender = SenderType::Group(group);

	assert_eq!(sender.user_id(), "789012");
	assert_eq!(sender.name(), Some("Bob"));
	assert_eq!(sender.sex(), &Sex::Male);
	assert_eq!(sender.age(), Some(30));
}

#[test]
fn test_sender_type_clone() {
	let friend =
		FriendSender { user_id: "123456", nick: Some("Alice"), sex: Sex::Female, age: Some(25) };
	let sender1 = SenderType::Friend(friend);
	let sender2 = sender1.clone();

	assert_eq!(sender1, sender2);
}

#[test]
fn test_sender_type_debug() {
	let friend =
		FriendSender { user_id: "123456", nick: Some("Alice"), sex: Sex::Female, age: Some(25) };
	let sender = SenderType::Friend(friend);
	let debug_str = format!("{:?}", sender);

	assert!(debug_str.contains("Friend"));
	assert!(debug_str.contains("123456"));
}

#[test]
fn test_sender_type_equality() {
	let friend1 =
		FriendSender { user_id: "123456", nick: Some("Alice"), sex: Sex::Female, age: Some(25) };
	let friend2 =
		FriendSender { user_id: "123456", nick: Some("Alice"), sex: Sex::Female, age: Some(25) };
	let sender1 = SenderType::Friend(friend1);
	let sender2 = SenderType::Friend(friend2);

	assert_eq!(sender1, sender2);
}

#[test]
fn test_sender_type_inequality() {
	let friend =
		FriendSender { user_id: "123456", nick: Some("Alice"), sex: Sex::Female, age: Some(25) };
	let group = GroupSender {
		user_id: "123456",
		nick: Some("Alice"),
		sex: Sex::Female,
		age: Some(25),
		role: Role::Member,
		card: None,
		level: None,
		title: None,
	};
	let sender1 = SenderType::Friend(friend);
	let sender2 = SenderType::Group(group);

	assert_ne!(sender1, sender2);
}

#[test]
fn test_sender_type_as_methods_with_data() {
	let friend =
		FriendSender { user_id: "123456", nick: Some("Alice"), sex: Sex::Female, age: Some(25) };
	let sender = SenderType::Friend(friend);

	if let Some(f) = sender.as_friend() {
		assert_eq!(f.user_id(), "123456");
		assert_eq!(f.name(), Some("Alice"));
		assert_eq!(f.sex(), &Sex::Female);
		assert_eq!(f.age(), Some(25));
	} else {
		panic!("Expected Friend sender");
	}
}

#[test]
fn test_sender_type_group_specific_fields() {
	let group = GroupSender {
		user_id: "123456",
		nick: Some("Alice"),
		sex: Sex::Female,
		age: Some(25),
		role: Role::Admin,
		card: Some("Group Admin"),
		level: Some(10),
		title: Some("Active Member"),
	};
	let sender = SenderType::Group(group);

	if let Some(g) = sender.as_group() {
		assert_eq!(g.role(), &Role::Admin);
		assert_eq!(g.card(), Some("Group Admin"));
		assert_eq!(g.level(), Some(10));
		assert_eq!(g.title(), Some("Active Member"));
	} else {
		panic!("Expected Group sender");
	}
}

#[test]
fn test_sender_type_minimal_friend() {
	let friend = FriendSender { user_id: "123456", nick: None, sex: Sex::Unknown, age: None };
	let sender = SenderType::Friend(friend);

	assert!(sender.is_friend());
	assert_eq!(sender.user_id(), "123456");
	assert_eq!(sender.name(), None);
	assert_eq!(sender.age(), None);
}

#[test]
fn test_sender_type_minimal_group() {
	let group = GroupSender {
		user_id: "123456",
		nick: None,
		sex: Sex::Unknown,
		age: None,
		role: Role::Member,
		card: None,
		level: None,
		title: None,
	};
	let sender = SenderType::Group(group);

	assert!(sender.is_group());
	assert_eq!(sender.user_id(), "123456");
	assert_eq!(sender.name(), None);
	assert_eq!(sender.age(), None);
}
