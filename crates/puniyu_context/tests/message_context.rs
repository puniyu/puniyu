use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;
use puniyu_account::AccountInfo;
use puniyu_adapter_types::{
	AdapterInfo, AdapterPlatform, AdapterProtocol, SendMsgType, adapter_info,
};
use puniyu_bot::Bot;
use puniyu_command_types::ArgValue;
use puniyu_contact::{Contact, ContactType, contact_friend, contact_group_temp};
use puniyu_context::MessageContext;
use puniyu_element::receive::{AtElement, Elements, ReplyElement, TextElement};
use puniyu_event::{
	EventBase, EventType, SubEventType,
	message::{FriendMessage, GroupTempMessage, MessageBase, MessageEvent, MessageSubEventType},
};
use puniyu_message::Message;
use puniyu_runtime::{AccountProvider, AdapterProvider, SendMessage};
use puniyu_sender::{Sender, SenderType, sender_friend, sender_group_temp};

#[derive(Debug)]
struct TestRuntime {
	adapter: AdapterInfo,
	account: AccountInfo,
}

impl AdapterProvider for TestRuntime {
	fn adapter_info(&self) -> &AdapterInfo {
		&self.adapter
	}
}

impl AccountProvider for TestRuntime {
	fn account_info(&self) -> &AccountInfo {
		&self.account
	}
}

#[async_trait]
impl SendMessage for TestRuntime {
	async fn send_message(
		&self,
		_contact: &ContactType<'_>,
		_message: &Message,
	) -> puniyu_error::Result<SendMsgType> {
		Ok(SendMsgType { message_id: "test-msg".to_string(), time: 0 })
	}
}

#[derive(Debug)]
struct TestBot {
	runtime: Arc<TestRuntime>,
}

impl puniyu_bot::Bot for TestBot {
	fn runtime(&self) -> &dyn puniyu_runtime::BotRuntime {
		self.runtime.as_ref()
	}
}

fn leak_bot() -> &'static Arc<dyn Bot> {
	let adapter = adapter_info!(
		name: "test-adapter",
		platform: AdapterPlatform::Other,
		protocol: AdapterProtocol::Console,
	);
	let account =
		AccountInfo { uin: "10000".to_string(), name: "Puniyu".to_string(), avatar: Bytes::new() };
	Box::leak(Box::new(
		Arc::new(TestBot { runtime: Arc::new(TestRuntime { adapter, account }) }) as Arc<dyn Bot>
	))
}

fn leak_friend_contact() -> &'static puniyu_contact::FriendContact<'static> {
	Box::leak(Box::new(contact_friend!(peer: "123456", name: "Alice")))
}

fn leak_friend_sender() -> &'static puniyu_sender::FriendSender<'static> {
	Box::leak(Box::new(sender_friend!(user_id: "123456", nick: "Alice")))
}

fn leak_group_temp_contact() -> &'static puniyu_contact::GroupTempContact<'static> {
	Box::leak(Box::new(contact_group_temp!(peer: "654321", name: "Temp Group")))
}

fn leak_group_temp_sender() -> &'static puniyu_sender::GroupTempSender<'static> {
	Box::leak(Box::new(sender_group_temp!(user_id: "123456")))
}

fn leak_message_elements() -> &'static Vec<Elements<'static>> {
	Box::leak(Box::new(vec![
		Elements::Text(TextElement::from("hello")),
		Elements::At(AtElement::from("10000")),
		Elements::Reply(ReplyElement::from("reply-1")),
	]))
}

fn make_message_context() -> MessageContext<'static> {
	let event = Box::leak(Box::new(MessageEvent::Friend(FriendMessage::new(
		leak_bot().as_ref(),
		"msg-event-1",
		"123456",
		leak_friend_contact(),
		leak_friend_sender(),
		1,
		"msg-1",
		leak_message_elements(),
	))));
	MessageContext::new(event, HashMap::<String, ArgValue>::new())
}

fn make_group_temp_message_context() -> MessageContext<'static> {
	let event = Box::leak(Box::new(MessageEvent::GroupTemp(GroupTempMessage::new(
		leak_bot().as_ref(),
		"msg-event-temp-1",
		"123456",
		leak_group_temp_contact(),
		leak_group_temp_sender(),
		2,
		"msg-temp-1",
		leak_message_elements(),
	))));
	MessageContext::new(event, HashMap::<String, ArgValue>::new())
}

fn base_snapshot<E>(event: &E) -> (u64, String, String, String, String)
where
	E: EventBase,
{
	(
		event.time(),
		event.event_id().to_string(),
		event.user_id().to_string(),
		event.contact().peer().to_string(),
		event.sender().user_id().to_string(),
	)
}

fn message_summary<M>(message: &M) -> (&str, usize, Vec<&str>, Option<&str>)
where
	M: MessageBase,
{
	(message.message_id(), message.elements().len(), message.get_at(), message.get_reply_id())
}

#[test]
fn message_context_implements_event_and_message_traits() {
	let ctx: MessageContext<'static> = make_message_context();

	assert_eq!(ctx.event_type(), EventType::Message);
	assert_eq!(ctx.sub_event(), SubEventType::Message(MessageSubEventType::Friend));
	assert_eq!(
		base_snapshot(&ctx),
		(
			1,
			"msg-event-1".to_string(),
			"123456".to_string(),
			"123456".to_string(),
			"123456".to_string(),
		)
	);
	assert_eq!(message_summary(&ctx), ("msg-1", 3, vec!["10000"], Some("reply-1")));
}

#[test]
fn message_context_inherent_helpers_still_work() {
	let ctx = make_message_context();

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
	let ctx = make_group_temp_message_context();

	assert!(!ctx.is_friend());
	assert!(!ctx.is_group());
	assert!(ctx.is_group_temp());
	assert!(ctx.as_group().is_none());
	assert!(ctx.as_group_temp().is_some());
	assert_eq!(ctx.contact().peer(), "654321");
	assert!(ctx.contact().is_group_temp());
	assert_eq!(
		match ctx.sender() {
			SenderType::Friend(_) => "friend",
			SenderType::Group(_) => "group",
			SenderType::GroupTemp(_) => "grouptemp",
			SenderType::Guild(_) => "guild",
		},
		"grouptemp"
	);
}
