use puniyu_event::notion::NotionSubEvent;

#[test]
fn test_notion_sub_event_display() {
	assert_eq!(NotionSubEvent::ReceiveLike.to_string(), "receiveLike");
	assert_eq!(NotionSubEvent::FriendAdd.to_string(), "friendAdd");
	assert_eq!(NotionSubEvent::FriendDecrease.to_string(), "friendDecrease");
	assert_eq!(NotionSubEvent::PrivatePoke.to_string(), "privatePoke");
	assert_eq!(NotionSubEvent::PrivateRecall.to_string(), "privateRecall");
	assert_eq!(NotionSubEvent::GroupPoke.to_string(), "groupPoke");
}

#[test]
fn test_notion_sub_event_from_str() {
	use std::str::FromStr;

	assert_eq!(NotionSubEvent::from_str("receiveLike").unwrap(), NotionSubEvent::ReceiveLike);
	assert_eq!(NotionSubEvent::from_str("friendAdd").unwrap(), NotionSubEvent::FriendAdd);
	assert_eq!(NotionSubEvent::from_str("privatePoke").unwrap(), NotionSubEvent::PrivatePoke);
}

#[test]
fn test_notion_sub_event_equality() {
	assert_eq!(NotionSubEvent::ReceiveLike, NotionSubEvent::ReceiveLike);
	assert_eq!(NotionSubEvent::FriendAdd, NotionSubEvent::FriendAdd);
	assert_ne!(NotionSubEvent::ReceiveLike, NotionSubEvent::FriendAdd);
}

#[test]
fn test_notion_sub_event_clone() {
	let event = NotionSubEvent::GroupPoke;
	let cloned = event.clone();
	assert_eq!(event, cloned);
}

#[test]
fn test_notion_sub_event_group_types() {
	assert_eq!(NotionSubEvent::GroupFileUpload.to_string(), "groupFileUpload");
	assert_eq!(NotionSubEvent::GroupCardChange.to_string(), "groupCardChange");
	assert_eq!(NotionSubEvent::GroupMemberAdd.to_string(), "groupMemberAdd");
	assert_eq!(NotionSubEvent::GroupAdminChange.to_string(), "groupAdminChange");
}
