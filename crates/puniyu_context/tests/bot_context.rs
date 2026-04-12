use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;
use puniyu_account::AccountInfo;
use puniyu_adapter_types::{AdapterInfo, AdapterPlatform, AdapterProtocol, SendMsgType, adapter_info};
use puniyu_bot::Bot;
use puniyu_contact::ContactType;
use puniyu_context::BotContext;
use puniyu_message::Message;
use puniyu_runtime::{AccountProvider, AdapterProvider, SendMessage};

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

fn make_bot_with_account(uin: &str, name: &str, avatar: Bytes) -> Arc<dyn Bot> {
	let adapter = adapter_info!(
		name: "test-adapter",
		platform: AdapterPlatform::Other,
		protocol: AdapterProtocol::Console,
	);
	let account = AccountInfo { uin: uin.to_string(), name: name.to_string(), avatar };
	Arc::new(TestBot { runtime: Arc::new(TestRuntime { adapter, account }) })
}

#[test]
fn test_bot_context_creation() {
	let bot = make_bot_with_account("bot123", "TestBot", Bytes::new());
	let context = BotContext::new(bot.as_ref());

	assert_eq!(context.account().uin, "bot123");
	assert_eq!(context.account().name, "TestBot");
}

#[test]
fn test_bot_context_with_avatar() {
	let bot = make_bot_with_account(
		"bot123",
		"TestBot",
		Bytes::from("https://example.com/avatar.jpg"),
	);
	let context = BotContext::new(bot.as_ref());

	assert_eq!(context.account().avatar, "https://example.com/avatar.jpg");
}
