/// 创建并注册机器人。
///
#[macro_export]
macro_rules! register_bot {
	(adapter: $adapter:expr, runtime: $runtime:expr, account: $account:expr $(,)?) => {{
		let runtime: ::std::sync::Arc<dyn $crate::FrameworkRuntime> = $runtime;
		let bot = $crate::Bot::new($adapter, runtime, $account);
		$crate::BotRegistry::register(bot)
	}};
}

/// 按索引或 UIN 注销机器人。
#[macro_export]
macro_rules! unregister_bot {
	($index:expr $(,)?) => {{ $crate::BotRegistry::unregister_with_index($index) }};
	(index: $index:expr $(,)?) => {{ $crate::BotRegistry::unregister_with_index($index) }};
	(bot_id: $bot_id:expr $(,)?) => {{ $crate::BotRegistry::unregister_with_bot_id($bot_id) }};
}
