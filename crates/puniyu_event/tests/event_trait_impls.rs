use puniyu_event::{EventBase, EventType, SubEventType, message::MessageSubEventType};
mod common;

#[test]
fn message_event_implements_event_and_message_traits() {
	let event = common::make_message_event();

	assert_eq!(event.event_type(), EventType::Message);
	assert_eq!(event.sub_event(), SubEventType::Message(MessageSubEventType::Friend));
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
fn group_temp_message_event_implements_event_and_message_traits() {
	let event = common::make_group_temp_message_event();

	assert_eq!(event.event_type(), EventType::Message);
	assert_eq!(event.sub_event(), SubEventType::Message(MessageSubEventType::GroupTemp));
	assert_eq!(
		common::base_snapshot(&event),
		(
			2,
			"msg-event-temp-1".to_string(),
			"123456".to_string(),
			"654321".to_string(),
			"123456".to_string(),
		)
	);
	assert_eq!(common::message_summary(&event), ("msg-temp-1", 0));
	assert_eq!(common::sender_variant_name(&event), "grouptemp");
	assert!(event.as_group_temp().is_some());
}
