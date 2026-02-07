mod adapter;
mod plugin;

#[cfg(feature = "config")]
#[proc_macro_attribute]
pub fn adapter_config(
	args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	adapter::config(args, item)
}

/// 注册服务路由
///
/// # 示例
/// ```rust, ignore
/// use puniyu_adapter::macros::server;
/// use actix_web::web::{self, ServiceConfig};
///
/// #[server]
/// pub fn routes(cfg: &mut ServiceConfig) {
///     cfg.service(
///         web::resource("/hello")
///             .route(web::get().to(|| async { "Hello World!" }))
///     );
/// }
/// ```
#[cfg(feature = "server")]
#[proc_macro_attribute]
pub fn adapter_server(
	_args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	adapter::server(item)
}

/// 注册适配器API
/// # 示例
/// ```rust, ignore
/// use puniyu_adapter::macros::api;
/// use puniyu_adapter::adapter::AdapterApi
///
/// #[api]
/// pub fn api() -> AdapterApi {
///     /// 实现的适配器Api
///     let group_api = /** 群组Api */;
///     let friend_api = /** 好友Api */;
///     let message_api = /** 消息Api */;
///     let account_api = /** 账号Api */;
///     AdapterApi::new(group_api, friend_api, account_api, message_api)
/// }
/// ```
#[cfg(feature = "adapter")]
#[proc_macro_attribute]
pub fn adapter_api(
	_args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	adapter::api(item)
}

/// 适配器钩子宏
///
/// 用于在适配器中定义钩子函数，在特定事件或状态变化时自动执行。
///
/// # 参数
///
/// | 参数 | 类型 | 必需 | 默认值 | 说明 |
/// |------|------|:----:|--------|------|
/// | `name` | `&str` | | 函数名 | 钩子名称 |
/// | `type` | `&str` | | `"event"` | 钩子类型 |
/// | `rank` | `u64` | | `500` | 优先级，数值越小优先级越高 |
///
/// # 钩子类型
///
/// 钩子类型使用 `category` 或 `category.subtype` 格式：
///
/// ## Event 类型
/// - `"event"` - 所有事件（默认）
/// - `"event.message"` - 消息事件
/// - `"event.notion"` - 通知事件
/// - `"event.request"` - 请求事件
/// - `"event.all"` - 所有事件
///
/// ## Status 类型
/// - `"status"` - 状态变化（默认为 start）
/// - `"status.start"` - 启动状态
/// - `"status.stop"` - 停止状态
///
/// # 编译时验证
///
/// 此宏会在编译时验证钩子类型字符串，如果提供了无效的类型，将产生编译错误。
///
/// # 示例
///
/// ## 基础示例
/// ```rust,ignore
///
/// #[hook(type = "status.start")]
/// async fn on_start(_ev: Option<&Event>) -> HookResult {
///     println!("Adapter started!");
///     Ok(())
/// }
/// ```
///
/// ## 消息事件钩子
/// ```rust,ignore
/// #[hook(type = "event.message", rank = 100)]
/// async fn on_message(ev: Option<&Event>) -> HookResult {
///     if let Some(event) = ev {
///         println!("Received message: {:?}", event);
///     }
///     Ok(())
/// }
/// ```
///
/// ## 自定义名称
/// ```rust,ignore
/// #[hook(name = "my_hook", type = "event.all")]
/// async fn custom_hook(_ev: Option<&Event>) -> HookResult{
///     Ok(())
/// }
/// ```
#[cfg(feature = "hook")]
#[proc_macro_attribute]
pub fn adapter_hook(
	args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	adapter::hook(args, item)
}

/// 适配器宏
/// 
/// 此宏需要一个info参数
/// 
/// ## 示例
/// ```rust,ignore
/// use puniyu_adapter::macros::adapter;
/// use puniyu_adapter::handler::HandlerResult;
/// use puniyu_adapter::adapter::*;
/// 
/// fn info() -> AdapterInfo {
///     adapter_info!(
///         name: env!("CARGO_PKG_NAME"),
///         version: env!("CARGO_PKG_VERSION").into(),
///         platform: AdapterPlatform::QQ,
///         standard: AdapterStandard::OneBotV11,
///         protocol: AdapterProtocol::NapCat,
///         communication: AdapterCommunication::WebSocketClient,
///         connect_time: Utc::now()
///     )
/// }
/// 
/// async fn main() -> HandlerResult {}
/// ```
 #[cfg(feature = "adapter")]
