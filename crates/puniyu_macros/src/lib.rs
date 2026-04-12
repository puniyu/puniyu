//! # puniyu_macros
//!
//! `puniyu_macros` 提供 puniyu 生态中的过程宏，用于声明插件、命令、任务、配置和适配器入口。
//!
//! ## 插件侧宏
//!
//! - [`plugin`]：声明插件入口函数
//! - [`plugin_config`]：声明插件配置结构体
//! - [`plugin_hook`]：声明插件钩子函数
//! - [`command`]：声明命令处理函数
//! - [`arg`]：为命令补充参数描述
//! - [`task`]：声明定时任务函数
//!
//! ## 适配器侧宏
//!
//! - [`adapter`]：声明适配器入口函数
//! - [`adapter_config`]：声明适配器配置结构体
//! - [`adapter_hook`]：声明适配器钩子函数
//!
//! ## 示例
//!
//! ```rust, ignore
//! use puniyu_plugin::prelude::*;
//!
//! #[plugin]
//! async fn __main() {}
//! ```

mod adapter;
mod common;
mod plugin;
mod types;
pub(crate) use types::*;

use zyn::{ToTokens, syn::spanned::Spanned};

/// 声明适配器配置结构体。
///
/// 该宏会为结构体生成适配器配置注册逻辑，并使用结构体的默认值作为初始配置。
/// 默认配置名取结构体名的 snake_case，也可以通过 `name` 参数覆盖。
///
/// # 示例
///
/// ```rust, ignore
/// use puniyu_macros::adapter_config;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Default, Debug, Clone, Serialize, Deserialize)]
/// #[adapter_config(name = "console")]
/// pub struct ConsoleConfig {
///     pub enabled: bool,
///     pub prompt: String,
/// }
/// ```
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

/// 声明适配器钩子函数。
///
/// 要求函数为 `async`，接收一个引用参数，对应适配器侧 `HookType`，并返回
/// `puniyu_adapter::Result`。
///
/// # 示例
///
/// ```rust, ignore
/// use puniyu_adapter::hook::HookType;
/// use puniyu_macros::adapter_hook;
///
/// #[adapter_hook(name = "on_start", hook_type = "status.start", priority = 100)]
/// async fn on_start(event: &HookType) -> puniyu_adapter::Result {
///     let _ = event;
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

/// 声明适配器入口函数。
///
/// 参数中需要提供：
/// - `info`：返回 `AdapterInfo` 的函数
/// - `runtime`：返回适配器运行时的函数
///
/// 要求函数为 `async`。当函数体非空时，返回类型必须为 `puniyu_adapter::Result`；
/// 当函数体为空时，可以省略返回值类型。
///
/// # 示例
///
/// ```rust, ignore
/// use puniyu_adapter::types::*;
/// use puniyu_macros::adapter;
///
/// #[adapter(runtime = runtime::runtime)]
/// async fn main() -> puniyu_adapter::Result {
///     Ok(())
/// }
/// ```
#[zyn::attribute(debug(pretty))]
pub fn adapter(#[zyn(input)] item: zyn::syn::ItemFn, args: zyn::Args) -> proc_macro::TokenStream {
	let cfg = match AdapterArgs::from_args(&args) {
		Ok(cfg) => cfg,
		Err(err) => bail!("{err}"),
	};
	adapter::adapter(item, cfg)
}

/// 声明插件入口函数。
///
/// 可选参数：
/// - `desc`：插件描述
/// - `prefix`：插件命令前缀
///
/// 要求函数为 `async`。当函数体非空时，返回类型必须为 `puniyu_plugin::Result`；
/// 当函数体为空时，可以省略返回值类型。
///
/// # 示例
///
/// ```rust, ignore
/// use puniyu_macros::plugin;
/// use puniyu_plugin::prelude::*;
///
/// #[plugin(desc = "基础功能插件", prefix = "basic")]
/// async fn __main() -> puniyu_plugin::Result {
///     Ok(())
/// }
/// ```
#[zyn::attribute(debug(pretty))]
pub fn plugin(#[zyn(input)] item: zyn::syn::ItemFn, args: zyn::Args) -> proc_macro::TokenStream {
	let cfg = match PluginArg::from_args(&args) {
		Ok(cfg) => cfg,
		Err(err) => bail!("{err}"),
	};
	plugin::plugin(item, cfg)
}

/// 声明插件配置结构体。
///
/// 该宏会为结构体生成插件配置注册逻辑，并使用结构体的默认值作为初始配置。
/// 默认配置名取结构体名的 snake_case，也可以通过 `name` 参数覆盖。
///
/// # 示例
///
/// ```rust, ignore
/// use puniyu_macros::plugin_config;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Default, Debug, Clone, Serialize, Deserialize)]
/// #[plugin_config(name = "basic")]
/// pub struct BasicConfig {
///     pub enabled: bool,
///     pub prefix: String,
/// }
/// ```
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

