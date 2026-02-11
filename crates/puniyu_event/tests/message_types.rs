use puniyu_event::message::MessageSubType;

#[test]
fn test_message_sub_type_display() {
	assert_eq!(MessageSubType::Friend.to_string(), "friend");
	assert_eq!(MessageSubType::Group.to_string(), "group");
	assert_eq!(MessageSubType::Guild.to_string(), "guild");
}

#[test]
fn test_message_sub_type_from_str() {
	use std::str::FromStr;

	assert_eq!(MessageSubType::from_str("friend").unwrap(), MessageSubType::Friend);
	assert_eq!(MessageSubType::from_str("group").unwrap(), MessageSubType::Group);
	assert_eq!(MessageSubType::from_str("guild").unwrap(), MessageSubType::Guild);
}

#[test]
fn test_message_sub_type_equality() {
	assert_eq!(MessageSubType::Friend, MessageSubType::Friend);
	assert_eq!(MessageSubType::Group, MessageSubType::Group);
	assert_ne!(MessageSubType::Friend, MessageSubType::Group);
}

#[test]
fn test_message_sub_type_ordering() {
	assert!(MessageSubType::Friend < MessageSubType::Group);
	assert!(MessageSubType::Group < MessageSubType::Guild);
}

#[test]
fn test_message_sub_type_clone() {
	let sub_type = MessageSubType::Friend;
	let cloned = sub_type.clone();
	assert_eq!(sub_type, cloned);
}
