use puniyu_event::notion::NotionSubEventType;

#[test]
fn test_notion_sub_event_display() {
	assert_eq!(NotionSubEventType::FriendAdd.to_string(), "friendAdd");
	assert_eq!(NotionSubEventType::FriendDecrease.to_string(), "friendDecrease");
	assert_eq!(NotionSubEventType::PrivateRecall.to_string(), "privateRecall");
	assert_eq!(NotionSubEventType::GroupRecall.to_string(), "groupRecall");
}

#[test]
fn test_notion_sub_event_from_str() {
	use std::str::FromStr;

	assert_eq!(NotionSubEventType::from_str("friendAdd").unwrap(), NotionSubEventType::FriendAdd);
	assert_eq!(
		NotionSubEventType::from_str("privateRecall").unwrap(),
		NotionSubEventType::PrivateRecall
	);
}

#[test]
fn test_notion_sub_event_equality() {
	assert_eq!(NotionSubEventType::FriendAdd, NotionSubEventType::FriendAdd);
	assert_ne!(NotionSubEventType::FriendAdd, NotionSubEventType::FriendDecrease);
}

#[test]
fn test_notion_sub_event_group_types() {
	assert_eq!(NotionSubEventType::GroupFileUpload.to_string(), "groupFileUpload");
	assert_eq!(NotionSubEventType::GroupMemberAdd.to_string(), "groupMemberAdd");
	assert_eq!(NotionSubEventType::GroupMemberBan.to_string(), "groupMemberBan");
	assert_eq!(NotionSubEventType::GroupWholeBan.to_string(), "groupWholeBan");
}

#[test]
fn test_notion_sub_event_friend_types() {
	assert_eq!(NotionSubEventType::PrivateFileUpload.to_string(), "privateFileUpload");
	assert_eq!(NotionSubEventType::PrivateRecall.to_string(), "privateRecall");
}

#[test]
fn test_notion_sub_event_serialization() {
	use serde_json;

	let event = NotionSubEventType::GroupRecall;
	let json = serde_json::to_string(&event).unwrap();
	assert_eq!(json, r#""groupRecall""#);
}

#[test]
fn test_notion_sub_event_deserialization() {
	use serde_json;

	let json = r#""groupRecall""#;
	let event: NotionSubEventType = serde_json::from_str(json).unwrap();
	assert_eq!(event, NotionSubEventType::GroupRecall);
}
