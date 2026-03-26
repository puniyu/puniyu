/// 注册机器人宏
///
/// 提供便捷的方式创建并注册机器人到注册表。
///
/// # 用法
///
/// ```rust,ignore
/// use puniyu_bot::register_bot;
///
/// let index = register_bot!(
///     adapter: adapter_info,
///     api: api_instance,
///     account: account_info,
/// )?;
///
/// println!("机器人已注册，索引: {}", index);
/// ```
///
/// # 参数
///
/// - `adapter` - 适配器信息
/// - `api` - 适配器 API 实例
/// - `account` - 账户信息
///
/// # 返回值
///
/// 返回分配的索引号，失败时返回错误
#[macro_export]
macro_rules! register_bot {
    (adapter: $adapter:expr, api: $api:expr, account: $account:expr) => {
        let bot = Bot::new($adapter, $api, $account);
        $crate::BotRegistry::register(bot)
    };
}

/// 注销机器人宏
///
/// 提供便捷的方式从注册表注销机器人。支持使用索引或 UIN 注销。
///
/// # 用法
///
/// ## 使用索引注销
///
/// ```rust,ignore
/// use puniyu_bot::unregister_bot;
///
/// // 直接传入索引
/// unregister_bot!(123)?;
///
/// // 或使用命名参数
/// unregister_bot!(index: 123)?;
/// ```
///
/// ## 使用 UIN 注销
///
/// ```rust,ignore
/// use puniyu_bot::unregister_bot;
///
/// unregister_bot!(bot_id: "123456")?;
/// ```
///
/// # 参数
///
/// - 直接传入 `u64` 索引
/// - `index: u64` - 机器人的注册表索引
/// - `bot_id: &str` - 机器人的 UIN
#[macro_export]
macro_rules! unregister_bot {
    ($index:expr) => {
        $crate::BotRegistry::unregister_with_index($index)
    };
    (index: $index:expr) => {
        $crate::BotRegistry::unregister_with_index($index)
    };
    (bot_id: $bot_id:expr) => {
        $crate::BotRegistry::unregister_with_bot_id($bot_id)
    };
}