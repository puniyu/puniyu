use puniyu_contact::{Contact, ContactType, FriendContact, GroupContact, GroupTempContact, SceneType};

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
fn test_new_with_owned_strings() {
	let contact =
		ContactType::new(SceneType::Friend, String::from("123456"), Some(String::from("Alice")));

	assert!(contact.is_friend());
	assert_eq!(contact.peer(), "123456");
	assert_eq!(contact.name(), Some("Alice"));
}

#[test]
fn test_from_friend() {
	let friend = FriendContact { peer: "123456".into(), name: Some("Alice".into()) };
	let contact = ContactType::from(friend);

	assert!(contact.is_friend());
	assert!(contact.as_friend().is_some());
	assert_eq!(contact.peer(), "123456");
}

#[test]
fn test_from_group() {
	let group = GroupContact { peer: "789012".into(), name: Some("Dev Team".into()) };
	let contact = ContactType::from(group);

	assert!(contact.is_group());
	assert!(contact.as_group().is_some());
	assert_eq!(contact.peer(), "789012");
}

#[test]
fn test_as_friend() {
	let friend = FriendContact { peer: "123456".into(), name: Some("Alice".into()) };
	let contact = ContactType::Friend(friend);

	assert!(contact.as_friend().is_some());
	assert!(contact.as_group().is_none());
}

#[test]
fn test_as_group() {
	let group = GroupContact { peer: "789012".into(), name: Some("Dev Team".into()) };
	let contact = ContactType::Group(group);

	assert!(contact.as_group().is_some());
	assert!(contact.as_friend().is_none());
	assert!(contact.as_group_temp().is_none());
}

#[test]
fn test_as_group_temp() {
	let group = GroupTempContact { peer: "246810".into(), name: Some("Temp Team".into()) };
	let contact = ContactType::GroupTemp(group);

	assert!(contact.as_group_temp().is_some());
	assert!(contact.as_group().is_none());
	assert!(contact.as_friend().is_none());
}

#[test]
fn test_trait_methods() {
	let friend = FriendContact { peer: "123456".into(), name: Some("Alice".into()) };
	let contact = ContactType::Friend(friend);

	assert_eq!(contact.peer(), "123456");
	assert_eq!(contact.name(), Some("Alice"));
}
