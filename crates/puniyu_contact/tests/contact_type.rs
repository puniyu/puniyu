use std::str::FromStr;

use puniyu_contact::{
	Contact, ContactType, FriendContact, GroupContact, GroupTempContact, GuildContact, SceneType,
	contact, contact_friend, contact_group, contact_group_temp, contact_guild,
};

#[test]
fn test_from_friend() {
	let friend = FriendContact::builder().peer("123456").name("Alice").build();
	let contact = ContactType::from(friend);

	assert!(contact.is_friend());
	assert!(!contact.is_group());
	assert!(!contact.is_group_temp());
	assert!(!contact.is_guild());
	assert_eq!(contact.peer(), "123456");
	assert_eq!(contact.name(), Some("Alice"));
}

#[test]
fn test_from_group() {
	let group = GroupContact::builder().peer("789012").name("Dev Team").build();
	let contact = ContactType::from(group);

	assert!(contact.is_group());
	assert!(!contact.is_friend());
	assert!(!contact.is_group_temp());
	assert!(!contact.is_guild());
	assert_eq!(contact.peer(), "789012");
	assert_eq!(contact.name(), Some("Dev Team"));
}

#[test]
fn test_from_group_temp() {
	let temp = GroupTempContact::builder().peer("246810").name("Temp Team").build();
	let contact = ContactType::from(temp);

	assert!(contact.is_group_temp());
	assert!(!contact.is_group());
	assert!(!contact.is_friend());
	assert!(!contact.is_guild());
	assert_eq!(contact.peer(), "246810");
	assert_eq!(contact.name(), Some("Temp Team"));
}

#[test]
fn test_from_guild() {
	let guild =
		GuildContact::builder().peer("9527").name("Guild Channel").sub_name("General").build();
	let contact = ContactType::from(guild);

	assert!(contact.is_guild());
	assert!(!contact.is_group());
	assert!(!contact.is_friend());
	assert!(!contact.is_group_temp());
	assert_eq!(contact.peer(), "9527");
	assert_eq!(contact.name(), Some("Guild Channel"));
}

#[test]
fn test_from_friend_reference() {
	let friend = FriendContact::builder().peer("123456").name("Alice").build();
	let contact = ContactType::from(&friend);

	assert!(contact.is_friend());
	assert_eq!(contact.peer(), "123456");
	assert_eq!(contact.name(), Some("Alice"));
}

#[test]
fn test_from_group_reference() {
	let group = GroupContact::builder().peer("789012").name("Dev Team").build();
	let contact = ContactType::from(&group);

	assert!(contact.is_group());
	assert_eq!(contact.peer(), "789012");
	assert_eq!(contact.name(), Some("Dev Team"));
}

#[test]
fn test_from_group_temp_reference() {
	let temp = GroupTempContact::builder().peer("246810").name("Temp Team").build();
	let contact = ContactType::from(&temp);

	assert!(contact.is_group_temp());
	assert_eq!(contact.peer(), "246810");
	assert_eq!(contact.name(), Some("Temp Team"));
}

#[test]
fn test_from_guild_reference() {
	let guild =
		GuildContact::builder().peer("9527").name("Guild Channel").sub_name("General").build();
	let contact = ContactType::from(&guild);

	assert!(contact.is_guild());
	assert_eq!(contact.peer(), "9527");
	assert_eq!(contact.name(), Some("Guild Channel"));
}

// ===== ContactType::new 通过 SceneType 构造 =====

#[test]
fn test_new_friend() {
	let contact = ContactType::new(SceneType::Friend, "123456", Some("Alice"));

	assert!(contact.is_friend());
	assert_eq!(contact.peer(), "123456");
	assert_eq!(contact.name(), Some("Alice"));
}

#[test]
fn test_new_group() {
	let contact = ContactType::new(SceneType::Group, "789012", Some("Dev Team"));

	assert!(contact.is_group());
	assert_eq!(contact.peer(), "789012");
	assert_eq!(contact.name(), Some("Dev Team"));
}

#[test]
fn test_new_group_temp() {
	let contact = ContactType::new(SceneType::GroupTemp, "246810", Some("Temp Team"));

	assert!(contact.is_group_temp());
	assert!(!contact.is_group());
	assert_eq!(contact.peer(), "246810");
	assert_eq!(contact.name(), Some("Temp Team"));
}

#[test]
fn test_new_guild() {
	let contact = ContactType::new(SceneType::Guild, "9527", Some("Guild Channel"));

	assert!(contact.is_guild());
	assert!(!contact.is_group());
	assert_eq!(contact.peer(), "9527");
	assert_eq!(contact.name(), Some("Guild Channel"));
}

#[test]
fn test_new_with_owned_strings() {
	let contact =
		ContactType::new(SceneType::Friend, String::from("123456"), Some(String::from("Alice")));

	assert!(contact.is_friend());
	assert_eq!(contact.peer(), "123456");
	assert_eq!(contact.name(), Some("Alice"));
}

