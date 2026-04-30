/// 创建并注册机器人。
///
#[macro_export]
macro_rules! register_bot {
	(bot: $bot:expr $(,)?) => {{
		let bot: ::std::sync::Arc<$crate::Bot> = $bot;
		$crate::BotRegistry::register(bot)
	}};
	(adapter: $adapter:expr, account: $account:expr $(,)?) => {{
		let adapter: ::std::sync::Arc<dyn $crate::AdapterRuntime> = $adapter;
		let account = $account;
		$crate::BotRegistry::register(::std::sync::Arc::new($crate::Bot::new(adapter, account)))
	}};
	(runtime: $runtime:expr, account: $account:expr $(,)?) => {{
		let adapter: ::std::sync::Arc<dyn $crate::AdapterRuntime> = $runtime;
		let account = $account;
		$crate::BotRegistry::register(::std::sync::Arc::new($crate::Bot::new(adapter, account)))
	}};
}

/// 按索引或 UIN 注销机器人。
#[macro_export]
macro_rules! unregister_bot {
	($index:expr $(,)?) => {{ $crate::BotRegistry::unregister_with_index($index) }};
	(index: $index:expr $(,)?) => {{ $crate::BotRegistry::unregister_with_index($index) }};
	(bot_id: $bot_id:expr $(,)?) => {{ $crate::BotRegistry::unregister_with_bot_id($bot_id) }};
}
