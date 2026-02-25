//! 事件集成测试
//!
//! 测试事件的完整功能和交互

use puniyu_event::EventType;

#[test]
fn test_event_type_conversion() {
	// 测试 EventType 的转换
	assert_eq!(EventType::Message.to_string(), "message");
	assert_eq!(EventType::Notion.to_string(), "notion");
	assert_eq!(EventType::Request.to_string(), "request");
}

#[test]
fn test_event_type_parsing() {
	use std::str::FromStr;

	// 测试从字符串解析
	let msg_type = EventType::from_str("message").unwrap();
	assert_eq!(msg_type, EventType::Message);

	let notion_type = EventType::from_str("notion").unwrap();
	assert_eq!(notion_type, EventType::Notion);

	let request_type = EventType::from_str("request").unwrap();
	assert_eq!(request_type, EventType::Request);
}

#[test]
fn test_event_type_default() {
	let default_type = EventType::default();
	assert_eq!(default_type, EventType::Unknown);
}

#[test]
fn test_event_type_comparison() {
	assert!(EventType::Message < EventType::Notion);
	assert!(EventType::Notion < EventType::Request);
	assert!(EventType::Request < EventType::Unknown);
}

#[test]
fn test_event_type_serialization() {
	use serde_json;

	let event_type = EventType::Message;
	let json = serde_json::to_string(&event_type).unwrap();
	assert_eq!(json, r#""Message""#);

	let event_type = EventType::Notion;
	let json = serde_json::to_string(&event_type).unwrap();
	assert_eq!(json, r#""Notion""#);
}

#[test]
fn test_event_type_deserialization() {
	use serde_json;

	let json = r#""Message""#;
	let event_type: EventType = serde_json::from_str(json).unwrap();
	assert_eq!(event_type, EventType::Message);

	let json = r#""Request""#;
	let event_type: EventType = serde_json::from_str(json).unwrap();
	assert_eq!(event_type, EventType::Request);
}