// ===== as_* 访问器 =====

#[test]
fn test_as_methods() {
	let friend = FriendContact::builder().peer("u1").name("A").build();
	let contact = ContactType::Friend(friend);

	assert!(contact.as_friend().is_some());
	assert!(contact.as_group().is_none());
	assert!(contact.as_group_temp().is_none());
	assert!(contact.as_guild().is_none());
	assert_eq!(contact.as_friend().expect("friend").peer(), "u1");
}

// ===== trait 分发 =====

#[test]
fn test_trait_methods_on_enum() {
	let friend = FriendContact::builder().peer("u2").name("B").build();
	let contact = ContactType::Friend(friend);

	assert_eq!(contact.scene(), SceneType::Friend);
	assert_eq!(contact.peer(), "u2");
	assert_eq!(contact.name(), Some("B"));
}

#[test]
fn test_trait_object_equality() {
	let friend = contact_friend!(peer: "123456", name: "Alice");
	let same_friend = ContactType::new(SceneType::Friend, "123456", Some("Alice"));
	let group = contact_group!(peer: "123456", name: "Alice");

	let left: &dyn Contact<Scene = SceneType> = &friend;
	let right: &dyn Contact<Scene = SceneType> = &same_friend;
	let other: &dyn Contact<Scene = SceneType> = &group;

	assert!(left == right);
	assert!(left != other);
}

#[test]
fn test_contact_macro_flows_through_contact_type() {
	let contacts = [
		contact!(Friend, peer: "123456", name: "Alice"),
		contact!(Group, peer: "789012", name: "Dev Team"),
		contact!(GroupTemp, peer: "246810", name: "Temp Team"),
		contact!(Guild, peer: "9527", name: "Guild Channel"),
	];

	assert!(contacts[0].is_friend());
	assert_eq!(contacts[0].peer(), "123456");

	assert!(contacts[1].is_group());
	assert_eq!(contacts[1].peer(), "789012");

	assert!(contacts[2].is_group_temp());
	assert_eq!(contacts[2].peer(), "246810");

	assert!(contacts[3].is_guild());
	assert_eq!(contacts[3].peer(), "9527");
}

// ===== serde 序列化往返 =====

#[test]
fn test_serde_roundtrip_friend() {
	let json = r#"{"type":"friend","field0":{"peer":"123456","name":"Alice"}}"#;
	let contact: ContactType = serde_json::from_str(json).expect("deserialize");

	assert!(contact.is_friend());
	assert_eq!(contact.peer(), "123456");
	assert_eq!(contact.name(), Some("Alice"));

	let restored_json = serde_json::to_string(&contact).expect("serialize");
	let restored: ContactType = serde_json::from_str(&restored_json).expect("deserialize again");
	assert_eq!(contact, restored);
}

#[test]
fn test_serde_roundtrip_group() {
	let group = contact!(Group, peer: "789012", name: "Dev Team");
	let json = serde_json::to_string(&group).expect("serialize");
	assert!(json.contains("\"type\":\"group\""));

	let restored: ContactType = serde_json::from_str(&json).expect("deserialize");
	assert_eq!(group, restored);
}

#[test]
fn test_serde_roundtrip_group_temp() {
	let temp = contact!(GroupTemp, peer: "246810", name: "Temp Team");
	let json = serde_json::to_string(&temp).expect("serialize");
	assert!(json.contains("\"type\":\"grouptemp\""));

	let restored: ContactType = serde_json::from_str(&json).expect("deserialize");
	assert_eq!(temp, restored);
}

#[test]
fn test_serde_roundtrip_guild() {
	let guild = contact!(Guild, peer: "9527", name: "Guild Channel");
	let json = serde_json::to_string(&guild).expect("serialize");
	assert!(json.contains("\"type\":\"guild\""));

	let restored: ContactType = serde_json::from_str(&json).expect("deserialize");
	assert_eq!(guild, restored);
}

#[test]
fn test_serde_friend_borrowed_view() {
	let json = r#"{"type":"friend","field0":{"peer":"123456","name":"Alice"}}"#;
	let contact: ContactType = serde_json::from_str(json).expect("deserialize");

	match contact {
		ContactType::Friend(friend) => {
			assert_eq!(friend.peer(), "123456");
			assert_eq!(friend.name(), Some("Alice"));
		}
		ContactType::Group(_) | ContactType::GroupTemp(_) | ContactType::Guild(_) => {
			panic!("expected friend contact")
		}
	}
}

#[test]
fn test_owned_string_through_public_api_layers() {
	let contact = contact!(Friend, peer: String::from("friend-001"), name: String::from("Alice"));
	let json = serde_json::to_string(&contact).expect("serialize");
	let decoded: ContactType = serde_json::from_str(&json).expect("deserialize");

	assert_eq!(decoded.peer(), "friend-001");
	assert_eq!(decoded.name(), Some("Alice"));

	let borrowed_view = decoded.as_friend().expect("friend");
	assert_eq!(borrowed_view.peer(), "friend-001");
	assert_eq!(borrowed_view.name(), Some("Alice"));

	let expected = FriendContact::builder().peer("friend-001").name("Alice").build();
	assert_eq!(contact, ContactType::Friend(expected));
}