/// 声明命令处理函数。
///
/// 常用参数：
/// - `name`：命令名称，必填
/// - `desc`：命令描述，可选
/// - `priority`：优先级，可选
/// - `alias`：别名列表，可选
/// - `permission`：权限，可选，支持 `"all"` 和 `"admin"`
///
/// 要求：
/// - 函数必须是 `async`
/// - 必须且只能接收一个参数：`&MessageContext<'_>`
/// - 返回类型必须为 `puniyu_plugin::Result<CommandAction>`
///
/// # 示例
///
/// ```rust, ignore
/// use puniyu_macros::{arg, command};
/// use puniyu_plugin::prelude::*;
///
/// #[command(name = "echo", desc = "回显文本", alias = ["say"], priority = 100)]
/// #[arg(name = "message", desc = "消息内容", required = true)]
/// #[arg(name = "times", r#type = "integer", mode = "optional", desc = "重复次数")]
/// async fn echo(ctx: &MessageContext<'_>) -> puniyu_plugin::Result<CommandAction> {
///     Ok(CommandAction::Done)
/// }
/// ```
#[zyn::attribute(debug(pretty))]
pub fn command(#[zyn(input)] item: zyn::syn::ItemFn, cfg: zyn::Args) -> proc_macro::TokenStream {
	let cfg = match CommandArgs::from_args(&cfg) {
		Ok(cfg) => cfg,
		Err(err) => bail!("{err}"),
	};
	plugin::command(item, cfg)
}

/// 为命令补充参数描述。
///
/// 该宏通常与 [`command`] 一起使用，常用参数包括：
/// - `name`：参数名
/// - `desc`：参数说明
/// - `type`：参数类型，支持 `string`、`integer`、`boolean`
/// - `mode`：参数模式，支持 `positional`、`optional`
/// - `required`：是否必填
///
/// # 示例
///
/// ```rust, ignore
/// use puniyu_macros::{arg, command};
/// use puniyu_plugin::prelude::*;
///
/// #[command(name = "echo")]
/// #[arg(name = "message", desc = "消息内容", required = true)]
/// #[arg(name = "times", r#type = "integer", mode = "optional", desc = "重复次数")]
/// #[arg(name = "silent", r#type = "boolean", mode = "optional", desc = "是否静默执行")]
/// async fn echo(ctx: &MessageContext<'_>) -> puniyu_plugin::Result<CommandAction> {
///     Ok(CommandAction::Done)
/// }
/// ```
#[zyn::attribute(debug(pretty))]
pub fn arg(#[zyn(input)] item: zyn::syn::ItemFn) -> proc_macro::TokenStream {
	item.to_token_stream()
}

/// 声明定时任务函数。
///
/// 常用参数：
/// - `cron`：cron 表达式，必填
/// - `name`：任务名，可选
///
/// 要求：
/// - 函数必须是 `async`
/// - 函数不接收参数
/// - 返回类型必须为 `puniyu_plugin::Result`
/// - `cron` 会在编译时校验
///
/// # 示例
///
/// ```rust, ignore
/// use puniyu_macros::task;
/// use puniyu_plugin::prelude::*;
///
/// #[task(cron = "*/5 * * * *", name = "health_check")]
/// async fn health_check() -> puniyu_plugin::Result {
///     Ok(())
/// }
/// ```
#[zyn::attribute(debug(pretty))]
pub fn task(#[zyn(input)] item: zyn::syn::ItemFn, args: zyn::Args) -> proc_macro::TokenStream {
	let cfg = match TaskArgs::from_args(&args) {
		Ok(cfg) => cfg,
		Err(err) => bail!("{err}"),
	};
	plugin::task(item, cfg)
}

/// 声明插件钩子函数。
///
/// 常用参数：
/// - `name`：钩子名称，可选
/// - `hook_type`：钩子类型，可选，如 `"event.message"`、`"status.start"`
/// - `priority`：优先级，可选
///
/// 要求函数为 `async`，接收一个引用参数，对应插件侧 `HookType`，并返回
/// `puniyu_plugin::Result`。
///
/// # 示例
///
/// ```rust, ignore
/// use puniyu_macros::plugin_hook;
/// use puniyu_plugin::hook::HookType;
///
/// #[plugin_hook(name = "on_message", hook_type = "event.message", priority = 100)]
/// async fn on_message(event: &HookType) -> puniyu_plugin::Result {
///     Ok(())
/// }
/// ```
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
