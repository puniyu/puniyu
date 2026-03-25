use puniyu_event::request::RequestSubEventType;

#[test]
fn test_request_sub_event_display() {
	assert_eq!(RequestSubEventType::PrivateApply.to_string(), "privateApply");
	assert_eq!(RequestSubEventType::GroupApply.to_string(), "groupApply");
	assert_eq!(RequestSubEventType::GroupInvite.to_string(), "groupInvite");
}

#[test]
fn test_request_sub_event_from_str() {
	use std::str::FromStr;

	assert_eq!(
		RequestSubEventType::from_str("privateApply").unwrap(),
		RequestSubEventType::PrivateApply
	);
	assert_eq!(
		RequestSubEventType::from_str("groupApply").unwrap(),
		RequestSubEventType::GroupApply
	);
	assert_eq!(
		RequestSubEventType::from_str("groupInvite").unwrap(),
		RequestSubEventType::GroupInvite
	);
}

#[test]
fn test_request_sub_event_equality() {
	assert_eq!(RequestSubEventType::PrivateApply, RequestSubEventType::PrivateApply);
	assert_eq!(RequestSubEventType::GroupApply, RequestSubEventType::GroupApply);
	assert_ne!(RequestSubEventType::PrivateApply, RequestSubEventType::GroupApply);
}

#[test]
fn test_request_sub_event_ordering() {
	assert!(RequestSubEventType::PrivateApply < RequestSubEventType::GroupApply);
	assert!(RequestSubEventType::GroupApply < RequestSubEventType::GroupInvite);
}

#[test]
fn test_request_sub_event_clone() {
	let event = RequestSubEventType::PrivateApply;
	let cloned = event.clone();
	assert_eq!(event, cloned);
}

#[test]
fn test_request_sub_event_serialization() {
	use serde_json;

	let event = RequestSubEventType::PrivateApply;
	let json = serde_json::to_string(&event).unwrap();
	assert_eq!(json, r#""privateApply""#);

	let event = RequestSubEventType::GroupInvite;
	let json = serde_json::to_string(&event).unwrap();
	assert_eq!(json, r#""groupInvite""#);
}

#[test]
fn test_request_sub_event_deserialization() {
	use serde_json;

	let json = r#""privateApply""#;
	let event: RequestSubEventType = serde_json::from_str(json).unwrap();
	assert_eq!(event, RequestSubEventType::PrivateApply);

	let json = r#""groupApply""#;
	let event: RequestSubEventType = serde_json::from_str(json).unwrap();
	assert_eq!(event, RequestSubEventType::GroupApply);
}
