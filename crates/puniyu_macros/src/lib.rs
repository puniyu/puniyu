mod adapter;
mod common;
mod plugin;
mod types;
pub(crate) use types::*;

use zyn::{ToTokens, syn::spanned::Spanned};

#[zyn::attribute]
pub fn adapter_config(
	#[zyn(input)] item: zyn::syn::ItemStruct,
	args: zyn::Args,
) -> zyn::TokenStream {
	let cfg = match ConfigArgs::from_args(&args) {
		Ok(cfg) => cfg,
		Err(err) => bail!("{err}"),
	};
	adapter::config(item, cfg)
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
/// | `priority` | `u64` | | `500` | 优先级，数值越小优先级越高 |
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
/// #[hook(type = "event.message", priority = 100)]
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

#[zyn::attribute(debug(pretty))]
pub fn adapter_hook(
	#[zyn(input)] item: zyn::syn::ItemFn,
	args: zyn::Args,
) -> proc_macro::TokenStream {
	let cfg = match HookArgs::from_args(&args) {
		Ok(cfg) => cfg,
		Err(err) => bail!("{err}"),
	};
	adapter::hook(item, cfg)
}

#[zyn::attribute(debug(pretty))]
pub fn adapter(#[zyn(input)] item: zyn::syn::ItemFn, args: zyn::Args) -> proc_macro::TokenStream {
	let cfg = match AdapterArgs::from_args(&args) {
		Ok(cfg) => cfg,
		Err(err) => bail!("{err}"),
	};
	adapter::adapter(item, cfg)
}

#[zyn::attribute(debug(pretty))]
pub fn plugin(#[zyn(input)] item: zyn::syn::ItemFn, args: zyn::Args) -> proc_macro::TokenStream {
	let cfg = match PluginArg::from_args(&args) {
		Ok(cfg) => cfg,
		Err(err) => bail!("{err}"),
	};
	plugin::plugin(item, cfg)
}

#[zyn::attribute(debug(pretty))]
pub fn plugin_config(
	#[zyn(input)] item: zyn::syn::ItemStruct,
	args: zyn::Args,
) -> proc_macro::TokenStream {
	let cfg = match ConfigArgs::from_args(&args) {
		Ok(cfg) => cfg,
		Err(err) => bail!("{err}"),
	};
	plugin::config(item, cfg)
}

#[zyn::attribute(debug(pretty))]
pub fn command(#[zyn(input)] item: zyn::syn::ItemFn, cfg: zyn::Args) -> proc_macro::TokenStream {
	let cfg = match CommandArgs::from_args(&cfg) {
		Ok(cfg) => cfg,
		Err(err) => bail!("{err}"),
	};
	plugin::command(item, cfg)
}

/// 命令参数宏
///
/// 用于为 [`command`] 定义的命令函数添加参数配置。
/// 可以在同一个函数上使用多个 `#[arg]` 属性来定义多个参数。
///
/// # 参数
///
/// | 参数 | 类型 | 必需 | 默认值 | 说明 |
/// |------|------|:----:|--------|------|
/// | `name` | `&str` | ✓ | - | 参数名称 |
/// | `type` | `&str` | | `"string"` | 参数类型 |
/// | `mode` | `&str` | | `"positional"` | 参数模式 |
/// | `required` | `bool` | | `false` | 是否必需 |
/// | `desc` | `&str` | | `None` | 参数描述 |
///
/// ## 参数类型
///
/// | 类型 | 说明 |
/// |------|------|
/// | `"string"` | 字符串类型（默认） |
/// | `"int"` | 整数类型 |
/// | `"float"` | 浮点数类型 |
/// | `"bool"` | 布尔类型 |
///
/// ## 参数模式
///
/// | 模式 | 说明 |
/// |------|------|
/// | `"positional"` | 位置参数（默认），按顺序匹配 |
/// | `"named"` | 命名参数，需要 `--flag value` 格式 |
///
/// # 示例
///
/// ## 基础用法
///
/// ```rust,ignore
/// #[command(name = "echo", desc = "回显消息")]
/// #[arg(name = "message", desc = "要回显的消息")]
/// async fn echo(ctx: &MessageContext) -> HandlerResult<CommandAction> {
///     let msg = ctx.arg("message").and_then(|v| v.as_str()).unwrap_or("");
///     ctx.reply(message!(segment!(text, msg))).await?;
///     Ok(().into())
/// }
/// ```
///
/// ## 必需参数
///
/// ```rust,ignore
/// #[command(name = "greet", desc = "打招呼")]
/// #[arg(name = "name", required = true, desc = "用户名")]
/// async fn greet(ctx: &MessageContext) -> HandlerResult<CommandAction> {
///     // 调用：!greet Alice
///     let name = ctx.arg("name").and_then(|v| v.as_str()).unwrap_or("");
///     ctx.reply(message!(segment!(text, format!("你好，{}！", name)))).await?;
///     Ok(().into())
/// }
/// ```
///
/// ## 不同类型的参数
///
/// ```rust,ignore
/// #[command(name = "add", desc = "加法计算")]
/// #[arg(name = "a", type = "int", required = true, desc = "第一个数")]
/// #[arg(name = "b", type = "int", required = true, desc = "第二个数")]
/// async fn add(ctx: &MessageContext) -> HandlerResult<CommandAction> {
///     // 调用：!add 10 20
///     let a = ctx.arg("a").and_then(|v| v.as_int()).unwrap_or(0);
///     let b = ctx.arg("b").and_then(|v| v.as_int()).unwrap_or(0);
///     ctx.reply(message!(segment!(text, format!("结果: {}", a + b)))).await?;
///     Ok(().into())
/// }
/// ```
///
/// ## 命名参数
///
/// ```rust,ignore
/// #[command(name = "repeat", desc = "重复消息")]
/// #[arg(name = "message", required = true, desc = "要重复的消息")]
/// #[arg(name = "count", type = "int", mode = "named", desc = "重复次数")]
/// async fn repeat(ctx: &MessageContext) -> HandlerResult<CommandAction> {
///     // 调用：!repeat hello --count 3
///     let message = ctx.arg("message").and_then(|v| v.as_str()).unwrap_or("");
///     let count = ctx.arg("count").and_then(|v| v.as_int()).unwrap_or(1);
///     
///     for _ in 0..count {
///         ctx.reply(message!(segment!(text, message))).await?;
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
/// async fn calc(ctx: &MessageContext) -> HandlerResult<CommandAction> {
///     // 调用：!calc 10 20 --op add
///     let a = ctx.arg("a").and_then(|v| v.as_int()).unwrap_or(0);
///     let b = ctx.arg("b").and_then(|v| v.as_int()).unwrap_or(0);
///     let op = ctx.arg("op").and_then(|v| v.as_str()).unwrap_or("add");
///     
///     let result = match op {
///         "add" => a + b,
///         "sub" => a - b,
///         "mul" => a * b,
///         "div" => a / b,
///         _ => 0,
///     };
///     
///     ctx.reply(message!(segment!(text, format!("结果: {}", result)))).await?;
///     Ok(().into())
/// }
/// ```
#[zyn::attribute(debug(pretty))]
pub fn arg(#[zyn(input)] item: zyn::syn::ItemFn) -> proc_macro::TokenStream {
	item.to_token_stream()
}

#[zyn::attribute(debug(pretty))]
pub fn task(#[zyn(input)] item: zyn::syn::ItemFn, args: zyn::Args) -> proc_macro::TokenStream {
	let cfg = match TaskArgs::from_args(&args) {
		Ok(cfg) => cfg,
		Err(err) => bail!("{err}"),
	};
	plugin::task(item, cfg)
}

#[zyn::attribute(debug(pretty))]
pub fn plugin_hook(
	#[zyn(input)] item: zyn::syn::ItemFn,
	args: zyn::Args,
) -> proc_macro::TokenStream {
	let cfg = match HookArgs::from_args(&args) {
		Ok(cfg) => cfg,
		Err(err) => bail!("{err}"),
	};
	plugin::hook(item, cfg)
}