// ===== SceneType 字符串和 JSON 往返 =====

#[test]
fn test_scene_type_string_roundtrip() {
	assert_eq!(SceneType::Friend.to_string(), "friend");
	assert_eq!(SceneType::Group.to_string(), "group");
	assert_eq!(SceneType::GroupTemp.to_string(), "grouptemp");
	assert_eq!(SceneType::Guild.to_string(), "guild");

	assert_eq!(SceneType::from_str("group").expect("group"), SceneType::Group);
	assert_eq!(SceneType::from_str("grouptemp").expect("grouptemp"), SceneType::GroupTemp);
	assert_eq!(SceneType::from_str("guild").expect("guild"), SceneType::Guild);
}

#[test]
fn test_scene_type_json_roundtrip() {
	let json = serde_json::to_string(&SceneType::Friend).expect("serialize");
	assert_eq!(json, r#""friend""#);
	assert_eq!(serde_json::to_string(&SceneType::GroupTemp).expect("serialize"), r#""grouptemp""#);
	assert_eq!(serde_json::to_string(&SceneType::Guild).expect("serialize"), r#""guild""#);

	let decoded: SceneType = serde_json::from_str(&json).expect("deserialize");
	assert_eq!(decoded, SceneType::Friend);
}

// ===== 宏边界用例 =====

#[test]
fn test_friend_macro_position_and_named() {
	// 仅 peer
	let f1 = contact_friend!("123456");
	assert_eq!(f1.peer(), "123456");
	assert_eq!(f1.name(), None);

	// peer + name 位置参数
	let f2 = contact_friend!("123456", "Alice");
	assert_eq!(f2.peer(), "123456");
	assert_eq!(f2.name(), Some("Alice"));

	// 命名字段
	let f3 = contact_friend!(peer: "123456", name: "Alice");
	assert_eq!(f3.peer(), "123456");
	assert_eq!(f3.name(), Some("Alice"));

	// 仅 peer 命名字段
	let f4 = contact_friend!(peer: "123456");
	assert_eq!(f4.peer(), "123456");
	assert_eq!(f4.name(), None);
}

#[test]
fn test_group_macro_position_and_named() {
	let g1 = contact_group!("789012");
	assert_eq!(g1.peer(), "789012");
	assert_eq!(g1.name(), None);

	let g2 = contact_group!("789012", "Dev Team");
	assert_eq!(g2.peer(), "789012");
	assert_eq!(g2.name(), Some("Dev Team"));

	let g3 = contact_group!(peer: "789012", name: "Dev Team");
	assert_eq!(g3.peer(), "789012");
	assert_eq!(g3.name(), Some("Dev Team"));
}

#[test]
fn test_group_temp_macro_position_and_named() {
	let t1 = contact_group_temp!("246810");
	assert_eq!(t1.peer(), "246810");
	assert_eq!(t1.name(), None);

	let t2 = contact_group_temp!("246810", "Temp Team");
	assert_eq!(t2.peer(), "246810");
	assert_eq!(t2.name(), Some("Temp Team"));

	let t3 = contact_group_temp!(peer: "246810", name: "Temp Team");
	assert_eq!(t3.peer(), "246810");
	assert_eq!(t3.name(), Some("Temp Team"));
}

#[test]
fn test_guild_macro_position_and_named() {
	let x1 = contact_guild!("9527");
	assert_eq!(x1.peer(), "9527");
	assert_eq!(x1.name(), None);
	assert_eq!(x1.sub_name(), None);

	let x2 = contact_guild!("9527", "Guild Channel");
	assert_eq!(x2.peer(), "9527");
	assert_eq!(x2.name(), Some("Guild Channel"));
	assert_eq!(x2.sub_name(), None);

	let x3 = contact_guild!("9527", "Guild Channel", sub_name: "General");
	assert_eq!(x3.peer(), "9527");
	assert_eq!(x3.name(), Some("Guild Channel"));
	assert_eq!(x3.sub_name(), Some("General"));

	let x4 = contact_guild!(peer: "9527", name: "Guild Channel", sub_name: "General");
	assert_eq!(x4.peer(), "9527");
	assert_eq!(x4.name(), Some("Guild Channel"));
	assert_eq!(x4.sub_name(), Some("General"));
}

#[test]
fn test_macros_in_collection() {
	let friends = [contact_friend!("111", "User1"), contact_friend!("222")];
	assert_eq!(friends.len(), 2);
	assert_eq!(friends[0].peer(), "111");
	assert_eq!(friends[1].peer(), "222");

	let groups = [contact_group!("111", "Group1"), contact_group!("222")];
	assert_eq!(groups.len(), 2);
	assert_eq!(groups[0].peer(), "111");
	assert_eq!(groups[1].peer(), "222");
}
