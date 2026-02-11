use puniyu_event::{EventType, Permission};

#[test]
fn test_event_type_display() {
	assert_eq!(EventType::Message.to_string(), "message");
	assert_eq!(EventType::Notion.to_string(), "notion");
	assert_eq!(EventType::Request.to_string(), "request");
	assert_eq!(EventType::Unknown.to_string(), "unknown");
}

#[test]
fn test_event_type_default() {
	let default_type = EventType::default();
	assert_eq!(default_type, EventType::Unknown);
}

#[test]
fn test_event_type_from_str() {
	use std::str::FromStr;

	assert_eq!(EventType::from_str("message").unwrap(), EventType::Message);
	assert_eq!(EventType::from_str("notion").unwrap(), EventType::Notion);
	assert_eq!(EventType::from_str("request").unwrap(), EventType::Request);
}

#[test]
fn test_permission_default() {
	let default_perm = Permission::default();
	assert_eq!(default_perm, Permission::All);
}

#[test]
fn test_permission_display() {
	assert_eq!(Permission::All.to_string(), "all");
	assert_eq!(Permission::Master.to_string(), "master");
}

#[test]
fn test_permission_from_str() {
	use std::str::FromStr;

	assert_eq!(Permission::from_str("master").unwrap(), Permission::Master);
	assert_eq!(Permission::from_str("Master").unwrap(), Permission::Master);
	assert_eq!(Permission::from_str("all").unwrap(), Permission::All);
	assert_eq!(Permission::from_str("anything").unwrap(), Permission::All);
}

#[test]
fn test_permission_equality() {
	assert_eq!(Permission::All, Permission::All);
	assert_eq!(Permission::Master, Permission::Master);
	assert_ne!(Permission::All, Permission::Master);
}
