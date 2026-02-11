use puniyu_sender::{Role, Sender, Sex, sender_friend, sender_group};

#[test]
fn test_sender_friend_macro_full() {
	let sender = sender_friend!(
		user_id: "123456",
		nick: "Alice",
		sex: Sex::Female,
		age: 25u32,
	);

	assert_eq!(sender.user_id(), "123456");
	assert_eq!(sender.name(), Some("Alice"));
	assert_eq!(sender.sex(), &Sex::Female);
	assert_eq!(sender.age(), Some(25));
}

#[test]
fn test_sender_friend_macro_minimal() {
	let sender = sender_friend!(user_id: "123456");

	assert_eq!(sender.user_id(), "123456");
	assert_eq!(sender.name(), None);
	assert_eq!(sender.age(), None);
}

#[test]
fn test_sender_friend_macro_single_arg() {
	let sender = sender_friend!("123456");

	assert_eq!(sender.user_id(), "123456");
	assert_eq!(sender.name(), None);
	assert_eq!(sender.age(), None);
}

#[test]
fn test_sender_friend_macro_partial() {
	let sender = sender_friend!(
		user_id: "123456",
		nick: "Alice",
	);

	assert_eq!(sender.user_id(), "123456");
	assert_eq!(sender.name(), Some("Alice"));
	assert_eq!(sender.age(), None);
}

#[test]
fn test_sender_friend_macro_with_sex() {
	let sender = sender_friend!(
		user_id: "123456",
		sex: Sex::Male,
	);

	assert_eq!(sender.user_id(), "123456");
	assert_eq!(sender.sex(), &Sex::Male);
}

#[test]
fn test_sender_friend_macro_with_age() {
	let sender = sender_friend!(
		user_id: "123456",
		age: 30u32,
	);

	assert_eq!(sender.user_id(), "123456");
	assert_eq!(sender.age(), Some(30));
}

#[test]
fn test_sender_friend_macro_unicode() {
	let sender = sender_friend!(
		user_id: "123456",
		nick: "爱丽丝",
	);

	assert_eq!(sender.name(), Some("爱丽丝"));
}

#[test]
fn test_sender_group_macro_full() {
	let sender = sender_group!(
		user_id: "123456",
		nick: "Alice",
		sex: Sex::Female,
		age: 25u32,
		role: Role::Admin,
		card: "Group Admin",
		level: 10u32,
		title: "Active Member",
	);

	assert_eq!(sender.user_id(), "123456");
	assert_eq!(sender.name(), Some("Alice"));
	assert_eq!(sender.sex(), &Sex::Female);
	assert_eq!(sender.age(), Some(25));
	assert_eq!(sender.role(), &Role::Admin);
	assert_eq!(sender.card(), Some("Group Admin"));
	assert_eq!(sender.level(), Some(10));
	assert_eq!(sender.title(), Some("Active Member"));
}

#[test]
fn test_sender_group_macro_minimal() {
	let sender = sender_group!(user_id: "123456");

	assert_eq!(sender.user_id(), "123456");
	assert_eq!(sender.name(), None);
	assert_eq!(sender.age(), None);
	assert_eq!(sender.card(), None);
	assert_eq!(sender.level(), None);
	assert_eq!(sender.title(), None);
}

#[test]
fn test_sender_group_macro_single_arg() {
	let sender = sender_group!("123456");

	assert_eq!(sender.user_id(), "123456");
	assert_eq!(sender.name(), None);
	assert_eq!(sender.age(), None);
	assert_eq!(sender.card(), None);
	assert_eq!(sender.level(), None);
	assert_eq!(sender.title(), None);
}

#[test]
fn test_sender_group_macro_with_role() {
	let sender = sender_group!(
		user_id: "123456",
		role: Role::Owner,
	);

	assert_eq!(sender.user_id(), "123456");
	assert_eq!(sender.role(), &Role::Owner);
}

#[test]
fn test_sender_group_macro_with_card() {
	let sender = sender_group!(
		user_id: "123456",
		card: "Member Card",
	);

	assert_eq!(sender.user_id(), "123456");
	assert_eq!(sender.card(), Some("Member Card"));
}

#[test]
fn test_sender_group_macro_with_level() {
	let sender = sender_group!(
		user_id: "123456",
		level: 99u32,
	);

	assert_eq!(sender.user_id(), "123456");
	assert_eq!(sender.level(), Some(99));
}

#[test]
fn test_sender_group_macro_with_title() {
	let sender = sender_group!(
		user_id: "123456",
		title: "VIP Member",
	);

	assert_eq!(sender.user_id(), "123456");
	assert_eq!(sender.title(), Some("VIP Member"));
}

#[test]
fn test_sender_group_macro_partial() {
	let sender = sender_group!(
		user_id: "123456",
		nick: "Alice",
		role: Role::Admin,
	);

	assert_eq!(sender.user_id(), "123456");
	assert_eq!(sender.name(), Some("Alice"));
	assert_eq!(sender.role(), &Role::Admin);
	assert_eq!(sender.card(), None);
	assert_eq!(sender.level(), None);
	assert_eq!(sender.title(), None);
}

#[test]
fn test_sender_group_macro_unicode() {
	let sender = sender_group!(
		user_id: "123456",
		nick: "爱丽丝",
		card: "管理员",
		title: "活跃成员",
	);

	assert_eq!(sender.name(), Some("爱丽丝"));
	assert_eq!(sender.card(), Some("管理员"));
	assert_eq!(sender.title(), Some("活跃成员"));
}

#[test]
fn test_sender_group_macro_different_roles() {
	let member = sender_group!(
		user_id: "111",
		role: Role::Member,
	);
	let admin = sender_group!(
		user_id: "222",
		role: Role::Admin,
	);
	let owner = sender_group!(
		user_id: "333",
		role: Role::Owner,
	);

	assert_eq!(member.role(), &Role::Member);
	assert_eq!(admin.role(), &Role::Admin);
	assert_eq!(owner.role(), &Role::Owner);
}

#[test]
fn test_sender_friend_macro_trailing_comma() {
	let sender = sender_friend!(
		user_id: "123456",
		nick: "Alice",
	);

	assert_eq!(sender.user_id(), "123456");
	assert_eq!(sender.name(), Some("Alice"));
}

#[test]
fn test_sender_group_macro_trailing_comma() {
	let sender = sender_group!(
		user_id: "123456",
		role: Role::Admin,
	);

	assert_eq!(sender.user_id(), "123456");
	assert_eq!(sender.role(), &Role::Admin);
}
