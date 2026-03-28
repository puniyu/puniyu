use puniyu_sender::{FriendSender, GroupSender, Role, Sender, SenderType, Sex};

#[test]
fn test_from_friend() {
	let friend = FriendSender {
		user_id: "123456".into(),
		nick: Some("Alice".into()),
		sex: Sex::Female,
		age: Some(25),
	};
	let sender = SenderType::from(friend);

	assert!(sender.is_friend());
	assert!(!sender.is_group());
	assert_eq!(sender.user_id(), "123456");
}

#[test]
fn test_from_group() {
	let group = GroupSender {
		user_id: "123456".into(),
		nick: Some("Alice".into()),
		sex: Sex::Female,
		age: Some(25),
		role: Role::Admin,
		card: Some("管理员".into()),
		level: Some(10),
		title: Some("活跃成员".into()),
	};
	let sender = SenderType::from(group);

	assert!(sender.is_group());
	assert!(!sender.is_friend());
	assert_eq!(sender.user_id(), "123456");
}

#[test]
fn test_as_friend() {
	let friend = FriendSender {
		user_id: "123456".into(),
		nick: Some("Alice".into()),
		sex: Sex::Female,
		age: Some(25),
	};
	let sender = SenderType::Friend(friend);

	assert!(sender.as_friend().is_some());
	assert!(sender.as_group().is_none());
	assert_eq!(sender.as_friend().unwrap().user_id(), "123456");
}

#[test]
fn test_as_group() {
	let group = GroupSender {
		user_id: "123456".into(),
		nick: Some("Alice".into()),
		sex: Sex::Female,
		age: Some(25),
		role: Role::Admin,
		card: Some("管理员".into()),
		level: Some(10),
		title: Some("活跃成员".into()),
	};
	let sender = SenderType::Group(group);

	assert!(sender.as_group().is_some());
	assert!(sender.as_friend().is_none());

	let g = sender.as_group().unwrap();
	assert_eq!(g.user_id(), "123456");
	assert_eq!(g.role(), &Role::Admin);
	assert_eq!(g.card(), Some("管理员"));
}

#[test]
fn test_trait_methods() {
	let friend = FriendSender {
		user_id: "123456".into(),
		nick: Some("Alice".into()),
		sex: Sex::Female,
		age: Some(25),
	};
	let sender = SenderType::Friend(friend);

	assert_eq!(sender.user_id(), "123456");
	assert_eq!(sender.name(), Some("Alice"));
	assert_eq!(sender.sex(), &Sex::Female);
	assert_eq!(sender.age(), Some(25));
}
