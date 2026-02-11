use puniyu_contact::{Contact, GroupContact, SceneType};

#[test]
fn test_group_contact_creation() {
	let group = GroupContact { scene: SceneType::Group, peer: "789012", name: Some("Dev Team") };

	assert_eq!(group.peer, "789012");
	assert_eq!(group.name, Some("Dev Team"));
}

#[test]
fn test_group_contact_without_name() {
	let group = GroupContact { scene: SceneType::Group, peer: "789012", name: None };

	assert_eq!(group.peer, "789012");
	assert_eq!(group.name, None);
}

#[test]
fn test_group_contact_trait_methods() {
	let group = GroupContact { scene: SceneType::Group, peer: "789012", name: Some("Dev Team") };

	assert_eq!(group.peer(), "789012");
	assert_eq!(group.name(), Some("Dev Team"));
}

#[test]
fn test_group_contact_clone() {
	let group1 = GroupContact { scene: SceneType::Group, peer: "789012", name: Some("Dev Team") };
	let group2 = group1.clone();

	assert_eq!(group1, group2);
}

#[test]
fn test_group_contact_debug() {
	let group = GroupContact { scene: SceneType::Group, peer: "789012", name: Some("Dev Team") };
	let debug_str = format!("{:?}", group);

	assert!(debug_str.contains("GroupContact"));
	assert!(debug_str.contains("789012"));
}

#[test]
fn test_group_contact_equality() {
	let group1 = GroupContact { scene: SceneType::Group, peer: "789012", name: Some("Dev Team") };
	let group2 = GroupContact { scene: SceneType::Group, peer: "789012", name: Some("Dev Team") };
	let group3 = GroupContact { scene: SceneType::Group, peer: "111222", name: Some("QA Team") };

	assert_eq!(group1, group2);
	assert_ne!(group1, group3);
}

#[test]
fn test_group_contact_empty_peer() {
	let group = GroupContact { scene: SceneType::Group, peer: "", name: Some("NoID Group") };

	assert_eq!(group.peer(), "");
	assert_eq!(group.name(), Some("NoID Group"));
}

#[test]
fn test_group_contact_unicode_name() {
	let group =
		GroupContact { scene: SceneType::Group, peer: "789012", name: Some("开发团队") };

	assert_eq!(group.name(), Some("开发团队"));
}

#[test]
fn test_group_contact_special_characters() {
	let group = GroupContact { scene: SceneType::Group, peer: "group@789", name: Some("Team #1") };

	assert_eq!(group.peer(), "group@789");
	assert_eq!(group.name(), Some("Team #1"));
}
