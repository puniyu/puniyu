use puniyu_event::notion::NotionSubEventType;

#[test]
fn test_notion_sub_event_display() {
	assert_eq!(NotionSubEventType::ReceiveLike.to_string(), "receiveLike");
	assert_eq!(NotionSubEventType::FriendAdd.to_string(), "friendAdd");
	assert_eq!(NotionSubEventType::FriendDecrease.to_string(), "friendDecrease");
	assert_eq!(NotionSubEventType::PrivatePoke.to_string(), "privatePoke");
	assert_eq!(NotionSubEventType::PrivateRecall.to_string(), "privateRecall");
	assert_eq!(NotionSubEventType::GroupPoke.to_string(), "groupPoke");
}

#[test]
fn test_notion_sub_event_from_str() {
	use std::str::FromStr;

	assert_eq!(
		NotionSubEventType::from_str("receiveLike").unwrap(),
		NotionSubEventType::ReceiveLike
	);
	assert_eq!(NotionSubEventType::from_str("friendAdd").unwrap(), NotionSubEventType::FriendAdd);
	assert_eq!(
		NotionSubEventType::from_str("privatePoke").unwrap(),
		NotionSubEventType::PrivatePoke
	);
}

#[test]
fn test_notion_sub_event_equality() {
	assert_eq!(NotionSubEventType::ReceiveLike, NotionSubEventType::ReceiveLike);
	assert_eq!(NotionSubEventType::FriendAdd, NotionSubEventType::FriendAdd);
	assert_ne!(NotionSubEventType::ReceiveLike, NotionSubEventType::FriendAdd);
}

#[test]
fn test_notion_sub_event_clone() {
	let event = NotionSubEventType::GroupPoke;
	let cloned = event.clone();
	assert_eq!(event, cloned);
}

#[test]
fn test_notion_sub_event_group_types() {
	assert_eq!(NotionSubEventType::GroupFileUpload.to_string(), "groupFileUpload");
	assert_eq!(NotionSubEventType::GroupCardChange.to_string(), "groupCardChange");
	assert_eq!(NotionSubEventType::GroupMemberAdd.to_string(), "groupMemberAdd");
	assert_eq!(NotionSubEventType::GroupAdminChange.to_string(), "groupAdminChange");
}

#[test]
fn test_notion_sub_event_friend_types() {
	assert_eq!(NotionSubEventType::PrivateFileUpload.to_string(), "privateFileUpload");
	assert_eq!(NotionSubEventType::PrivateRecall.to_string(), "privateRecall");
	assert_eq!(NotionSubEventType::PrivatePoke.to_string(), "privatePoke");
}

#[test]
fn test_notion_sub_event_serialization() {
	use serde_json;

	let event = NotionSubEventType::GroupPoke;
	let json = serde_json::to_string(&event).unwrap();
	assert_eq!(json, r#""groupPoke""#);
}

#[test]
fn test_notion_sub_event_deserialization() {
	use serde_json;

	let json = r#""groupPoke""#;
	let event: NotionSubEventType = serde_json::from_str(json).unwrap();
	assert_eq!(event, NotionSubEventType::GroupPoke);
}
