use puniyu_sender::{GroupSender, Role, Sender, Sex};

#[test]
fn test_creation() {
	let sender = GroupSender {
		user_id: "123456".into(),
		nick: Some("Alice".into()),
		sex: Sex::Female,
		age: Some(25),
		role: Role::Admin,
		card: Some("管理员".into()),
		level: Some(10),
		title: Some("活跃成员".into()),
	};

	assert_eq!(sender.user_id, "123456");
	assert_eq!(sender.role, Role::Admin);
	assert_eq!(sender.card.as_deref(), Some("管理员"));
	assert_eq!(sender.level, Some(10));
	assert_eq!(sender.title.as_deref(), Some("活跃成员"));
}

#[test]
fn test_trait_methods() {
	let sender = GroupSender {
		user_id: "123456".into(),
		nick: Some("Alice".into()),
		sex: Sex::Female,
		age: Some(25),
		role: Role::Admin,
		card: Some("管理员".into()),
		level: Some(10),
		title: Some("活跃成员".into()),
	};

	assert_eq!(sender.user_id(), "123456");
	assert_eq!(sender.name(), Some("Alice"));
	assert_eq!(sender.role(), &Role::Admin);
	assert_eq!(sender.card(), Some("管理员"));
}

#[test]
fn test_different_roles() {
	let member = GroupSender {
		user_id: "111".into(),
		nick: None,
		sex: Sex::Unknown,
		age: None,
		role: Role::Member,
		card: None,
		level: None,
		title: None,
	};
	let admin = GroupSender {
		user_id: "222".into(),
		nick: None,
		sex: Sex::Unknown,
		age: None,
		role: Role::Admin,
		card: None,
		level: None,
		title: None,
	};
	let owner = GroupSender {
		user_id: "333".into(),
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

#[test]
fn test_owned_string_creation() {
	let sender = GroupSender {
		user_id: String::from("123456").into(),
		nick: Some(String::from("Alice").into()),
		sex: Sex::Female,
		age: Some(25),
		role: Role::Admin,
		card: Some(String::from("管理员").into()),
		level: Some(10),
		title: Some(String::from("活跃成员").into()),
	};

	assert_eq!(sender.user_id(), "123456");
	assert_eq!(sender.name(), Some("Alice"));
	assert_eq!(sender.card(), Some("管理员"));
	assert_eq!(sender.title(), Some("活跃成员"));
}
