use puniyu_event::{EventType, SubEventType, message::MessageSubEventType};

mod common;

#[test]
fn root_event_message_helpers_work() {
	let event = common::make_event_message();

	assert!(event.as_message().is_some());
	assert_eq!(event.event_type(), EventType::Message);
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
fn root_event_extension_helpers_work() {
	let event = common::make_event_extension();

	assert!(event.as_message().is_none());
	assert!(event.as_extension().is_some());
	assert!(
		event
			.as_extension()
			.and_then(|extension| extension.extension::<common::TestExtensionEvent>())
			.is_some()
	);
	assert_eq!(event.event_type(), EventType::Extension);
	assert_eq!(
		event.sub_event(),
		SubEventType::Extension(puniyu_event::extension::ExtensionSubEventType::new(
			"test.extension"
		))
	);
	assert_eq!(
		common::event_snapshot(&event),
		(
			2,
			"ext-event-1".to_string(),
			"123456".to_string(),
			"123456".to_string(),
			"123456".to_string(),
		)
	);
}
