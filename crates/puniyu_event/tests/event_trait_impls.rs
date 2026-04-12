use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;
use puniyu_account::AccountInfo;
use puniyu_adapter_types::{AdapterInfo, AdapterPlatform, AdapterProtocol, SendMsgType, adapter_info};
use puniyu_bot::Bot;
use puniyu_contact::{Contact, ContactType, contact_friend, contact_group_temp};
use puniyu_element::receive::Elements;
use puniyu_event::{EventBase, EventType, SubEventType, message::{FriendMessage, GroupTempMessage, MessageBase, MessageEvent, MessageSubEventType}};
use puniyu_message::Message;
use puniyu_runtime::{AccountProvider, AdapterProvider, SendMessage};
use puniyu_sender::{Sender, SenderType, sender_friend, sender_group_temp};

#[derive(Debug)]
struct TestRuntime {
	adapter: AdapterInfo,
	account: AccountInfo,
}

impl AdapterProvider for TestRuntime {
	fn adapter_info(&self) -> &AdapterInfo { &self.adapter }
}

impl AccountProvider for TestRuntime {
	fn account_info(&self) -> &AccountInfo { &self.account }
}

#[async_trait]
impl SendMessage for TestRuntime {
	async fn send_message(&self, _contact: &ContactType<'_>, _message: &Message) -> puniyu_error::Result<SendMsgType> {
		Ok(SendMsgType { message_id: "test-msg".to_string(), time: 0 })
	}
}

#[derive(Debug)]
struct TestBot {
	runtime: Arc<TestRuntime>,
}

impl puniyu_bot::Bot for TestBot {
	fn runtime(&self) -> &dyn puniyu_runtime::BotRuntime { self.runtime.as_ref() }
}

fn leak_bot() -> &'static Arc<dyn Bot> {
	let adapter = adapter_info!("console", AdapterPlatform::QQ, AdapterProtocol::Console);
	let account = AccountInfo { uin: "10000".to_string(), name: "Puniyu".to_string(), avatar: Bytes::new() };
	Box::leak(Box::new(Arc::new(TestBot { runtime: Arc::new(TestRuntime { adapter, account }) }) as Arc<dyn Bot>))
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

fn leak_empty_elements() -> &'static Vec<Elements<'static>> {
	Box::leak(Box::new(Vec::new()))
}

fn make_message_event() -> MessageEvent<'static> {
	MessageEvent::Friend(FriendMessage::new(
		leak_bot().as_ref(),
		"msg-event-1",
		"123456",
		leak_friend_contact(),
		leak_friend_sender(),
		1,
		"msg-1",
		leak_empty_elements(),
	))
}

fn make_group_temp_message_event() -> MessageEvent<'static> {
	MessageEvent::GroupTemp(GroupTempMessage::new(
		leak_bot().as_ref(),
		"msg-event-temp-1",
		"123456",
		leak_group_temp_contact(),
		leak_group_temp_sender(),
		2,
		"msg-temp-1",
		leak_empty_elements(),
	))
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

fn message_summary<M>(message: &M) -> (&str, usize)
where
	M: MessageBase,
{
	(message.message_id(), message.elements().len())
}

#[test]
fn message_event_implements_event_and_message_traits() {
	let event = make_message_event();

	assert_eq!(event.event_type(), EventType::Message);
	assert_eq!(event.sub_event(), SubEventType::Message(MessageSubEventType::Friend));
	assert_eq!(
		base_snapshot(&event),
		(
			1,
			"msg-event-1".to_string(),
			"123456".to_string(),
			"123456".to_string(),
			"123456".to_string(),
		)
	);
	assert_eq!(message_summary(&event), ("msg-1", 0));
}

#[test]
fn group_temp_message_event_implements_event_and_message_traits() {
	let event = make_group_temp_message_event();

	assert_eq!(event.event_type(), EventType::Message);
	assert_eq!(event.sub_event(), SubEventType::Message(MessageSubEventType::GroupTemp));
	assert_eq!(
		base_snapshot(&event),
		(
			2,
			"msg-event-temp-1".to_string(),
			"123456".to_string(),
			"654321".to_string(),
			"123456".to_string(),
		)
	);
	assert_eq!(message_summary(&event), ("msg-temp-1", 0));
	assert_eq!(
		match event.sender() {
			SenderType::Friend(_) => "friend",
			SenderType::Group(_) => "group",
			SenderType::GroupTemp(_) => "grouptemp",
			SenderType::Guild(_) => "guild",
		},
		"grouptemp"
	);
	assert!(event.as_group_temp().is_some());
}
