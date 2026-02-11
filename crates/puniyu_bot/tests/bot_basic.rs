use puniyu_account::AccountInfo;
use puniyu_adapter_core::adapter_info;
use puniyu_adapter_core::api::AdapterApi;
use puniyu_adapter_core::types::info::{AdapterPlatform, AdapterProtocol};
use puniyu_bot::Bot;
use puniyu_version::Version;

fn create_test_bot() -> Bot {
	let adapter = adapter_info!(
		name: "test_adapter",
		platform: AdapterPlatform::QQ,
		protocol: AdapterProtocol::Console,
		version: Version::new(1, 0, 0),
	);

	let api = AdapterApi::default();

	let account = AccountInfo {
		uin: "123456".to_string(),
		name: "TestBot".to_string(),
		avatar: "".to_string(),
	};

	Bot::new(adapter, api, account)
}

#[test]
fn test_bot_creation() {
	let bot = create_test_bot();

	assert_eq!(bot.account().uin, "123456");
	assert_eq!(bot.account().name, "TestBot");
	assert_eq!(bot.adapter().name, "test_adapter");
}

#[test]
fn test_bot_adapter_access() {
	let bot = create_test_bot();

	let adapter = bot.adapter();
	assert_eq!(adapter.name, "test_adapter");
	assert_eq!(adapter.platform, AdapterPlatform::QQ);
	assert_eq!(adapter.protocol, AdapterProtocol::Console);
}

#[test]
fn test_bot_api_access() {
	let bot = create_test_bot();

	let api = bot.api();
	let _message_api = api.message();
	let _group_api = api.group();
	let _friend_api = api.friend();
	let _account_api = api.account();
}

#[test]
fn test_bot_account_access() {
	let bot = create_test_bot();

	let account = bot.account();
	assert_eq!(account.uin, "123456");
	assert_eq!(account.name, "TestBot");
	assert_eq!(account.avatar, "");
}

#[test]
fn test_bot_clone() {
	let bot1 = create_test_bot();
	let bot2 = bot1.clone();

	assert_eq!(bot1.account().uin, bot2.account().uin);
	assert_eq!(bot1.adapter().name, bot2.adapter().name);
}

#[test]
fn test_bot_equality() {
	let bot1 = create_test_bot();
	let bot2 = bot1.clone();

	// 使用克隆的实例进行比较，因为 AdapterInfo 包含时间戳
	assert_eq!(bot1, bot2);
}

#[test]
fn test_bot_with_different_accounts() {
	let adapter = adapter_info!(
		name: "test_adapter",
		platform: AdapterPlatform::QQ,
		protocol: AdapterProtocol::Console,
		version: Version::new(1, 0, 0),
	);

	let api = AdapterApi::default();

	let account1 =
		AccountInfo { uin: "123456".to_string(), name: "Bot1".to_string(), avatar: "".to_string() };

	let account2 =
		AccountInfo { uin: "789012".to_string(), name: "Bot2".to_string(), avatar: "".to_string() };

	let bot1 = Bot::new(adapter.clone(), api.clone(), account1);
	let bot2 = Bot::new(adapter, api, account2);

	assert_ne!(bot1, bot2);
	assert_eq!(bot1.account().uin, "123456");
	assert_eq!(bot2.account().uin, "789012");
}

#[test]
fn test_bot_with_avatar() {
	let adapter = adapter_info!(
		name: "test_adapter",
		platform: AdapterPlatform::QQ,
		protocol: AdapterProtocol::Console,
		version: Version::new(1, 0, 0),
	);

	let api = AdapterApi::default();

	let account = AccountInfo {
		uin: "123456".to_string(),
		name: "TestBot".to_string(),
		avatar: "https://example.com/avatar.jpg".to_string(),
	};

	let bot = Bot::new(adapter, api, account);

	assert_eq!(bot.account().avatar, "https://example.com/avatar.jpg");
}

#[test]
fn test_bot_debug_format() {
	let bot = create_test_bot();

	let debug_str = format!("{:?}", bot);
	assert!(debug_str.contains("Bot"));
}

#[test]
fn test_bot_with_different_platforms() {
	let platforms = vec![
		AdapterPlatform::QQ,
		AdapterPlatform::Wechat,
		AdapterPlatform::Telegram,
		AdapterPlatform::Discord,
	];

	for platform in platforms {
		let adapter = adapter_info!(
			name: "test_adapter",
			platform: platform.clone(),
			protocol: AdapterProtocol::Console,
			version: Version::new(1, 0, 0),
		);

		let api = AdapterApi::default();

		let account = AccountInfo {
			uin: "123456".to_string(),
			name: "TestBot".to_string(),
			avatar: "".to_string(),
		};

		let bot = Bot::new(adapter, api, account);

		assert_eq!(bot.adapter().platform, platform);
	}
}

#[test]
fn test_bot_with_different_protocols() {
	let protocols = vec![
		AdapterProtocol::Console,
		AdapterProtocol::NapCat,
		AdapterProtocol::GoCqHttp,
		AdapterProtocol::Lagrange,
	];

	for protocol in protocols {
		let adapter = adapter_info!(
			name: "test_adapter",
			platform: AdapterPlatform::QQ,
			protocol: protocol.clone(),
			version: Version::new(1, 0, 0),
		);

		let api = AdapterApi::default();

		let account = AccountInfo {
			uin: "123456".to_string(),
			name: "TestBot".to_string(),
			avatar: "".to_string(),
		};

		let bot = Bot::new(adapter, api, account);

		assert_eq!(bot.adapter().protocol, protocol);
	}
}