#[proc_macro_attribute]
pub fn adapter(
	args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	adapter::adapter(args, item)
}

#[cfg(feature = "config")]
#[proc_macro_attribute]
pub fn plugin_config(
	args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	plugin::config(args, item)
}

/// 注册插件
/// 此宏包含以下检测：
/// 1. 函数是否为异步函数
/// 2. 插件名称，插件版本，插件作者
/// 3. 插件名称是否规范
///
/// # 示例
/// ## 最小化示例
/// ```rust, ignore
/// use puniyu_plugin::plugin;
///
/// #[plugin]
/// pub async fn hello() {}
/// ```
///
/// ## 完整示例
/// ```rust, ignore
/// use puniyu_plugin::plugin;
///
/// #[plugin]
/// pub async fn hello() -> Result<(), Box<dyn std::error::Error>> {
///     println!("Plugin initialized!");
///     Ok(())
/// }
/// ```
#[cfg(feature = "plugin")]
#[proc_macro_attribute]
pub fn plugin(
	args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	plugin::plugin(args, item)
}

/// 命令宏
///
/// 用于定义命令处理函数。
///
/// # 参数
///
/// | 参数 | 类型 | 必需 | 默认值 | 说明 |
/// |------|------|:----:|--------|------|
/// | `name` | `&str` | ✓ | - | 命令名称 |
/// | `desc` | `&str` | | `None` | 命令描述 |
/// | `rank` | `u64` | | `500` | 优先级，数值越小优先级越高 |
/// | `alias` | `[&str]` | | `[]` | 命令别名列表 |
/// | `permission` | `&str` | | `"all"` | 权限等级：`"all"` 所有人，`"master"` 仅主人 |
///
/// # 命令别名
///
/// 使用 `alias` 为命令定义别名，用户可以通过别名触发命令：
///
/// ```rust,ignore
/// #[command(name = "help", alias = ["h", "?"])]
/// async fn help(bot: &BotContext, ev: &MessageContext) -> HandlerResult {
///     // !help、!h、!? 都可以触发此命令
///     Ok(().into())
/// }
/// ```
///
/// # 命令参数
///
/// 使用 `#[arg]` 属性宏定义命令参数，可以在同一个函数上使用多个 `#[arg]` 属性：
///
/// ```rust,ignore
/// #[command(name = "echo", desc = "回显消息")]
/// #[arg(name = "message", type = "string", required = true, desc = "要回显的消息")]
/// #[arg(name = "count", type = "int", mode = "named", desc = "重复次数")]
/// async fn echo(bot: &BotContext, ev: &MessageContext) -> HandlerResult {
///     let msg = ev.arg("message").and_then(|v| v.as_str()).unwrap_or("");
///     let count = ev.arg("count").and_then(|v| v.as_int()).unwrap_or(1);
///     
///     for _ in 0..count {
///         bot.reply(message!(segment!(text, msg))).await?;
///     }
///     Ok(().into())
/// }
/// ```
///
/// ## `#[arg]` 参数说明
///
/// | 参数 | 类型 | 必需 | 默认值 | 说明 |
/// |------|------|:----:|--------|------|
/// | `name` | `&str` | ✓ | - | 参数名称 |
/// | `type` | `&str` | | `"string"` | 参数类型 |
/// | `mode` | `&str` | | `"positional"` | 参数模式 |
/// | `required` | `bool` | | `false` | 是否必需 |
/// | `desc` | `&str` | | `None` | 参数描述 |
///
/// ### 参数类型
///
/// | 类型 | 说明 |
/// |------|------|
/// | `"string"` | 字符串类型（默认） |
/// | `"int"` | 整数类型 |
/// | `"float"` | 浮点数类型 |
/// | `"bool"` | 布尔类型 |
///
/// ### 参数模式
///
/// | 模式 | 说明 |
/// |------|------|
/// | `"positional"` | 位置参数（默认），按顺序匹配 |
/// | `"named"` | 命名参数，需要 `--flag value` 格式 |
///
/// # 示例
///
/// ## 基础示例
///
/// ```rust,ignore
/// #[command(name = "echo", desc = "回显消息")]
/// #[arg(name = "message")]
/// async fn echo(bot: &BotContext, ev: &MessageContext) -> HandlerResult {
///     // 调用：!echo hello
///     let msg = ev.arg("message").and_then(|v| v.as_str()).unwrap_or("");
///     bot.reply(message!(segment!(text, msg))).await?;
///     Ok(().into())
/// }
/// ```
///
/// ## 无参数命令
///
/// ```rust,ignore
/// #[command(name = "ping", alias = ["p"], desc = "测试延迟")]
/// async fn ping(bot: &BotContext, ev: &MessageContext) -> HandlerResult {
///     // !ping 或 !p 都可以触发
///     bot.reply(message!(segment!(text, "pong!"))).await?;
///     Ok(().into())
/// }
/// ```
///
/// ## 权限控制
///
/// 使用 `permission` 限制命令的使用权限，权限不足时会自动提示：
///
/// ```rust,ignore
/// #[command(name = "reload", desc = "重载配置", permission = "master")]
/// async fn reload(bot: &BotContext, ev: &MessageContext) -> HandlerResult {
///     // 仅主人可执行此命令，其他用户会收到"权限不足"提示
///     bot.reply(message!(segment!(text, "配置已重载"))).await?;
///     Ok(().into())
/// }
/// ```
///
/// ## 多参数示例
///
/// ```rust,ignore
/// #[command(name = "repeat", alias = ["r"], desc = "重复消息")]
/// #[arg(name = "message", required = true, desc = "要重复的消息")]
/// #[arg(name = "count", type = "int", mode = "named", desc = "重复次数")]
/// async fn repeat(bot: &BotContext, ev: &MessageContext) -> HandlerResult {
///     // 调用：!repeat hello --count 3
///     let message = ev.arg("message").and_then(|v| v.as_str()).unwrap_or("");
///     let count = ev.arg("count").and_then(|v| v.as_int()).unwrap_or(1);
///     
///     for _ in 0..count {
///         bot.reply(message!(segment!(text, message))).await?;
///     }
///     Ok(().into())
/// }
/// ```
///
/// ## 混合参数模式
///
/// ```rust,ignore
/// #[command(name = "calc", desc = "计算器")]
/// #[arg(name = "a", type = "int", required = true, desc = "第一个数")]
/// #[arg(name = "b", type = "int", required = true, desc = "第二个数")]
/// #[arg(name = "op", mode = "named", desc = "运算符")]
/// async fn calc(bot: &BotContext, ev: &MessageContext) -> HandlerResult {
///     // 调用：!calc 10 20 --op add
///     let a = ev.arg("a").and_then(|v| v.as_int()).unwrap_or(0);
///     let b = ev.arg("b").and_then(|v| v.as_int()).unwrap_or(0);
///     let op = ev.arg("op").and_then(|v| v.as_str()).unwrap_or("add");
///     
///     let result = match op {
///         "add" => a + b,
///         "sub" => a - b,
///         "mul" => a * b,
///         "div" => a / b,
///         _ => 0,
///     };
///     
///     bot.reply(message!(segment!(text, format!("结果: {}", result)))).await?;
///     Ok(().into())
/// }
/// ```
#[cfg(feature = "command")]
#[proc_macro_attribute]
pub fn command(
	args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	plugin::command(args, item)
}

