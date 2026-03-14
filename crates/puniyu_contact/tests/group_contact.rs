use puniyu_contact::{contact_group, Contact, GroupContact};

#[test]
fn test_creation() {
	let group = GroupContact {
		peer: "789012",
		name: Some("Dev Team"),
	};

	assert_eq!(group.peer, "789012");
	assert_eq!(group.name, Some("Dev Team"));
}

#[test]
fn test_trait_methods() {
	let group = GroupContact {
		peer: "789012",
		name: Some("Dev Team"),
	};

	assert_eq!(group.peer(), "789012");
	assert_eq!(group.name(), Some("Dev Team"));
}

#[test]
fn test_clone_and_equality() {
	let group1 = GroupContact {
		peer: "789012",
		name: Some("Dev Team"),
	};
	let group2 = group1.clone();

	assert_eq!(group1, group2);
}

#[test]
fn test_macro() {
	let group = contact_group!("789012", "Dev Team");
	assert_eq!(group.peer(), "789012");
	assert_eq!(group.name(), Some("Dev Team"));
}
