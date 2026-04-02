use puniyu_event::{
	EventType,
	message::MessageSubEventType,
	notion::{ContentType as NotionContentType, NotionSubEventType},
	request::{ContentType as RequestContentType, RequestSubEventType},
};
mod common;

#[test]
fn message_event_implements_event_and_message_traits() {
	let event = common::make_message_event();

	assert_eq!(event.event_type(), &EventType::Message);
	assert_eq!(event.sub_event(), &MessageSubEventType::Friend);
	assert_eq!(
		common::base_snapshot(&event),
		(
			1,
			"msg-event-1".to_string(),
			"123456".to_string(),
			"123456".to_string(),
			"123456".to_string(),
		)
	);
	assert_eq!(common::message_summary(&event), ("msg-1", 0));
}

#[test]
fn notion_event_implements_event_and_notion_traits() {
	let event = common::make_notion_event();

	assert_eq!(event.event_type(), &EventType::Notion);
	assert_eq!(event.sub_event(), &NotionSubEventType::GroupRecall);
	assert_eq!(
		common::base_snapshot(&event),
		(
			2,
			"notion-event-1".to_string(),
			"123456".to_string(),
			"654321".to_string(),
			"123456".to_string(),
		)
	);

	let (notion, content) = common::notion_summary(&event);
	assert_eq!(notion, "收到群聊撤回事件");
	match content {
		NotionContentType::GroupRecall(content) => assert_eq!(content.message_id, "msg-1"),
		_ => panic!("expected group recall content"),
	}
}

#[test]
fn request_event_implements_event_and_request_traits() {
	let event = common::make_request_event();

	assert_eq!(event.event_type(), &EventType::Request);
	assert_eq!(event.sub_event(), &RequestSubEventType::PrivateApply);
	assert_eq!(
		common::base_snapshot(&event),
		(
			3,
			"request-event-1".to_string(),
			"123456".to_string(),
			"123456".to_string(),
			"123456".to_string(),
		)
	);

	let (request, content) = common::request_summary(&event);
	assert_eq!(request, "收到好友申请请求");
	match content {
		RequestContentType::PrivateApply(content) => assert_eq!(content.message, "hello"),
		_ => panic!("expected private apply content"),
	}
}
