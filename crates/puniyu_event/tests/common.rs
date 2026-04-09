#![allow(dead_code)]


use async_trait::async_trait;
use bytes::Bytes;
use puniyu_account::AccountInfo;
use puniyu_adapter_runtime::{AdapterRuntime, Runtime};
use puniyu_adapter_types::{AdapterPlatform, AdapterProtocol, SendMsgType, adapter_info};
use puniyu_bot::Bot;
use puniyu_contact::{Contact, ContactType, contact_friend, contact_group, contact_group_temp};
use puniyu_element::receive::Elements;
use puniyu_event::{
	Event, EventBase,
	extension::{ExtensionEvent, ExtensionSubEventType},
	message::{FriendMessage, GroupTempMessage, MessageBase, MessageEvent},
};
use puniyu_message::Message;
use puniyu_sender::{Sender, SenderType, sender_friend, sender_group, sender_group_temp};

struct TestRuntime;

#[async_trait]
impl Runtime for TestRuntime {
	async fn send_message(
		&self,
		_contact: &ContactType<'_>,
		_message: &Message,
	) -> puniyu_error::Result<SendMsgType> {
		Ok(SendMsgType { message_id: "test-msg".to_string(), time: 0 })
	}

}

fn test_runtime() -> AdapterRuntime {
	AdapterRuntime::from_runtime(TestRuntime)
}

pub fn make_bot() -> Bot {
	let adapter = adapter_info!("console", AdapterPlatform::QQ, AdapterProtocol::Console);
	let account =
		AccountInfo { uin: "10000".to_string(), name: "Puniyu".to_string(), avatar: Bytes::new() };
	Bot::new(adapter, test_runtime(), account)
}

pub fn base_snapshot<E>(event: &E) -> (u64, String, String, String, String)
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

pub fn event_snapshot(event: &Event<'_>) -> (u64, String, String, String, String) {
	(
		event.time(),
		event.event_id().to_string(),
		event.user_id().to_string(),
		event.contact().peer().to_string(),
		event.sender().user_id().to_string(),
	)
}

pub fn message_summary<M>(message: &M) -> (&str, usize)
where
	M: MessageBase,
{
	(message.message_id(), message.elements().len())
}


pub fn leak_bot() -> &'static Bot {
	Box::leak(Box::new(make_bot()))
}

pub fn leak_contact() -> &'static puniyu_contact::FriendContact<'static> {
	Box::leak(Box::new(contact_friend!(peer: "123456", name: "Alice")))
}

pub fn leak_sender() -> &'static puniyu_sender::FriendSender<'static> {
	Box::leak(Box::new(sender_friend!(user_id: "123456", nick: "Alice")))
}

pub fn leak_group_contact() -> &'static puniyu_contact::GroupContact<'static> {
	Box::leak(Box::new(contact_group!(peer: "654321", name: "Dev Group")))
}

pub fn leak_group_temp_contact() -> &'static puniyu_contact::GroupTempContact<'static> {
	Box::leak(Box::new(contact_group_temp!(peer: "654321", name: "Temp Group")))
}

pub fn leak_group_sender() -> &'static puniyu_sender::GroupSender<'static> {
	Box::leak(Box::new(sender_group!(user_id: "123456")))
}

pub fn leak_group_temp_sender() -> &'static puniyu_sender::GroupTempSender<'static> {
	Box::leak(Box::new(sender_group_temp!(user_id: "123456")))
}

pub fn leak_empty_elements() -> &'static Vec<Elements<'static>> {
	Box::leak(Box::new(Vec::new()))
}

pub fn make_message_event() -> MessageEvent<'static> {
	MessageEvent::Friend(FriendMessage::new(
		leak_bot(),
		"msg-event-1",
		"123456",
		leak_contact(),
		leak_sender(),
		1,
		"msg-1",
		leak_empty_elements(),
	))
}

pub fn make_group_temp_message_event() -> MessageEvent<'static> {
	MessageEvent::GroupTemp(GroupTempMessage::new(
		leak_bot(),
		"msg-event-temp-1",
		"123456",
		leak_group_temp_contact(),
		leak_group_temp_sender(),
		2,
		"msg-temp-1",
		leak_empty_elements(),
	))
}

pub fn sender_variant_name(event: &MessageEvent<'_>) -> &'static str {
	match event.sender() {
		SenderType::Friend(_) => "friend",
		SenderType::Group(_) => "group",
		SenderType::GroupTemp(_) => "grouptemp",
	}
}

pub fn make_event_message() -> Event<'static> {
	Event::Message(Box::new(make_message_event()))
}

pub struct TestExtensionEvent {
	bot: &'static Bot,
	contact: &'static puniyu_contact::FriendContact<'static>,
	sender: &'static puniyu_sender::FriendSender<'static>,
}

impl EventBase for TestExtensionEvent {
	fn time(&self) -> u64 {
		2
	}

	fn event_type(&self) -> puniyu_event::EventType {
		puniyu_event::EventType::Extension
	}

	fn event_id(&self) -> &str {
		"ext-event-1"
	}

	fn sub_event(&self) -> puniyu_event::SubEventType {
		puniyu_event::SubEventType::Extension(ExtensionSubEventType::new("test.extension"))
	}

	fn bot(&self) -> &Bot {
		self.bot
	}

	fn self_id(&self) -> &str {
		self.bot.account().uin.as_str()
	}

	fn user_id(&self) -> &str {
		"123456"
	}

	fn contact(&self) -> puniyu_contact::ContactType<'_> {
		(*self.contact).clone().into()
	}

	fn sender(&self) -> puniyu_sender::SenderType<'_> {
		(*self.sender).clone().into()
	}
}

impl ExtensionEvent for TestExtensionEvent {}

pub fn make_event_extension() -> Event<'static> {
	Event::Extension(Box::new(TestExtensionEvent {
		bot: leak_bot(),
		contact: leak_contact(),
		sender: leak_sender(),
	}))
}
