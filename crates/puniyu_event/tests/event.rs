use puniyu_contact::Contact;
use puniyu_event::{
	EventType, SubEventType, message::MessageSubEventType, notion::NotionSubEventType,
	request::RequestSubEventType,
};
use puniyu_sender::Sender;

mod common;

#[test]
fn root_event_message_helpers_work() {
	let event = common::make_event_message();

	assert!(event.as_message().is_some());
	assert!(event.as_notion().is_none());
	assert!(event.as_request().is_none());
	assert_eq!(event.event_type(), &EventType::Message);
	assert_eq!(event.sub_event(), SubEventType::Message(MessageSubEventType::Friend));
	assert_eq!(
		common::event_snapshot(&event),
		(
			1,
			"msg-event-1".to_string(),
			"123456".to_string(),
			"123456".to_string(),
			"123456".to_string(),
		)
	);
}

#[test]
fn root_event_notion_helpers_work() {
	let event = common::make_event_notion();

	assert!(event.as_message().is_none());
	assert!(event.as_notion().is_some());
	assert!(event.as_request().is_none());
	assert_eq!(event.event_type(), &EventType::Notion);
	assert_eq!(event.sub_event(), SubEventType::Notion(NotionSubEventType::GroupRecall));
	assert_eq!(event.contact().peer(), "654321");
	assert_eq!(event.sender().user_id(), "123456");
}

#[test]
fn root_event_request_helpers_work() {
	let event = common::make_event_request();

	assert!(event.as_message().is_none());
	assert!(event.as_notion().is_none());
	assert!(event.as_request().is_some());
	assert_eq!(event.event_type(), &EventType::Request);
	assert_eq!(event.sub_event(), SubEventType::Request(RequestSubEventType::PrivateApply));
	assert_eq!(event.contact().peer(), "123456");
	assert_eq!(event.sender().user_id(), "123456");
}
