use puniyu_sender::{GroupSender, Role, Sender, Sex};

#[test]
fn test_creation() {
	let sender = GroupSender {
		user_id: "123456",
		nick: Some("Alice"),
		sex: Sex::Female,
		age: Some(25),
		role: Role::Admin,
		card: Some("管理员"),
		level: Some(10),
		title: Some("活跃成员"),
	};

	assert_eq!(sender.user_id, "123456");
	assert_eq!(sender.role, Role::Admin);
	assert_eq!(sender.card, Some("管理员"));
	assert_eq!(sender.level, Some(10));
	assert_eq!(sender.title, Some("活跃成员"));
}

#[test]
fn test_trait_methods() {
	let sender = GroupSender {
		user_id: "123456",
		nick: Some("Alice"),
		sex: Sex::Female,
		age: Some(25),
		role: Role::Admin,
		card: Some("管理员"),
		level: Some(10),
		title: Some("活跃成员"),
	};

	assert_eq!(sender.user_id(), "123456");
	assert_eq!(sender.name(), Some("Alice"));
	assert_eq!(sender.role(), &Role::Admin);
	assert_eq!(sender.card(), Some("管理员"));
}

#[test]
fn test_different_roles() {
	let member = GroupSender {
		user_id: "111",
		nick: None,
		sex: Sex::Unknown,
		age: None,
		role: Role::Member,
		card: None,
		level: None,
		title: None,
	};
	let admin = GroupSender {
		user_id: "222",
		nick: None,
		sex: Sex::Unknown,
		age: None,
		role: Role::Admin,
		card: None,
		level: None,
		title: None,
	};
	let owner = GroupSender {
		user_id: "333",
		nick: None,
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
