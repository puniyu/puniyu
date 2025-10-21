use puniyu_bot::{BotRegistry, register_bot};
use puniyu_builder::adapter::{
	AccountInfo, AdapterCommunication, AdapterInfo, AdapterPlatform, AdapterProtocol,
	AdapterStandard,
};
use puniyu_builder::{account_info, adapter_info};

#[test]
fn get_bot() {
	use puniyu_core::bot::get_bot;
	use std::time::SystemTime;
	let start_time = SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
	let adapter_info = adapter_info!(
		name: "Console",
		platform: AdapterPlatform::Other,
		standard: AdapterStandard::Other,
		protocol: AdapterProtocol::Console,
		communication: AdapterCommunication::Other,
		connect_time: start_time
	);
	let bot_id = "test";
	let account_info = account_info!(
		uin: bot_id,
		name: bot_id,
		avatar: String::new()
	);
	register_bot!(adapter_info, account_info);
	let bot = get_bot(0);
	assert!(bot.is_some());
}

#[test]
fn get_bot_with_id() {
	use puniyu_core::bot::get_bot;
	use std::time::SystemTime;
	let start_time = SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
	let adapter_info = adapter_info!(
		name: "Console",
		platform: AdapterPlatform::Other,
		standard: AdapterStandard::Other,
		protocol: AdapterProtocol::Console,
		communication: AdapterCommunication::Other,
		connect_time: start_time
	);
	let bot_id = "test";
	let account_info = account_info!(
		uin: bot_id,
		name: bot_id,
		avatar: String::new()
	);
	register_bot!(adapter_info, account_info);
	let bot = get_bot("test");
	assert!(bot.is_some());
}
