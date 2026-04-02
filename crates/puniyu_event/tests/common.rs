#![allow(dead_code)]

use bytes::Bytes;
use puniyu_account::AccountInfo;
use puniyu_adapter_api::AdapterApi;
use puniyu_adapter_types::{AdapterPlatform, AdapterProtocol, adapter_info};
use puniyu_bot::Bot;
use puniyu_contact::{Contact, contact_friend, contact_group};
use puniyu_element::receive::Elements;
use puniyu_event::{
	Event, EventBase,
	message::{FriendMessage, MessageBase, MessageEvent},
	notion::{GroupRecall, GroupRecallType, NotionBase, NotionEvent},
	request::{PrivateApply, PrivateApplyType, RequestBase, RequestEvent},
};
use puniyu_sender::{Sender, sender_friend, sender_group};

pub fn make_bot() -> Bot {
	let adapter = adapter_info!("console", AdapterPlatform::QQ, AdapterProtocol::Console);
	let account =
		AccountInfo { uin: "10000".to_string(), name: "Puniyu".to_string(), avatar: Bytes::new() };
	Bot::new(adapter, AdapterApi::default(), account)
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

pub fn notion_summary<N>(notion: &N) -> (&str, N::Content)
where
	N: NotionBase,
{
	(notion.notion(), notion.content())
}

pub fn request_summary<R>(request: &R) -> (&str, R::Content)
where
	R: RequestBase,
{
	(request.request(), request.content())
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

pub fn leak_group_sender() -> &'static puniyu_sender::GroupSender<'static> {
	Box::leak(Box::new(sender_group!(user_id: "123456")))
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

pub fn make_notion_event() -> NotionEvent<'static> {
	let content = Box::leak(Box::new(GroupRecallType { message_id: "msg-1".to_string() }));

	NotionEvent::GroupRecall(GroupRecall::new(
		leak_bot(),
		"notion-event-1",
		"123456",
		leak_group_contact(),
		leak_group_sender(),
		2,
		content,
	))
}

pub fn make_request_event() -> RequestEvent<'static> {
	let content = Box::leak(Box::new(PrivateApplyType { message: "hello".to_string() }));

	RequestEvent::PrivateApply(PrivateApply::new(
		leak_bot(),
		"request-event-1",
		"123456",
		leak_contact(),
		leak_sender(),
		3,
		content,
	))
}

pub fn make_event_message() -> Event<'static> {
	Event::Message(Box::new(make_message_event()))
}

pub fn make_event_notion() -> Event<'static> {
	Event::Notion(Box::new(make_notion_event()))
}

pub fn make_event_request() -> Event<'static> {
	Event::Request(Box::new(make_request_event()))
}
