use bytes::Bytes;
use puniyu_account::AccountInfo;
use puniyu_adapter_core::adapter_info;
use puniyu_adapter_core::api::AdapterApi;
use puniyu_adapter_core::types::info::{AdapterPlatform, AdapterProtocol};
use puniyu_bot::Bot;
use puniyu_context::BotContext;

fn create_mock_bot() -> Bot {
	let adapter_info = adapter_info!(
		name: "test-adapter",
		platform: AdapterPlatform::Other,
		protocol: AdapterProtocol::Console,
	);
	let account = AccountInfo {
		uin: "bot123".to_string(),
		name: "TestBot".to_string(),
		avatar: Bytes::new(),
	};
	let api = AdapterApi::default();
	Bot::new(adapter_info, api, account)
}

#[test]
fn test_bot_context_creation() {
	let bot = create_mock_bot();
	let context = BotContext::new(&bot);

	assert_eq!(context.account().uin, "bot123");
	assert_eq!(context.account().name, "TestBot");
}

#[test]
fn test_bot_context_api_access() {
	let bot = create_mock_bot();
	let context = BotContext::new(&bot);

	let _api = context.api();
	// API 访问不应该 panic
}

#[test]
fn test_bot_context_account_access() {
	let bot = create_mock_bot();
	let context = BotContext::new(&bot);

	let account = context.account();
	assert_eq!(account.uin, "bot123");
	assert_eq!(account.name, "TestBot");
}

#[test]
fn test_bot_context_clone() {
	let bot = create_mock_bot();
	let context1 = BotContext::new(&bot);
	let context2 = context1.clone();

	assert_eq!(context1.account().uin, context2.account().uin);
}

#[test]
fn test_bot_context_with_different_accounts() {
	let account1 =
		AccountInfo { uin: "bot123".to_string(), name: "Bot1".to_string(), avatar: Bytes::new() };
	let adapter_info1 = adapter_info!(
		name: "test-adapter",
		platform: AdapterPlatform::Other,
		protocol: AdapterProtocol::Console,
	);
	let bot1 = Bot::new(adapter_info1, AdapterApi::default(), account1);
	let context1 = BotContext::new(&bot1);

	let account2 =
		AccountInfo { uin: "bot456".to_string(), name: "Bot2".to_string(), avatar: Bytes::new() };
	let adapter_info2 = adapter_info!(
		name: "test-adapter",
		platform: AdapterPlatform::Other,
		protocol: AdapterProtocol::Console,
	);
	let bot2 = Bot::new(adapter_info2, AdapterApi::default(), account2);
	let context2 = BotContext::new(&bot2);

	assert_eq!(context1.account().uin, "bot123");
	assert_eq!(context2.account().uin, "bot456");
}

#[test]
fn test_bot_context_with_avatar() {
	let account = AccountInfo {
		uin: "bot123".to_string(),
		name: "TestBot".to_string(),
		avatar: Bytes::from("https://example.com/avatar.jpg"),
	};
	let adapter_info = adapter_info!(
		name: "test-adapter",
		platform: AdapterPlatform::Other,
		protocol: AdapterProtocol::Console,
	);
	let bot = Bot::new(adapter_info, AdapterApi::default(), account);
	let context = BotContext::new(&bot);

	assert_eq!(context.account().avatar, "https://example.com/avatar.jpg");
}

#[test]
fn test_bot_context_minimal_account() {
	let account =
		AccountInfo { uin: "bot123".to_string(), name: "".to_string(), avatar: Bytes::new() };
	let adapter_info = adapter_info!(
		name: "test-adapter",
		platform: AdapterPlatform::Other,
		protocol: AdapterProtocol::Console,
	);
	let bot = Bot::new(adapter_info, AdapterApi::default(), account);
	let context = BotContext::new(&bot);

	assert_eq!(context.account().uin, "bot123");
	assert_eq!(context.account().name, "");
}
