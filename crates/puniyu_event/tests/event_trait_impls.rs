use bytes::Bytes;
use puniyu_account::AccountInfo;
use puniyu_adapter_api::AdapterApi;
use puniyu_adapter_types::{AdapterPlatform, AdapterProtocol, adapter_info};
use puniyu_bot::Bot;
use puniyu_contact::Contact;
use puniyu_contact::contact_friend;
use puniyu_element::receive::Elements;
use puniyu_event::{
	EventBase, EventType,
	message::{FriendMessage, MessageBase, MessageBuilder, MessageEvent, MessageSubEventType},
	notion::{
		ContentType as NotionContentType, NotionBase, NotionBuilder, NotionEvent,
		NotionSubEventType, ReceiveLike, ReceiveLikeType,
	},
	request::{
		ContentType as RequestContentType, PrivateApply, PrivateApplyType, RequestBase,
		RequestBuilder, RequestEvent, RequestSubEventType,
	},
};
use puniyu_sender::Sender;
use puniyu_sender::sender_friend;

fn leak_bot() -> &'static Bot {
	let adapter = adapter_info!("console", AdapterPlatform::QQ, AdapterProtocol::Console);
	let account =
		AccountInfo { uin: "10000".to_string(), name: "Puniyu".to_string(), avatar: Bytes::new() };

	Box::leak(Box::new(Bot::new(adapter, AdapterApi::default(), account)))
}

fn leak_contact() -> &'static puniyu_contact::FriendContact<'static> {
	Box::leak(Box::new(contact_friend!(peer: "123456", name: "Alice")))
}

fn leak_sender() -> &'static puniyu_sender::FriendSender<'static> {
	Box::leak(Box::new(sender_friend!(user_id: "123456", nick: "Alice")))
}

fn leak_elements() -> &'static Vec<Elements<'static>> {
	Box::leak(Box::new(Vec::new()))
}

fn make_message_event() -> MessageEvent<'static> {
	MessageEvent::Friend(FriendMessage::new(MessageBuilder {
		bot: leak_bot(),
		event_id: "msg-event-1",
		user_id: "123456",
		contact: leak_contact(),
		sender: leak_sender(),
		time: 1,
		message_id: "msg-1",
		elements: leak_elements(),
	}))
}

fn make_notion_event() -> NotionEvent<'static> {
	let content = Box::leak(Box::new(ReceiveLikeType { count: 1 }));

	NotionEvent::ReceiveLike(ReceiveLike::new(NotionBuilder {
		bot: leak_bot(),
		event_id: "notion-event-1",
		time: 2,
		user_id: "123456",
		contact: leak_contact(),
		sender: leak_sender(),
		content,
	}))
}

fn make_request_event() -> RequestEvent<'static> {
	let content = Box::leak(Box::new(PrivateApplyType { message: "hello".to_string() }));

	RequestEvent::PrivateApply(PrivateApply::new(RequestBuilder {
		bot: leak_bot(),
		event_id: "request-event-1",
		time: 3,
		user_id: "123456",
		contact: leak_contact(),
		sender: leak_sender(),
		content,
	}))
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

fn notion_summary<N>(notion: &N) -> (&str, N::Content)
where
	N: NotionBase,
{
	(notion.notion(), notion.content())
}

fn request_summary<R>(request: &R) -> (&str, R::Content)
where
	R: RequestBase,
{
	(request.request(), request.content())
}

#[test]
fn message_event_implements_event_and_message_traits() {
	let event = make_message_event();

	assert_eq!(event.event_type(), &EventType::Message);
	assert_eq!(event.sub_event(), &MessageSubEventType::Friend);
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
fn notion_event_implements_event_and_notion_traits() {
	let event = make_notion_event();

	assert_eq!(event.event_type(), &EventType::Notion);
	assert_eq!(event.sub_event(), &NotionSubEventType::ReceiveLike);
	assert_eq!(
		base_snapshot(&event),
		(
			2,
			"notion-event-1".to_string(),
			"123456".to_string(),
			"123456".to_string(),
			"123456".to_string(),
		)
	);

	let (notion, content) = notion_summary(&event);
	assert_eq!(notion, "收到点赞事件");
	match content {
		NotionContentType::ReceiveLike(content) => assert_eq!(content.count, 1),
		_ => panic!("expected receive like content"),
	}
}

#[test]
fn request_event_implements_event_and_request_traits() {
	let event = make_request_event();

	assert_eq!(event.event_type(), &EventType::Request);
	assert_eq!(event.sub_event(), &RequestSubEventType::PrivateApply);
	assert_eq!(
		base_snapshot(&event),
		(
			3,
			"request-event-1".to_string(),
			"123456".to_string(),
			"123456".to_string(),
			"123456".to_string(),
		)
	);

	let (request, content) = request_summary(&event);
	assert_eq!(request, "收到好友申请请求");
	match content {
		RequestContentType::PrivateApply(content) => assert_eq!(content.message, "hello"),
		_ => panic!("expected private apply content"),
	}
}
