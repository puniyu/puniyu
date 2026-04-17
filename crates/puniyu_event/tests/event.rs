use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;
use puniyu_account::AccountInfo;
use puniyu_adapter_types::{
	AdapterInfo, AdapterPlatform, AdapterProtocol, SendMsgType, adapter_info,
};
use puniyu_bot::Bot;
use puniyu_contact::{Contact, ContactType, contact_friend};
use puniyu_element::receive::Elements;
use puniyu_event::{
	Event, EventType, SubEventType,
	extension::{ExtensionEvent, NoticeSubEventType},
	message::{FriendMessage, MessageEvent, MessageSubEventType},
};
use puniyu_message::Message;
use puniyu_runtime::{AccountProvider, AdapterProvider, AdapterRuntime, BotRuntime, SendMessage};
use puniyu_sender::{Sender, sender_friend};

#[derive(Debug)]
struct TestAdapterRuntime {
	adapter: AdapterInfo,
}

impl AdapterProvider for TestAdapterRuntime {
	fn adapter_info(&self) -> &AdapterInfo {
		&self.adapter
	}
}

#[async_trait]
impl SendMessage for TestAdapterRuntime {
	async fn send_message(
		&self,
		_contact: &ContactType<'_>,
		_message: &Message,
	) -> puniyu_error::Result<SendMsgType> {
		Ok(SendMsgType { message_id: "test-msg".to_string(), time: 0 })
	}
}

#[derive(Debug)]
struct TestRuntime {
	adapter: Arc<TestAdapterRuntime>,
	account: AccountInfo,
}

impl AccountProvider for TestRuntime {
	fn account_info(&self) -> &AccountInfo {
		&self.account
	}
}

impl BotRuntime for TestRuntime {
	fn adapter(&self) -> &dyn AdapterRuntime {
		self.adapter.as_ref()
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
	let adapter = adapter_info!("console", AdapterPlatform::QQ, AdapterProtocol::Console);
	let account =
		AccountInfo { uin: "10000".to_string(), name: "Puniyu".to_string(), avatar: Bytes::new() };
	Box::leak(Box::new(
		Arc::new(TestBot {
			runtime: Arc::new(TestRuntime {
				adapter: Arc::new(TestAdapterRuntime { adapter }),
				account,
			}),
		}) as Arc<dyn Bot>
	))
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

fn make_event_message() -> Event<'static> {
	Event::Message(Box::new(MessageEvent::Friend(FriendMessage::new(
		leak_bot().as_ref(),
		"msg-event-1",
		"123456",
		leak_friend_contact(),
		leak_friend_sender(),
		1,
		"msg-1",
		leak_empty_elements(),
	))))
}

fn event_snapshot(event: &Event<'_>) -> (u64, String, String, String, String) {
	(
		event.time(),
		event.event_id().to_string(),
		event.user_id().to_string(),
		event.contact().peer().to_string(),
		event.sender().user_id().to_string(),
	)
}

pub struct TestExtensionEvent {
	bot: &'static Arc<dyn Bot>,
	contact: &'static puniyu_contact::FriendContact<'static>,
	sender: &'static puniyu_sender::FriendSender<'static>,
}

impl puniyu_event::EventBase for TestExtensionEvent {
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
		self.r#type()
	}
	fn bot(&self) -> &dyn Bot {
		self.bot.as_ref()
	}
	fn self_id(&self) -> &str {
		self.bot.account_info().uin.as_str()
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

impl ExtensionEvent for TestExtensionEvent {
	fn r#type(&self) -> SubEventType {
		SubEventType::Notice(NoticeSubEventType::new("friend_poke"))
	}

	fn content(&self) -> &str {
		"Alice poked the bot"
	}
}

fn make_event_extension() -> Event<'static> {
	Event::Extension(Box::new(TestExtensionEvent {
		bot: leak_bot(),
		contact: leak_friend_contact(),
		sender: leak_friend_sender(),
	}))
}

#[test]
fn root_event_message_helpers_work() {
	let event = make_event_message();

	assert!(event.as_message().is_some());
	assert_eq!(event.event_type(), EventType::Message);
	assert_eq!(event.sub_event(), SubEventType::Message(MessageSubEventType::Friend));
	assert_eq!(
		event_snapshot(&event),
		(
			1,
			"msg-event-1".to_string(),
			"123456".to_string(),
			"123456".to_string(),
			"123456".to_string(),
		)
	);
}

#[test]
fn root_event_extension_helpers_work() {
	let event = make_event_extension();

	assert!(event.as_message().is_none());
	assert!(event.as_extension().is_some());
	assert_eq!(event.event_type(), EventType::Extension);
	assert_eq!(event.sub_event(), SubEventType::Notice(NoticeSubEventType::new("friend_poke")));
	assert_eq!(event.as_extension().unwrap().r#type(), SubEventType::Notice(NoticeSubEventType::new("friend_poke")));
	assert_eq!(event.as_extension().unwrap().content(), "Alice poked the bot");
	assert_eq!(
		event_snapshot(&event),
		(
			2,
			"ext-event-1".to_string(),
			"123456".to_string(),
			"123456".to_string(),
			"123456".to_string(),
		)
	);
}