#[cfg(feature = "command")]
#[proc_macro_attribute]
pub fn arg(
	_args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	item
}

/// 定时任务宏
///
/// 用于定义基于 Cron 表达式的定时任务。
///
/// # 参数
/// - `cron`: Cron 表达式（必需），定义任务执行的时间规则
/// - `name`: 任务名称（可选），默认使用函数名
///
/// # Cron 表达式格式
///
/// 支持标准的 Cron 表达式，格式为：
/// ```text
/// ┌───────────── 秒 (0 - 59)
/// │ ┌───────────── 分钟 (0 - 59)
/// │ │ ┌───────────── 小时 (0 - 23)
/// │ │ │ ┌───────────── 日期 (1 - 31)
/// │ │ │ │ ┌───────────── 月份 (1 - 12)
/// │ │ │ │ │ ┌───────────── 星期 (0 - 6) (周日到周六)
/// │ │ │ │ │ │
/// * * * * * *
/// ```
///
/// ## 特殊字符
/// - `*`: 匹配所有值
/// - `,`: 分隔多个值，如 `1,3,5`
/// - `-`: 指定范围，如 `1-5`
/// - `/`: 指定步长，如 `*/5` 表示每 5 个单位
///
/// ## 常用示例
/// - `0 0 * * * *`: 每小时整点执行
/// - `0 */30 * * * *`: 每 30 分钟执行
/// - `0 0 9 * * *`: 每天上午 9 点执行
/// - `0 0 0 * * 1`: 每周一凌晨执行
/// - `0 0 0 1 * *`: 每月 1 号凌晨执行
///
/// # 函数要求
/// - 必须是 `async` 函数
/// - 不能有任何参数
/// - 返回类型必须是 `Result<(), Box<dyn std::error::Error + Send + Sync>>`
///
/// # 示例
///
/// ## 基础示例
/// ```rust,ignore
/// #[task(cron = "0 0 * * * *")]
/// async fn hourly_cleanup() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
///     println!("每小时执行一次清理任务");
///     Ok(())
/// }
/// ```
///
/// ## 指定任务名称
/// ```rust,ignore
/// #[task(name = "数据备份", cron = "0 0 2 * * *")]
/// async fn backup_data() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
///     println!("每天凌晨 2 点执行数据备份");
///     Ok(())
/// }
/// ```
///
/// ## 高频任务
/// ```rust,ignore
/// #[task(cron = "*/10 * * * * *")]
/// async fn check_status() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
///     println!("每 10 秒检查一次状态");
///     Ok(())
/// }
/// ```
///
/// ## 复杂定时任务
/// ```rust,ignore
/// #[task(
///     name = "工作日报告",
///     cron = "0 0 18 * * 1-5"  // 工作日下午 6 点
/// )]
/// async fn daily_report() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
///     println!("生成每日工作报告");
///     // 执行报告生成逻辑
///     Ok(())
/// }
/// ```
#[cfg(feature = "task")]
#[proc_macro_attribute]
pub fn task(
	args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	plugin::task(args, item)
}

