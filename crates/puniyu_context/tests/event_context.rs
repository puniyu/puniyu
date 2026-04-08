use puniyu_context::EventContext;
use puniyu_event::{EventBase, EventType, SubEventType};
mod common;

#[test]
fn event_context_implements_event_base() {
	let ctx: EventContext<'static> = common::make_event_context();

	assert_eq!(ctx.event_type(), EventType::Message);
	assert_eq!(
		ctx.sub_event(),
		SubEventType::Message(puniyu_event::message::MessageSubEventType::Friend)
	);
	assert_eq!(
		common::base_snapshot(&ctx),
		(
			1,
			"msg-event-1".to_string(),
			"123456".to_string(),
			"123456".to_string(),
			"123456".to_string(),
		)
	);
}
