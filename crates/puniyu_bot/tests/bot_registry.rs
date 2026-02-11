#![cfg(feature = "registry")]

use puniyu_account::AccountInfo;
use puniyu_adapter_core::adapter_info;
use puniyu_adapter_core::api::AdapterApi;
use puniyu_adapter_core::types::info::{AdapterPlatform, AdapterProtocol};
use puniyu_bot::{Bot, BotRegistry};
use puniyu_version::Version;

fn create_test_bot(uin: &str, name: &str) -> Bot {
	let adapter = adapter_info!(
		name: "test_adapter",
		platform: AdapterPlatform::QQ,
		protocol: AdapterProtocol::Console,
		version: Version::new(1, 0, 0),
	);

	let api = AdapterApi::default();

	let account =
		AccountInfo { uin: uin.to_string(), name: name.to_string(), avatar: "".to_string() };

	Bot::new(adapter, api, account)
}

#[test]
fn test_bot_registry_register() {
	let bot = create_test_bot("100001", "Bot1");
	let index = BotRegistry::register(bot).expect("注册失败");
	assert!(index > 0);

	// 清理
	let _ = BotRegistry::unregister(index);
}

#[test]
fn test_bot_registry_register_multiple() {
	let bot1 = create_test_bot("100002", "Bot2");
	let bot2 = create_test_bot("100003", "Bot3");

	let index1 = BotRegistry::register(bot1).expect("注册 Bot1 失败");
	let index2 = BotRegistry::register(bot2).expect("注册 Bot2 失败");

	assert_ne!(index1, index2);

	// 清理
	let _ = BotRegistry::unregister(index1);
	let _ = BotRegistry::unregister(index2);
}

#[test]
fn test_bot_registry_get_with_index() {
	let bot = create_test_bot("100004", "Bot4");
	let index = BotRegistry::register(bot.clone()).expect("注册失败");

	let retrieved = BotRegistry::get_with_index(index);
	assert!(retrieved.is_some());
	assert_eq!(retrieved.unwrap().account().uin, "100004");

	// 清理
	let _ = BotRegistry::unregister(index);
}

#[test]
fn test_bot_registry_get_with_bot_id() {
	let bot = create_test_bot("100005", "Bot5");
	let index = BotRegistry::register(bot).expect("注册失败");

	let bots = BotRegistry::get_with_bot_id("100005");
	assert_eq!(bots.len(), 1);
	assert_eq!(bots[0].account().uin, "100005");

	// 清理
	let _ = BotRegistry::unregister(index);
}

#[test]
fn test_bot_registry_get_multiple_same_id() {
	let bot1 = create_test_bot("100006", "Bot6A");
	let bot2 = create_test_bot("100006", "Bot6B");

	let index1 = BotRegistry::register(bot1).expect("注册 Bot1 失败");
	let index2 = BotRegistry::register(bot2).expect("注册 Bot2 失败");

	let bots = BotRegistry::get_with_bot_id("100006");
	assert_eq!(bots.len(), 2);

	// 清理
	let _ = BotRegistry::unregister(index1);
	let _ = BotRegistry::unregister(index2);
}

#[test]
fn test_bot_registry_unregister_with_index() {
	let bot = create_test_bot("100007", "Bot7");
	let index = BotRegistry::register(bot).expect("注册失败");

	BotRegistry::unregister_with_index(index).expect("注销失败");

	let retrieved = BotRegistry::get_with_index(index);
	assert!(retrieved.is_none());
}

#[test]
fn test_bot_registry_unregister_with_bot_id() {
	let bot = create_test_bot("100008", "Bot8");
	let index = BotRegistry::register(bot).expect("注册失败");

	BotRegistry::unregister_with_bot_id("100008").expect("注销失败");

	let bots = BotRegistry::get_with_bot_id("100008");
	assert_eq!(bots.len(), 0);

	// 确保已清理
	let _ = BotRegistry::unregister(index);
}

#[test]
fn test_bot_registry_unregister_nonexistent() {
	let result = BotRegistry::unregister_with_index(999999);
	assert!(result.is_err());
}

#[test]
fn test_bot_registry_all() {
	let bot1 = create_test_bot("100009", "Bot9");
	let bot2 = create_test_bot("100010", "Bot10");

	let index1 = BotRegistry::register(bot1).expect("注册 Bot1 失败");
	let index2 = BotRegistry::register(bot2).expect("注册 Bot2 失败");

	let all_bots = BotRegistry::all();
	assert!(all_bots.len() >= 2);

	// 清理
	let _ = BotRegistry::unregister(index1);
	let _ = BotRegistry::unregister(index2);
}

#[test]
fn test_bot_registry_get_with_bot_id_enum() {
	let bot = create_test_bot("100011", "Bot11");
	let index = BotRegistry::register(bot).expect("注册失败");

	let bots = BotRegistry::get("100011");
	assert_eq!(bots.len(), 1);
	assert_eq!(bots[0].account().uin, "100011");

	// 清理
	let _ = BotRegistry::unregister(index);
}

#[test]
fn test_bot_registry_get_with_index_enum() {
	let bot = create_test_bot("100012", "Bot12");
	let index = BotRegistry::register(bot).expect("注册失败");

	let bots = BotRegistry::get(index);
	assert_eq!(bots.len(), 1);
	assert_eq!(bots[0].account().uin, "100012");

	// 清理
	let _ = BotRegistry::unregister(index);
}
