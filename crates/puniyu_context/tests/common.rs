#![allow(dead_code)]

use std::collections::HashMap;

use async_trait::async_trait;
use bytes::Bytes;
use puniyu_account::AccountInfo;
use puniyu_adapter_runtime::{AdapterRuntime, Runtime};
use puniyu_adapter_types::{AdapterPlatform, AdapterProtocol, SendMsgType, adapter_info};
use puniyu_bot::Bot;
use puniyu_command_types::ArgValue;
use puniyu_contact::{Contact, ContactType, contact_friend, contact_group_temp};
use puniyu_context::{EventContext, MessageContext};
use puniyu_element::receive::{AtElement, Elements, ReplyElement, TextElement};
use puniyu_event::{
	Event, EventBase,
	message::{FriendMessage, GroupTempMessage, MessageBase, MessageEvent},
};
use puniyu_message::Message;
use puniyu_sender::{Sender, SenderType, sender_friend, sender_group_temp};

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
	make_bot_with_account("10000", "Puniyu", Bytes::new())
}

pub fn make_bot_with_account(uin: &str, name: &str, avatar: Bytes) -> Bot {
	let adapter = adapter_info!(
		name: "test-adapter",
		platform: AdapterPlatform::Other,
		protocol: AdapterProtocol::Console,
	);
	let account = AccountInfo { uin: uin.to_string(), name: name.to_string(), avatar };

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

pub fn message_summary<M>(message: &M) -> (&str, usize, Vec<&str>, Option<&str>)
where
	M: MessageBase,
{
	(message.message_id(), message.elements().len(), message.get_at(), message.get_reply_id())
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

pub fn leak_empty_elements() -> &'static Vec<Elements<'static>> {
	Box::leak(Box::new(Vec::new()))
}

pub fn leak_message_elements() -> &'static Vec<Elements<'static>> {
	Box::leak(Box::new(vec![
		Elements::Text(TextElement::from("hello")),
		Elements::At(AtElement::from("10000")),
		Elements::Reply(ReplyElement::from("reply-1")),
	]))
}

pub fn make_message_event(elements: &'static Vec<Elements<'static>>) -> MessageEvent<'static> {
	MessageEvent::Friend(FriendMessage::new(
		leak_bot(),
		"msg-event-1",
		"123456",
		leak_contact(),
		leak_sender(),
		1,
		"msg-1",
		elements,
	))
}

pub fn make_message_context() -> MessageContext<'static> {
	let event = Box::leak(Box::new(make_message_event(leak_message_elements())));
	MessageContext::new(event, HashMap::<String, ArgValue>::new())
}

pub fn leak_group_temp_contact() -> &'static puniyu_contact::GroupTempContact<'static> {
	Box::leak(Box::new(contact_group_temp!(peer: "654321", name: "Temp Group")))
}

pub fn leak_group_temp_sender() -> &'static puniyu_sender::GroupTempSender<'static> {
	Box::leak(Box::new(sender_group_temp!(user_id: "123456")))
}

pub fn make_group_temp_message_event(elements: &'static Vec<Elements<'static>>) -> MessageEvent<'static> {
	MessageEvent::GroupTemp(GroupTempMessage::new(
		leak_bot(),
		"msg-event-temp-1",
		"123456",
		leak_group_temp_contact(),
		leak_group_temp_sender(),
		2,
		"msg-temp-1",
		elements,
	))
}

pub fn make_group_temp_message_context() -> MessageContext<'static> {
	let event = Box::leak(Box::new(make_group_temp_message_event(leak_message_elements())));
	MessageContext::new(event, HashMap::<String, ArgValue>::new())
}

pub fn sender_variant_name(ctx: &MessageContext<'_>) -> &'static str {
	match ctx.sender() {
		SenderType::Friend(_) => "friend",
		SenderType::Group(_) => "group",
		SenderType::GroupTemp(_) => "grouptemp",
	}
}

pub fn make_event_context() -> EventContext<'static> {
	let event =
		Box::leak(Box::new(Event::Message(Box::new(make_message_event(leak_empty_elements())))));

	EventContext::new(event)
}
