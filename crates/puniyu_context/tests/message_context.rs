use puniyu_contact::Contact;
use puniyu_context::MessageContext;
use puniyu_event::{
	EventBase, EventType, SubEventType,
	message::{MessageBase, MessageSubEventType},
};
use puniyu_sender::Sender;

mod common;

#[test]
fn message_context_implements_event_and_message_traits() {
	let ctx: MessageContext<'static> = common::make_message_context();

	assert_eq!(ctx.event_type(), EventType::Message);
	assert_eq!(ctx.sub_event(), SubEventType::Message(MessageSubEventType::Friend));
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
	assert_eq!(common::message_summary(&ctx), ("msg-1", 3, vec!["10000"], Some("reply-1")));
}

#[test]
fn message_context_inherent_helpers_still_work() {
	let ctx = common::make_message_context();

	assert!(ctx.is_friend());
	assert!(!ctx.is_group());
	assert!(!ctx.is_group_temp());
	assert_eq!(ctx.get_at(), vec!["10000"]);
	assert!(ctx.mentions_bot());
	assert_eq!(ctx.get_reply_id(), Some("reply-1"));
	assert_eq!(ctx.contact().peer(), "123456");
	assert_eq!(ctx.sender().user_id(), "123456");
}

#[test]
fn group_temp_message_context_helpers_work() {
	let ctx = common::make_group_temp_message_context();

	assert!(!ctx.is_friend());
	assert!(!ctx.is_group());
	assert!(ctx.is_group_temp());
	assert!(ctx.as_group().is_none());
	assert!(ctx.as_group_temp().is_some());
	assert_eq!(ctx.contact().peer(), "654321");
	assert!(ctx.contact().is_group_temp());
	assert_eq!(common::sender_variant_name(&ctx), "grouptemp");
}
