//! 子事件类型综合测试

use puniyu_event::message::MessageSubEventType;
use puniyu_event::notion::NotionSubEventType;
use puniyu_event::request::RequestSubEventType;
use std::str::FromStr;

#[test]
fn test_all_message_sub_types() {
	// 测试所有消息子类型
	let types = vec![
		(MessageSubEventType::Friend, "friend"),
		(MessageSubEventType::Group, "group"),
		(MessageSubEventType::Guild, "guild"),
	];

	for (sub_type, expected_str) in types {
		assert_eq!(sub_type.to_string(), expected_str);
		assert_eq!(MessageSubEventType::from_str(expected_str).unwrap(), sub_type);
	}
}

#[test]
fn test_all_notion_sub_types() {
	// 测试所有通知子类型
	let types = vec![
		(NotionSubEventType::ReceiveLike, "receiveLike"),
		(NotionSubEventType::FriendAdd, "friendAdd"),
		(NotionSubEventType::FriendDecrease, "friendDecrease"),
		(NotionSubEventType::PrivatePoke, "privatePoke"),
		(NotionSubEventType::PrivateRecall, "privateRecall"),
		(NotionSubEventType::PrivateFileUpload, "privateFileUpload"),
		(NotionSubEventType::GroupPoke, "groupPoke"),
		(NotionSubEventType::GroupFileUpload, "groupFileUpload"),
		(NotionSubEventType::GroupCardChange, "groupCardChange"),
		(NotionSubEventType::GroupMemberAdd, "groupMemberAdd"),
		(NotionSubEventType::GroupAdminChange, "groupAdminChange"),
	];

	for (sub_type, expected_str) in types {
		assert_eq!(sub_type.to_string(), expected_str);
		assert_eq!(NotionSubEventType::from_str(expected_str).unwrap(), sub_type);
	}
}

#[test]
fn test_all_request_sub_types() {
	// 测试所有请求子类型
	let types = vec![
		(RequestSubEventType::PrivateApply, "privateApply"),
		(RequestSubEventType::GroupApply, "groupApply"),
		(RequestSubEventType::GroupInvite, "groupInvite"),
	];

	for (sub_type, expected_str) in types {
		assert_eq!(sub_type.to_string(), expected_str);
		assert_eq!(RequestSubEventType::from_str(expected_str).unwrap(), sub_type);
	}
}

#[test]
fn test_sub_type_debug() {
	// 测试 Debug 输出
	let msg_type = MessageSubEventType::Friend;
	let debug_str = format!("{:?}", msg_type);
	assert!(!debug_str.is_empty());

	let notion_type = NotionSubEventType::GroupPoke;
	let debug_str = format!("{:?}", notion_type);
	assert!(!debug_str.is_empty());

	let request_type = RequestSubEventType::PrivateApply;
	let debug_str = format!("{:?}", request_type);
	assert!(!debug_str.is_empty());
}
