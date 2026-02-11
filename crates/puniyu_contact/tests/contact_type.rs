use puniyu_contact::{Contact, ContactType, FriendContact, GroupContact, SceneType};

#[test]
fn test_contact_type_new_friend() {
	let contact = ContactType::new(SceneType::Friend, "123456", Some("Alice"));

	assert!(contact.is_friend());
	assert_eq!(contact.peer(), "123456");
	assert_eq!(contact.name(), Some("Alice"));
}

#[test]
fn test_contact_type_new_group() {
	let contact = ContactType::new(SceneType::Group, "789012", Some("Dev Team"));

	assert!(contact.is_group());
	assert_eq!(contact.peer(), "789012");
	assert_eq!(contact.name(), Some("Dev Team"));
}

#[test]
fn test_contact_type_new_without_name() {
	let friend = ContactType::new(SceneType::Friend, "123456", None);
	let group = ContactType::new(SceneType::Group, "789012", None);

	assert_eq!(friend.name(), None);
	assert_eq!(group.name(), None);
}

#[test]
fn test_contact_type_from_friend() {
	let friend = FriendContact { scene: SceneType::Friend, peer: "123456", name: Some("Alice") };
	let contact = ContactType::from(friend);

	assert!(contact.as_friend().is_some());
	let f = contact.as_friend().unwrap();
	assert_eq!(f.peer, "123456");
	assert_eq!(f.name, Some("Alice"));
}

#[test]
fn test_contact_type_from_group() {
	let group = GroupContact { scene: SceneType::Group, peer: "789012", name: Some("Dev Team") };
	let contact = ContactType::from(group);

	assert!(contact.as_group().is_some());
	let g = contact.as_group().unwrap();
	assert_eq!(g.peer, "789012");
	assert_eq!(g.name, Some("Dev Team"));
}

#[test]
fn test_contact_type_as_friend() {
	let friend = FriendContact { scene: SceneType::Friend, peer: "123456", name: Some("Alice") };
	let contact = ContactType::Friend(friend);

	assert!(contact.as_friend().is_some());
	assert!(contact.as_group().is_none());
}

#[test]
fn test_contact_type_as_group() {
	let group = GroupContact { scene: SceneType::Group, peer: "789012", name: Some("Dev Team") };
	let contact = ContactType::Group(group);

	assert!(contact.as_group().is_some());
	assert!(contact.as_friend().is_none());
}

#[test]
fn test_contact_type_is_friend() {
	let friend = FriendContact { scene: SceneType::Friend, peer: "123456", name: Some("Alice") };
	let contact = ContactType::Friend(friend);

	assert!(contact.is_friend());
	assert!(!contact.is_group());
}

#[test]
fn test_contact_type_is_group() {
	let group = GroupContact { scene: SceneType::Group, peer: "789012", name: Some("Dev Team") };
	let contact = ContactType::Group(group);

	assert!(contact.is_group());
	assert!(!contact.is_friend());
}

#[test]
fn test_contact_type_trait_friend() {
	let friend = FriendContact { scene: SceneType::Friend, peer: "123456", name: Some("Alice") };
	let contact = ContactType::Friend(friend);

	assert_eq!(contact.peer(), "123456");
	assert_eq!(contact.name(), Some("Alice"));
}

#[test]
fn test_contact_type_trait_group() {
	let group = GroupContact { scene: SceneType::Group, peer: "789012", name: Some("Dev Team") };
	let contact = ContactType::Group(group);

	assert_eq!(contact.peer(), "789012");
	assert_eq!(contact.name(), Some("Dev Team"));
}

#[test]
fn test_contact_type_clone() {
	let friend = FriendContact { scene: SceneType::Friend, peer: "123456", name: Some("Alice") };
	let contact1 = ContactType::Friend(friend);
	let contact2 = contact1.clone();

	assert_eq!(contact1, contact2);
}

#[test]
fn test_contact_type_debug() {
	let friend = FriendContact { scene: SceneType::Friend, peer: "123456", name: Some("Alice") };
	let contact = ContactType::Friend(friend);
	let debug_str = format!("{:?}", contact);

	assert!(debug_str.contains("Friend"));
}

#[test]
fn test_contact_type_display_friend() {
	let friend = FriendContact { scene: SceneType::Friend, peer: "123456", name: Some("Alice") };
	let contact = ContactType::Friend(friend);
	let display_str = format!("{}", contact);

	assert!(display_str.contains("friend"));
}

#[test]
fn test_contact_type_display_group() {
	let group = GroupContact { scene: SceneType::Group, peer: "789012", name: Some("Dev Team") };
	let contact = ContactType::Group(group);
	let display_str = format!("{}", contact);

	assert!(display_str.contains("group"));
}

#[test]
fn test_contact_type_equality() {
	let friend1 = FriendContact { scene: SceneType::Friend, peer: "123456", name: Some("Alice") };
	let friend2 = FriendContact { scene: SceneType::Friend, peer: "123456", name: Some("Alice") };
	let contact1 = ContactType::Friend(friend1);
	let contact2 = ContactType::Friend(friend2);

	assert_eq!(contact1, contact2);
}

#[test]
fn test_contact_type_inequality_different_types() {
	let friend = FriendContact { scene: SceneType::Friend, peer: "123456", name: Some("Alice") };
	let group = GroupContact { scene: SceneType::Group, peer: "123456", name: Some("Alice") };
	let contact1 = ContactType::Friend(friend);
	let contact2 = ContactType::Group(group);

	assert_ne!(contact1, contact2);
}

#[test]
fn test_contact_type_without_name() {
	let friend = FriendContact { scene: SceneType::Friend, peer: "123456", name: None };
	let contact = ContactType::Friend(friend);

	assert_eq!(contact.peer(), "123456");
	assert_eq!(contact.name(), None);
}

#[test]
fn test_contact_type_mixed_operations() {
	let friend = FriendContact { scene: SceneType::Friend, peer: "111", name: Some("User1") };
	let group = GroupContact { scene: SceneType::Group, peer: "222", name: Some("Group1") };

	let contacts = [ContactType::Friend(friend), ContactType::Group(group)];

	assert_eq!(contacts.len(), 2);
	assert_eq!(contacts[0].peer(), "111");
	assert_eq!(contacts[1].peer(), "222");
	assert!(contacts[0].is_friend());
	assert!(contacts[1].is_group());
}

#[test]
fn test_contact_type_as_methods_usage() {
	let contacts = vec![
		ContactType::new(SceneType::Friend, "111", Some("User1")),
		ContactType::new(SceneType::Group, "222", Some("Group1")),
	];

	for contact in &contacts {
		if let Some(friend) = contact.as_friend() {
			assert_eq!(friend.peer(), "111");
		} else if let Some(group) = contact.as_group() {
			assert_eq!(group.peer(), "222");
		}
	}
}
