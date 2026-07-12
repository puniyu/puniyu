use puniyu_sender::{
	FriendSender, GroupSender, GroupTempSender, GuildSender, Role, Sender, SenderType, Sex,
};

#[test]
fn test_from_friend() {
	let friend =
		FriendSender::builder().user_id("123456").nick("Alice").sex(Sex::Female).age(25).build();
	let sender = SenderType::from(friend);

	assert!(sender.is_friend());
	assert!(!sender.is_group());
	assert!(!sender.is_group_temp());
	assert!(!sender.is_guild());
	assert_eq!(sender.user_id(), "123456");
	assert_eq!(sender.name(), Some("Alice"));
	assert_eq!(sender.sex(), Sex::Female);
	assert_eq!(sender.age(), Some(25));
}

#[test]
fn test_from_group() {
	let group = GroupSender::builder()
		.user_id("123456")
		.nick("Alice")
		.sex(Sex::Female)
		.age(25)
		.role(Role::Admin)
		.card("管理员")
		.level(10)
		.title("活跃成员")
		.build();
	let sender = SenderType::from(group);

	assert!(sender.is_group());
	assert!(!sender.is_friend());
	assert!(!sender.is_group_temp());
	assert!(!sender.is_guild());
	assert_eq!(sender.user_id(), "123456");
}

#[test]
fn test_from_group_temp() {
	let temp = GroupTempSender::builder()
		.user_id("246810")
		.nick("Carol")
		.sex(Sex::Female)
		.age(22)
		.role(Role::Member)
		.build();
	let sender = SenderType::from(temp);

	assert!(sender.is_group_temp());
	assert!(!sender.is_group());
	assert!(!sender.is_friend());
	assert!(!sender.is_guild());
	assert_eq!(sender.user_id(), "246810");
}

#[test]
fn test_from_guild() {
	let guild = GuildSender::builder()
		.user_id("9527")
		.nick("Carol")
		.sex(Sex::Female)
		.age(22)
		.role(Role::Admin)
		.card("频道管理员")
		.level(10)
		.title("频道成员")
		.build();
	let sender = SenderType::from(guild);

	assert!(sender.is_guild());
	assert!(!sender.is_group());
	assert!(!sender.is_friend());
	assert!(!sender.is_group_temp());
	assert_eq!(sender.user_id(), "9527");
}

#[test]
fn test_as_methods() {
	let friend = FriendSender::builder().user_id("u1").nick("A").sex(Sex::Male).age(18).build();
	let sender = SenderType::Friend(friend);

	assert!(sender.as_friend().is_some());
	assert!(sender.as_group().is_none());
	assert!(sender.as_group_temp().is_none());
	assert!(sender.as_guild().is_none());
	assert_eq!(sender.as_friend().expect("friend").user_id(), "u1");
}

#[test]
fn test_trait_methods_on_enum() {
	let group = GroupSender::builder()
		.user_id("u2")
		.nick("B")
		.sex(Sex::Male)
		.age(30)
		.role(Role::Owner)
		.card("群主")
		.level(99)
		.title("元老")
		.build();
	let sender = SenderType::Group(group);

	assert_eq!(sender.user_id(), "u2");
	assert_eq!(sender.name(), Some("B"));
	assert_eq!(sender.sex(), Sex::Male);
	assert_eq!(sender.age(), Some(30));
}

#[test]
fn test_sex_predicates() {
	assert!(Sex::Male.is_male());
	assert!(!Sex::Male.is_female());
	assert!(!Sex::Male.is_unknown());

	assert!(Sex::Female.is_female());
	assert!(!Sex::Female.is_male());
	assert!(!Sex::Female.is_unknown());

	assert!(Sex::Unknown.is_unknown());
	assert!(!Sex::Unknown.is_male());
	assert!(!Sex::Unknown.is_female());
}

#[test]
fn test_role_predicates() {
	assert!(Role::Owner.is_owner());
	assert!(!Role::Owner.is_admin());
	assert!(!Role::Owner.is_member());
	assert!(!Role::Owner.is_unknown());

	assert!(Role::Admin.is_admin());
	assert!(!Role::Admin.is_owner());

	assert!(Role::Member.is_member());
	assert!(!Role::Member.is_admin());

	assert!(Role::Unknown.is_unknown());
	assert!(!Role::Unknown.is_member());
}

#[test]
fn test_sender_type_serde_roundtrip() {
	let json =
		r#"{"type":"friend","field0":{"user_id":"123456","nick":"Alice","sex":"female","age":25}}"#;
	let sender: SenderType = serde_json::from_str(json).expect("deserialize");

	assert!(sender.is_friend());
	assert_eq!(sender.user_id(), "123456");
	assert_eq!(sender.name(), Some("Alice"));

	let restored_json = serde_json::to_string(&sender).expect("serialize");
	let restored: SenderType = serde_json::from_str(&restored_json).expect("deserialize again");
	assert_eq!(sender, restored);
}

#[test]
fn test_sender_type_group_serde_roundtrip() {
	let group = GroupSender::builder()
		.user_id("g1")
		.nick("G")
		.sex(Sex::Female)
		.age(20)
		.role(Role::Member)
		.card("名片")
		.level(5)
		.title("头衔")
		.build();
	let sender = SenderType::Group(group);

	let json = serde_json::to_string(&sender).expect("serialize");
	assert!(json.contains("\"type\":\"group\""));

	let restored: SenderType = serde_json::from_str(&json).expect("deserialize");
	assert_eq!(sender, restored);
}

#[test]
fn test_sender_type_guild_serde_roundtrip() {
	let guild = GuildSender::builder()
		.user_id("gu1")
		.nick("Gu")
		.sex(Sex::Male)
		.age(28)
		.role(Role::Admin)
		.card("频道名片")
		.level(8)
		.title("频道头衔")
		.build();
	let sender = SenderType::Guild(guild);

	let json = serde_json::to_string(&sender).expect("serialize");
	assert!(json.contains("\"type\":\"guild\""));

	let restored: SenderType = serde_json::from_str(&json).expect("deserialize");
	assert_eq!(sender, restored);
}

#[test]
fn test_sender_type_group_temp_serde_roundtrip() {
	let temp = GroupTempSender::builder().user_id("gt1").nick("Gt").role(Role::Member).build();
	let sender = SenderType::GroupTemp(temp);

	let json = serde_json::to_string(&sender).expect("serialize");
	assert!(json.contains("\"type\":\"grouptemp\""));

	let restored: SenderType = serde_json::from_str(&json).expect("deserialize");
	assert_eq!(sender, restored);
}
