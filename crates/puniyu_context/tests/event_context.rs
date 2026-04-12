use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;
use puniyu_account::AccountInfo;
use puniyu_adapter_types::{AdapterInfo, AdapterPlatform, AdapterProtocol, SendMsgType, adapter_info};
use puniyu_bot::Bot;
use puniyu_command_types::ArgValue;
use puniyu_contact::{Contact, ContactType, contact_friend};
use puniyu_context::EventContext;
use puniyu_element::receive::Elements;
use puniyu_event::{
	Event, EventBase, EventType, SubEventType,
	message::{FriendMessage, MessageEvent, MessageSubEventType},
};
use puniyu_message::Message;
use puniyu_runtime::{AccountProvider, AdapterProvider, SendMessage};
use puniyu_sender::{Sender, sender_friend};

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
	let account = AccountInfo { uin: "10000".to_string(), name: "Puniyu".to_string(), avatar: Bytes::new() };
	Box::leak(Box::new(Arc::new(TestBot { runtime: Arc::new(TestRuntime { adapter, account }) }) as Arc<dyn Bot>))
}

fn leak_friend_contact() -> &'static puniyu_contact::FriendContact<'static> {
	Box::leak(Box::new(contact_friend!(peer: "123456", name: "Alice")))
}

fn leak_friend_sender() -> &'static puniyu_sender::FriendSender<'static> {
	Box::leak(Box::new(sender_friend!(user_id: "123456", nick: "Alice")))
}

fn leak_empty_elements() -> &'static Vec<Elements<'static>> {
	Box::leak(Box::new(Vec::new()))
}

fn make_event_context() -> EventContext<'static> {
	let event = Box::leak(Box::new(Event::Message(Box::new(MessageEvent::Friend(FriendMessage::new(
		leak_bot().as_ref(),
		"msg-event-1",
		"123456",
		leak_friend_contact(),
		leak_friend_sender(),
		1,
		"msg-1",
		leak_empty_elements(),
	))))));
	let _args = HashMap::<String, ArgValue>::new();
	EventContext::new(event)
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

#[test]
fn event_context_implements_event_base() {
	let ctx: EventContext<'static> = make_event_context();

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
}
