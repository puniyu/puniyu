/// 创建并注册机器人。
///
/// ```rust
/// use puniyu_account::AccountInfo;
/// use puniyu_adapter_api::AdapterApi;
/// use puniyu_adapter_types::{AdapterInfo, AdapterPlatform, AdapterProtocol};
/// use puniyu_bot::{register_bot, unregister_bot, BotRegistry};
///
/// let mut adapter = AdapterInfo::default();
/// adapter.name = "console".to_string();
/// adapter.platform = AdapterPlatform::QQ;
/// adapter.protocol = AdapterProtocol::Console;
///
/// let account = AccountInfo {
///     uin: "123456789".to_string(),
///     name: "Puniyu".to_string(),
///     avatar: Default::default(),
/// };
///
/// let index = register_bot!(
///     adapter: adapter,
///     api: AdapterApi::default(),
///     account: account,
/// )
/// .unwrap();
///
/// assert!(BotRegistry::get(index).is_some());
/// unregister_bot!(index).unwrap();
/// ```
#[macro_export]
macro_rules! register_bot {
	(adapter: $adapter:expr, api: $api:expr, account: $account:expr $(,)?) => {{
		let bot = $crate::Bot::new($adapter, $api, $account);
		$crate::BotRegistry::register(bot)
	}};
}

/// 按索引或 UIN 注销机器人。
///
/// ```rust
/// use puniyu_account::AccountInfo;
/// use puniyu_adapter_api::AdapterApi;
/// use puniyu_adapter_types::{AdapterInfo, AdapterPlatform, AdapterProtocol};
/// use puniyu_bot::{get_bot, register_bot, unregister_bot};
///
/// let mut adapter = AdapterInfo::default();
/// adapter.name = "console".to_string();
/// adapter.platform = AdapterPlatform::QQ;
/// adapter.protocol = AdapterProtocol::Console;
///
/// let account = AccountInfo {
///     uin: "123456789".to_string(),
///     name: "Puniyu".to_string(),
///     avatar: Default::default(),
/// };
///
/// let index = register_bot!(
///     adapter: adapter,
///     api: AdapterApi::default(),
///     account: account,
/// )
/// .unwrap();
///
/// assert!(get_bot(index).is_some());
/// unregister_bot!(bot_id: "123456789").unwrap();
/// assert!(get_bot(index).is_none());
/// ```
#[macro_export]
macro_rules! unregister_bot {
	($index:expr $(,)?) => {{ $crate::BotRegistry::unregister_with_index($index) }};
	(index: $index:expr $(,)?) => {{ $crate::BotRegistry::unregister_with_index($index) }};
	(bot_id: $bot_id:expr $(,)?) => {{ $crate::BotRegistry::unregister_with_bot_id($bot_id) }};
}