/// 注册服务路由
///
/// # 示例
/// ```rust, ignore
/// use puniyu_plugin::server;
/// use actix_web::web::{self, ServiceConfig};
///
/// #[server]
/// pub fn routes(cfg: &mut ServiceConfig) {
///     cfg.service(
///         web::resource("/hello")
///             .route(web::get().to(|| async { "Hello World!" }))
///     );
/// }
/// ```
#[cfg(feature = "server")]
#[proc_macro_attribute]
pub fn server(
	_args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	plugin::server(item)
}

/// 插件钩子宏
///
/// 用于在插件中定义钩子函数，在特定事件或状态变化时自动执行。
///
/// # 参数
///
/// | 参数 | 类型 | 必需 | 默认值 | 说明 |
/// |------|------|:----:|--------|------|
/// | `name` | `&str` | | 函数名 | 钩子名称 |
/// | `type` | `&str` | | `"event"` | 钩子类型 |
/// | `rank` | `u64` | | `500` | 优先级，数值越小优先级越高 |
///
/// # 钩子类型
///
/// 钩子类型使用 `category` 或 `category.subtype` 格式：
///
/// ## Event 类型
/// - `"event"` - 所有事件（默认）
/// - `"event.message"` - 消息事件
/// - `"event.notion"` - 通知事件
/// - `"event.request"` - 请求事件
/// - `"event.all"` - 所有事件
///
/// ## Status 类型
/// - `"status"` - 状态变化（默认为 start）
/// - `"status.start"` - 启动状态
/// - `"status.stop"` - 停止状态
///
/// # 编译时验证
///
/// 此宏会在编译时验证钩子类型字符串，如果提供了无效的类型，将产生编译错误。
///
/// # 示例
///
/// ## 基础示例
/// ```rust,ignore
/// use puniyu_plugin::macros::hook;
/// use puniyu_plugin::event::Event;
///
/// #[hook(type = "status.start")]
/// async fn on_start(_ev: Option<&Event>) -> HookResult{
///     println!("Plugin started!");
///     Ok(())
/// }
/// ```
///
/// ## 消息事件钩子
/// ```rust,ignore
/// #[hook(type = "event.message", rank = 100)]
/// async fn on_message(ev: Option<&Event>) -> HookResult {
///     if let Some(event) = ev {
///         println!("Received message: {:?}", event);
///     }
///     Ok(())
/// }
/// ```
///
/// ## 自定义名称和优先级
/// ```rust,ignore
/// #[hook(name = "message_logger", type = "event.message", rank = 50)]
/// async fn log_messages(ev: Option<&Event>) -> HookResult {
///     // 高优先级消息日志记录
///     Ok(())
/// }
/// ```
///
/// ## 请求事件钩子
/// ```rust,ignore
/// #[hook(type = "event.request")]
/// async fn handle_request(ev: Option<&Event>) -> HookResult {
///     println!("Handling request event");
///     Ok(())
/// }
/// ```
#[cfg(feature = "hook")]
#[proc_macro_attribute]
pub fn plugin_hook(
	args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	plugin::hook(args, item)
}
