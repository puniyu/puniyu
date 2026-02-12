use puniyu_sender::{GroupSender, Role, Sender, Sex};

#[test]
fn test_group_sender_creation() {
	let sender = GroupSender {
		user_id: "123456",
		nick: Some("Alice"),
		sex: Sex::Female,
		age: Some(25),
		role: Role::Admin,
		card: Some("Group Admin"),
		level: Some(10),
		title: Some("Active Member"),
	};

	assert_eq!(sender.user_id, "123456");
	assert_eq!(sender.nick, Some("Alice"));
	assert_eq!(sender.sex, Sex::Female);
	assert_eq!(sender.age, Some(25));
	assert_eq!(sender.role, Role::Admin);
	assert_eq!(sender.card, Some("Group Admin"));
	assert_eq!(sender.level, Some(10));
	assert_eq!(sender.title, Some("Active Member"));
}

#[test]
fn test_group_sender_minimal() {
	let sender = GroupSender {
		user_id: "123456",
		nick: None,
		sex: Sex::Unknown,
		age: None,
		role: Role::Member,
		card: None,
		level: None,
		title: None,
	};

	assert_eq!(sender.user_id, "123456");
	assert_eq!(sender.nick, None);
	assert_eq!(sender.age, None);
	assert_eq!(sender.role, Role::Member);
	assert_eq!(sender.card, None);
	assert_eq!(sender.level, None);
	assert_eq!(sender.title, None);
}

#[test]
fn test_group_sender_trait_methods() {
	let sender = GroupSender {
		user_id: "123456",
		nick: Some("Alice"),
		sex: Sex::Female,
		age: Some(25),
		role: Role::Admin,
		card: Some("Group Admin"),
		level: Some(10),
		title: Some("Active Member"),
	};

	assert_eq!(sender.user_id(), "123456");
	assert_eq!(sender.name(), Some("Alice"));
	assert_eq!(sender.sex(), &Sex::Female);
	assert_eq!(sender.age(), Some(25));
}

#[test]
fn test_group_sender_specific_methods() {
	let sender = GroupSender {
		user_id: "123456",
		nick: Some("Alice"),
		sex: Sex::Female,
		age: Some(25),
		role: Role::Admin,
		card: Some("Group Admin"),
		level: Some(10),
		title: Some("Active Member"),
	};

	assert_eq!(sender.role(), &Role::Admin);
	assert_eq!(sender.card(), Some("Group Admin"));
	assert_eq!(sender.level(), Some(10));
	assert_eq!(sender.title(), Some("Active Member"));
}

#[test]
fn test_group_sender_different_roles() {
	let member = GroupSender {
		user_id: "111",
		nick: Some("Member"),
		sex: Sex::Unknown,
		age: None,
		role: Role::Member,
		card: None,
		level: None,
		title: None,
	};
	let admin = GroupSender {
		user_id: "222",
		nick: Some("Admin"),
		sex: Sex::Unknown,
		age: None,
		role: Role::Admin,
		card: None,
		level: None,
		title: None,
	};
	let owner = GroupSender {
		user_id: "333",
		nick: Some("Owner"),
		sex: Sex::Unknown,
		age: None,
		role: Role::Owner,
		card: None,
		level: None,
		title: None,
	};

	assert_eq!(member.role(), &Role::Member);
	assert_eq!(admin.role(), &Role::Admin);
	assert_eq!(owner.role(), &Role::Owner);
}

#[test]
fn test_group_sender_clone() {
	let sender1 = GroupSender {
		user_id: "123456",
		nick: Some("Alice"),
		sex: Sex::Female,
		age: Some(25),
		role: Role::Admin,
		card: Some("Group Admin"),
		level: Some(10),
		title: Some("Active Member"),
	};
	let sender2 = sender1.clone();

	assert_eq!(sender1, sender2);
}

#[test]
fn test_group_sender_debug() {
	let sender = GroupSender {
		user_id: "123456",
		nick: Some("Alice"),
		sex: Sex::Female,
		age: Some(25),
		role: Role::Admin,
		card: Some("Group Admin"),
		level: Some(10),
		title: Some("Active Member"),
	};
	let debug_str = format!("{:?}", sender);

	assert!(debug_str.contains("GroupSender"));
	assert!(debug_str.contains("123456"));
}

#[test]
fn test_group_sender_equality() {
	let sender1 = GroupSender {
		user_id: "123456",
		nick: Some("Alice"),
		sex: Sex::Female,
		age: Some(25),
		role: Role::Admin,
		card: Some("Group Admin"),
		level: Some(10),
		title: Some("Active Member"),
	};
	let sender2 = GroupSender {
		user_id: "123456",
		nick: Some("Alice"),
		sex: Sex::Female,
		age: Some(25),
		role: Role::Admin,
		card: Some("Group Admin"),
		level: Some(10),
		title: Some("Active Member"),
	};
	let sender3 = GroupSender {
		user_id: "789012",
		nick: Some("Bob"),
		sex: Sex::Male,
		age: Some(30),
		role: Role::Member,
		card: None,
		level: None,
		title: None,
	};

	assert_eq!(sender1, sender2);
	assert_ne!(sender1, sender3);
}

#[test]
fn test_group_sender_unicode_fields() {
	let sender = GroupSender {
		user_id: "123456",
		nick: Some("爱丽丝"),
		sex: Sex::Female,
		age: Some(25),
		role: Role::Admin,
		card: Some("管理员"),
		level: Some(10),
		title: Some("活跃成员"),
	};

	assert_eq!(sender.name(), Some("爱丽丝"));
	assert_eq!(sender.card(), Some("管理员"));
	assert_eq!(sender.title(), Some("活跃成员"));
}

#[test]
fn test_group_sender_special_characters() {
	let sender = GroupSender {
		user_id: "user@123",
		nick: Some("User #1"),
		sex: Sex::Unknown,
		age: None,
		role: Role::Member,
		card: Some("Card @123"),
		level: None,
		title: Some("Title #1"),
	};

	assert_eq!(sender.user_id(), "user@123");
	assert_eq!(sender.name(), Some("User #1"));
	assert_eq!(sender.card(), Some("Card @123"));
	assert_eq!(sender.title(), Some("Title #1"));
}

#[test]
fn test_group_sender_high_level() {
	let sender = GroupSender {
		user_id: "123456",
		nick: Some("HighLevel"),
		sex: Sex::Unknown,
		age: None,
		role: Role::Member,
		card: None,
		level: Some(999),
		title: None,
	};

	assert_eq!(sender.level(), Some(999));
}
