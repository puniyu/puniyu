use bytes::Bytes;
use puniyu_context::BotContext;

mod common;

#[test]
fn test_bot_context_creation() {
	let bot = common::make_bot_with_account("bot123", "TestBot", Bytes::new());
	let context = BotContext::new(&bot);

	assert_eq!(context.account().uin, "bot123");
	assert_eq!(context.account().name, "TestBot");
}

#[test]
fn test_bot_context_api_access() {
	let bot = common::make_bot();
	let context = BotContext::new(&bot);

	let _api = context.api();
	// API 访问不应该 panic
}

#[test]
fn test_bot_context_account_access() {
	let bot = common::make_bot_with_account("bot123", "TestBot", Bytes::new());
	let context = BotContext::new(&bot);

	let account = context.account();
	assert_eq!(account.uin, "bot123");
	assert_eq!(account.name, "TestBot");
}

#[test]
fn test_bot_context_clone() {
	let bot = common::make_bot();
	let context1 = BotContext::new(&bot);
	let context2 = context1.clone();

	assert_eq!(context1.account().uin, context2.account().uin);
}

#[test]
fn test_bot_context_with_different_accounts() {
	let bot1 = common::make_bot_with_account("bot123", "Bot1", Bytes::new());
	let context1 = BotContext::new(&bot1);

	let bot2 = common::make_bot_with_account("bot456", "Bot2", Bytes::new());
	let context2 = BotContext::new(&bot2);

	assert_eq!(context1.account().uin, "bot123");
	assert_eq!(context2.account().uin, "bot456");
}

#[test]
fn test_bot_context_with_avatar() {
	let bot = common::make_bot_with_account(
		"bot123",
		"TestBot",
		Bytes::from("https://example.com/avatar.jpg"),
	);
	let context = BotContext::new(&bot);

	assert_eq!(context.account().avatar, "https://example.com/avatar.jpg");
}

#[test]
fn test_bot_context_minimal_account() {
	let bot = common::make_bot_with_account("bot123", "", Bytes::new());
	let context = BotContext::new(&bot);

	assert_eq!(context.account().uin, "bot123");
	assert_eq!(context.account().name, "");
}
