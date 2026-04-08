
use async_trait::async_trait;
use bytes::Bytes;
use puniyu_account::AccountInfo;
use puniyu_adapter_api::{AdapterApi, Runtime, Error};
use puniyu_adapter_types::{AdapterPlatform, AdapterProtocol, SendMsgType, adapter_info};
use puniyu_bot::Bot;
use puniyu_contact::{ContactType, contact_friend};
use puniyu_event::Event;
use puniyu_event::request::{PrivateApply, PrivateApplyType, RequestEvent};
use puniyu_handler::Handler;
use puniyu_handler_command::Handler as CommandHandler;
use puniyu_message::Message;
use puniyu_sender::{FriendSender, sender_friend};

struct TestRuntime;

#[async_trait]
impl Runtime for TestRuntime {
	async fn send_message(
		&self,
		_contact: &ContactType<'_>,
		_message: &Message,
	) -> Result<SendMsgType, Error> {
		Ok(SendMsgType { message_id: "test-msg".to_string(), time: 0 })
	}

}

fn make_request_event() -> Event<'static> {
	let adapter = adapter_info!("console", AdapterPlatform::QQ, AdapterProtocol::Console);
	let account =
		AccountInfo { uin: "10000".to_string(), name: "Puniyu".to_string(), avatar: Bytes::new() };
	let api = AdapterApi::from_runtime(TestRuntime);
	let bot = Box::leak(Box::new(Bot::new(adapter, api, account)));

	let contact = Box::leak(Box::new(contact_friend!(peer: "123456", name: "Alice")));
	let sender: &'static FriendSender<'static> =
		Box::leak(Box::new(sender_friend!(user_id: "123456", nick: "Alice")));
	let content = Box::leak(Box::new(PrivateApplyType { message: "hello".to_string() }));

	let request = PrivateApply::new(bot, "evt-1", "123456", contact, sender, 0, content);

	Event::Request(Box::new(RequestEvent::PrivateApply(request)))
}

#[test]
fn command_handler_name_matches_expected() {
	let handler = CommandHandler;
	assert_eq!(handler.name(), "command");
	assert_eq!(handler.priority(), 5);
}

#[tokio::test]
async fn command_handler_ignores_non_message_events() {
	let handler = CommandHandler;
	let event = make_request_event();

	let result = handler.handle(&event).await;
	assert!(result.is_ok());
}
