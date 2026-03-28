use puniyu_contact::{Contact, GroupContact, contact_group};

#[test]
fn test_creation() {
	let group = GroupContact { peer: "789012".into(), name: Some("Dev Team".into()) };

	assert_eq!(group.peer, "789012");
	assert_eq!(group.name.as_deref(), Some("Dev Team"));
}

#[test]
fn test_trait_methods() {
	let group = GroupContact { peer: "789012".into(), name: Some("Dev Team".into()) };

	assert_eq!(group.peer(), "789012");
	assert_eq!(group.name(), Some("Dev Team"));
}

#[test]
fn test_clone_and_equality() {
	let group1 = GroupContact { peer: "789012".into(), name: Some("Dev Team".into()) };
	let group2 = group1.clone();

	assert_eq!(group1, group2);
}

#[test]
fn test_macro() {
	let group = contact_group!("789012", "Dev Team");
	assert_eq!(group.peer(), "789012");
	assert_eq!(group.name(), Some("Dev Team"));
}

#[test]
fn test_macro_with_owned_strings() {
	let group = contact_group!(String::from("789012"), String::from("Dev Team"));

	assert_eq!(group.peer(), "789012");
	assert_eq!(group.name(), Some("Dev Team"));
}
