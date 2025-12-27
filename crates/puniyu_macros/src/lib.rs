use proc_macro::TokenStream;
use quote::quote;

#[cfg(any(feature = "plugin", feature = "command"))]
use syn::ItemFn;
#[cfg(any(feature = "command", feature = "task"))]
use syn::{Ident, parse_macro_input};

#[cfg(any(feature = "plugin", feature = "command", feature = "task"))]
mod plugin;

#[cfg(any(feature = "plugin", feature = "command", feature = "task"))]
use plugin::{CommandArgs, PluginArg, TaskArgs};

#[cfg(any(feature = "adapter", feature = "plugin"))]
fn parse_struct_input(item: TokenStream) -> syn::Result<syn::ItemStruct> {
	syn::parse(item).map_err(|e| syn::Error::new(e.span(), "呜哇~这个宏只能用在结构体上！杂鱼~"))
}

#[cfg(any(feature = "adapter", feature = "plugin"))]
fn parse_config_name(args: TokenStream, struct_name: &syn::Ident) -> syn::Result<String> {
	use convert_case::{Case, Casing};
	if args.is_empty() {
		Ok(struct_name.to_string().to_case(Case::Lower))
	} else {
		syn::parse::<syn::LitStr>(args)
			.map(|lit| lit.value())
			.map_err(|e| syn::Error::new(e.span(), "呜哇~配置名称必须是字符串字面量！杂鱼~"))
	}
}

#[cfg(feature = "adapter")]
#[proc_macro_attribute]
pub fn adapter_config(args: TokenStream, item: TokenStream) -> TokenStream {
	let input_struct = match parse_struct_input(item) {
		Ok(s) => s,
		Err(e) => return e.to_compile_error().into(),
	};

	let struct_name = &input_struct.ident;

	let config_name = match parse_config_name(args, struct_name) {
		Ok(name) => name,
		Err(e) => return e.to_compile_error().into(),
	};
	let adapter_name = env!("CARGO_PKG_NAME");

	let expanded = quote! {
		#input_struct

		impl ::puniyu_adapter::Config for #struct_name {
			fn name(&self) -> &'static str {
				#config_name
			}

			fn config(&self) -> ::puniyu_adapter::toml::Value {
				let config_str = ::puniyu_adapter::toml::to_string(&Self::default())
					.expect("Failed to serialize config");
				::puniyu_adapter::toml::from_str(&config_str)
					.expect("Failed to deserialize config")
			}
		}

		impl #struct_name {
			pub fn get() -> Self {
				use ::puniyu_adapter::AdapterBuilder;
				let adapter_name = crate::Adapter.name().to_lowercase();
				let path = ::puniyu_adapter::ADAPTER_CONFIG_DIR.join(adapter_name).join(format!("{}.toml", #config_name));
				::puniyu_adapter::ConfigRegistry::get(&path)
					.and_then(|cfg| cfg.try_into::<#struct_name>().ok())
					.unwrap_or_default()
			}
		}

		::puniyu_adapter::inventory::submit! {
			crate::ConfigRegistry {
				adapter_name: #adapter_name,
				builder: || -> Box<dyn ::puniyu_adapter::Config> {
					Box::new(#struct_name::default())
				}
			}
		}
	};

	TokenStream::from(expanded)
}

#[cfg(feature = "adapter")]
#[proc_macro_attribute]
pub fn adapter(_: TokenStream, item: TokenStream) -> TokenStream {
	let input_struct = match parse_struct_input(item) {
		Ok(s) => s,
		Err(e) => return e.to_compile_error().into(),
	};

	let struct_name = &input_struct.ident;
	let adapter_struct_name = Ident::new("Adapter", proc_macro2::Span::call_site());
	let version_major = quote! { env!("CARGO_PKG_VERSION_MAJOR").parse::<u16>().unwrap() };
	let version_minor = quote! { env!("CARGO_PKG_VERSION_MINOR").parse::<u16>().unwrap() };
	let version_patch = quote! { env!("CARGO_PKG_VERSION_PATCH").parse::<u16>().unwrap() };

	let expanded = quote! {
		#input_struct

		pub struct #adapter_struct_name;

		#[::puniyu_adapter::async_trait]
		impl ::puniyu_adapter::AdapterBuilder for #adapter_struct_name {
			fn name(&self) -> &'static str {
				::puniyu_adapter::AdapterBuilder::name(&#struct_name)
			}

			fn version(&self) -> ::puniyu_adapter::Version {
				::puniyu_adapter::Version {
					major: #version_major,
					minor: #version_minor,
					patch: #version_patch,
				}
			}

			fn api(&self) -> &'static dyn ::puniyu_adapter::AdapterApi {
				::puniyu_adapter::AdapterBuilder::api(&#struct_name)
			}

			fn config(&self) -> Option<Vec<Box<dyn ::puniyu_adapter::Config>>> {
				::puniyu_adapter::inventory::iter::<ConfigRegistry>
					.into_iter()
					.map(|registry| (registry.builder)())
					.collect::<Vec<_>>()
					.into()
			}

			fn server(&self) -> Option<::puniyu_adapter::ServerType> {
				::puniyu_adapter::AdapterBuilder::server(&#struct_name)
			}

			async fn init(&self) -> ::puniyu_adapter::Result<()> {
				::puniyu_adapter::AdapterBuilder::init(&#struct_name).await
			}
		}

		/// 配置注册表
		pub(crate) struct ConfigRegistry {
			adapter_name: &'static str,
			/// 配置构造器
			builder: fn() -> Box<dyn ::puniyu_adapter::Config>,
		}
		::puniyu_adapter::inventory::collect!(ConfigRegistry);
	};

	TokenStream::from(expanded)
}

#[cfg(feature = "plugin")]
#[proc_macro_attribute]
pub fn plugin_config(args: TokenStream, item: TokenStream) -> TokenStream {
	let input_struct = match parse_struct_input(item) {
		Ok(s) => s,
		Err(e) => return e.to_compile_error().into(),
	};

	let struct_name = &input_struct.ident;

	let config_name = match parse_config_name(args, struct_name) {
		Ok(name) => name,
		Err(e) => return e.to_compile_error().into(),
	};
	let plugin_name = quote! { env!("CARGO_PKG_NAME") };

	let expanded = quote! {
		#input_struct

		impl ::puniyu_plugin::Config for #struct_name {
			fn name(&self) -> &'static str {
				#config_name
			}

			fn config(&self) -> ::puniyu_plugin::toml::Value {
				let config_str = ::puniyu_plugin::toml::to_string(&Self::default())
					.expect("Failed to serialize config");
				::puniyu_plugin::toml::from_str(&config_str)
					.expect("Failed to deserialize config")
			}
		}

		impl #struct_name {
			pub fn get() -> Self {
				use ::puniyu_plugin::PluginBuilder;
				let plugin_name = crate::Plugin.name().to_lowercase();
				let path = ::puniyu_plugin::PLUGIN_CONFIG_DIR.join(plugin_name).join(format!("{}.toml", #config_name));
				::puniyu_plugin::ConfigRegistry::get(&path)
					.and_then(|cfg| cfg.try_into::<#struct_name>().ok())
					.unwrap_or_default()
			}
		}

		::puniyu_plugin::inventory::submit! {
			crate::ConfigRegistry {
				plugin_name: #plugin_name,
				builder: || -> Box<dyn ::puniyu_plugin::Config> {
					Box::new(#struct_name::default())
				}
			}
		}
	};

	TokenStream::from(expanded)
}

/// 注册插件
/// 此宏包含以下检测：
/// 1. 函数是否为异步函数
/// 2. 插件名称，插件版本，插件作者
/// 3. 插件名称是否规范
///
/// # 示例
/// ## 最小化示例（使用默认初始化）
/// ```rust, ignore
/// use puniyu_plugin::plugin;
///
/// #[plugin]
/// pub async fn hello() {}
/// ```
///
/// ## 完整示例（自定义初始化逻辑）
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
pub fn plugin(args: TokenStream, item: TokenStream) -> TokenStream {
	let input_fn = if let Ok(fn_item) = syn::parse::<ItemFn>(item.clone()) {
		fn_item
	} else {
		return syn::Error::new_spanned(
			proc_macro2::TokenStream::from(item),
			"呜哇~这个宏只能用在函数上，不能用在结构体！杂鱼~",
		)
		.to_compile_error()
		.into();
	};

	let fn_sig = &input_fn.sig;

	let fn_name = &input_fn.sig.ident;
	let fn_vis = &input_fn.vis;
	let fn_block = &input_fn.block;

	let plugin_args = parse_macro_input!(args as PluginArg);
	let plugin_desc = match &plugin_args.desc {
		Some(desc) => quote! { #desc },
		None => quote! { "这个人很懒，没有设置呢" },
	};
	let plugin_prefix = match &plugin_args.prefix {
		Some(prefix) => quote! { Some(#prefix) },
		None => quote! { None },
	};

	let is_async = fn_sig.asyncness.is_some();
	if !is_async {
		return syn::Error::new_spanned(fn_sig, "呜哇~杂鱼函数连async都不会用吗？杂鱼~")
			.to_compile_error()
			.into();
	}

	let plugin_name = quote! { env!("CARGO_PKG_NAME") };
	let version_major = quote! { env!("CARGO_PKG_VERSION_MAJOR").parse::<u16>().unwrap() };
	let version_minor = quote! { env!("CARGO_PKG_VERSION_MINOR").parse::<u16>().unwrap() };
	let version_patch = quote! { env!("CARGO_PKG_VERSION_PATCH").parse::<u16>().unwrap() };
	let version_string = quote! { env!("CARGO_PKG_VERSION") };
	let plugin_author = quote! {
		{
			let authors = env!("CARGO_PKG_AUTHORS");
			if authors.is_empty() { "Unknown" } else { authors }
		}
	};

	let struct_name = Ident::new("Plugin", fn_name.span());

	let init_call = if fn_block.stmts.is_empty() {
		quote! {
			async {
				puniyu_plugin::logger::info!(
					"{} v{} 初始化完成",
					#plugin_name,
					#version_string,
				);
				Ok(())
			}
		}
	} else {
		quote! {
			async {
				#fn_name().await
			}
		}
	};

	let expanded = quote! {
		pub struct #struct_name;

		#fn_vis #fn_sig #fn_block

		#[::puniyu_plugin::async_trait]
		impl ::puniyu_plugin::PluginBuilder for #struct_name {
			fn name(&self) -> &'static str {
				#plugin_name
			}

			fn version(&self) -> ::puniyu_plugin::Version {
				::puniyu_plugin::Version {
					major: #version_major,
					minor: #version_minor,
					patch: #version_patch,
				}
			}

			fn author(&self) -> &'static str {
				#plugin_author
			}

			fn abi_version(&self) -> ::puniyu_plugin::Version {
				::puniyu_plugin::ABI_VERSION
			}

			fn description(&self) -> &'static str {
				#plugin_desc
			}

			fn prefix(&self) -> Option<&'static str> {
				#plugin_prefix
			}

			fn tasks(&self) -> Vec<Box<dyn ::puniyu_plugin::TaskBuilder>> {
				let plugin_name = self.name();
				::puniyu_plugin::inventory::iter::<TaskRegistry>
					.into_iter()
					.filter(|task| task.plugin_name == plugin_name)
					.map(|task| (task.builder)())
					.collect()
			}

			fn commands(&self) -> Vec<Box<dyn ::puniyu_plugin::CommandBuilder>> {
				let plugin_name = self.name();
				::puniyu_plugin::inventory::iter::<CommandRegistry>
					.into_iter()
					.filter(|command| command.plugin_name == plugin_name)
					.map(|command| (command.builder)())
					.collect()
			}

			fn config(&self) -> Option<Vec<Box<dyn ::puniyu_plugin::Config>>> {
				let plugin_name = self.name();
				let configs: Vec<_> = ::puniyu_plugin::inventory::iter::<ConfigRegistry>
					.into_iter()
					.filter(|config| config.plugin_name == plugin_name)
					.map(|config| (config.builder)())
					.collect();
				if configs.is_empty() {
					None
				} else {
					Some(configs)
				}
			}

			fn server(&self) -> Option<::puniyu_plugin::ServerType> {
				let plugin_name = self.name();
				let servers: Vec<_> = ::puniyu_plugin::inventory::iter::<ServerRegistry>
					.into_iter()
					.filter(|server| server.plugin_name == plugin_name)
					.map(|server| (server.builder)())
					.collect();

				if !servers.is_empty() {
					Some(::std::sync::Arc::new(move |cfg: &mut ::puniyu_plugin::actix_web::web::ServiceConfig| {
						servers.iter().for_each(|server| server(cfg));
					}))
				} else {
					None
				}
			}

			async fn init(&self) -> ::std::result::Result<(), Box<dyn std::error::Error>> {
				#init_call.await
			}
		}

		/// 插件注册表
		pub(crate) struct PluginRegistry {
			/// 插件构造器
			builder: fn() -> Box<dyn ::puniyu_plugin::PluginBuilder>,
		}
		::puniyu_plugin::inventory::collect!(PluginRegistry);

		/// 定时计划注册表
		pub(crate) struct TaskRegistry {
			/// 插件名称
			plugin_name: &'static str,
			/// 任务构造器
			builder: fn() -> Box<dyn ::puniyu_plugin::TaskBuilder>,
		}
		::puniyu_plugin::inventory::collect!(TaskRegistry);

		pub(crate) struct CommandRegistry {
			plugin_name: &'static str,
			/// 命令构造器
			builder: fn() -> Box<dyn ::puniyu_plugin::CommandBuilder>,
		}
		::puniyu_plugin::inventory::collect!(CommandRegistry);

		/// 配置注册表
		pub(crate) struct ConfigRegistry {
			plugin_name: &'static str,
			/// 配置构造器
			builder: fn() -> Box<dyn ::puniyu_plugin::Config>,
		}
		::puniyu_plugin::inventory::collect!(ConfigRegistry);

		/// 服务器注册表
		pub(crate) struct ServerRegistry {
			/// 插件名称
			plugin_name: &'static str,
			/// 服务器配置构造器
			builder: fn() -> ::puniyu_plugin::ServerType,
		}
		::puniyu_plugin::inventory::collect!(ServerRegistry);

		::puniyu_plugin::inventory::submit! {
			PluginRegistry {
				builder: || -> Box<dyn ::puniyu_plugin::PluginBuilder> { Box::new(#struct_name {}) },
			}
		}

		#[cfg(feature = "cdylib")]
		#[unsafe(no_mangle)]
		pub unsafe extern "C" fn plugin_info() -> *mut dyn ::puniyu_plugin::PluginBuilder {
			Box::into_raw(Box::new(#struct_name {}))
		}

		#[cfg(feature = "cdylib")]
		#[unsafe(no_mangle)]
		pub unsafe extern "C" fn setup_logger(logger: &::puniyu_plugin::logger::SharedLogger) {
			::puniyu_plugin::logger::setup_shared_logger(logger);
		}

		#[cfg(feature = "cdylib")]
		#[unsafe(no_mangle)]
		pub unsafe extern "C" fn setup_app_name(name: String) {
			::puniyu_plugin::APP_NAME.get_or_init(|| name);
		}

		#[cfg(feature = "cdylib")]
		#[unsafe(no_mangle)]
		pub unsafe extern "C" fn setup_event_bus(bus: ::std::sync::Arc<::puniyu_plugin::EventBus>) {
			::puniyu_plugin::EVENT_BUS.get_or_init(|| bus);
		}

	};

	TokenStream::from(expanded)
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
/// | `desc` | `&str` | | `""` | 命令描述 |
/// | `rank` | `u64` | | `500` | 优先级，数值越小优先级越高 |
/// | `alias` | `[&str]` | | `[]` | 命令别名列表 |
/// | `args` | `[Arg]` | | `[]` | 命令参数列表 |
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
///     HandlerAction::done()
/// }
/// ```
///
/// # 命令参数
///
/// 支持三种格式定义参数：
///
/// ## 1. 简单格式
///
/// 只指定参数名称，默认为可选的字符串类型位置参数：
///
/// ```rust,ignore
/// args = ["message", "count"]
/// // 调用：!echo hello 3
/// // message = "hello", count = "3"
/// ```
///
/// ## 2. 元组格式
///
/// ```rust,ignore
/// // (name, type, required, default, desc, mode)
/// args = [("count", "int", false, 1, "重复次数", "named")]
/// ```
///
/// ## 3. 对象格式
///
/// ```rust,ignore
/// args = [{ name = "count", arg_type = "int", mode = "named", default = 1 }]
/// ```
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
/// #[command(name = "echo", desc = "回显消息", args = ["message"])]
/// async fn echo(bot: &BotContext, ev: &MessageContext) -> HandlerResult {
///     // 调用：!echo hello
///     let msg = ev.arg("message").and_then(|v| v.as_str()).unwrap_or("");
///     bot.reply(msg.into()).await?;
///     HandlerAction::done()
/// }
/// ```
///
/// ## 带别名的命令
///
/// ```rust,ignore
/// #[command(name = "ping", alias = ["p"], desc = "测试延迟")]
/// async fn ping(bot: &BotContext, ev: &MessageContext) -> HandlerResult {
///     // !ping 或 !p 都可以触发
///     bot.reply("pong!".into()).await?;
///     HandlerAction::done()
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
///     bot.reply("配置已重载".into()).await?;
///     HandlerAction::done()
/// }
/// ```
///
/// ## 混合参数类型
///
/// ```rust,ignore
/// #[command(
///     name = "repeat",
///     alias = ["r"],
///     desc = "重复消息",
///     args = [
///         "message",  // 位置参数
///         { name = "count", arg_type = "int", mode = "named", default = 1 },
///     ]
/// )]
/// async fn repeat(bot: &BotContext, ev: &MessageContext) -> HandlerResult {
///     // 调用：!repeat hello --count 3
///     let message = ev.arg("message").and_then(|v| v.as_str()).unwrap_or("");
///     let count = ev.arg("count").and_then(|v| v.as_int()).unwrap_or(1);
///     for _ in 0..count {
///         bot.reply(message.into()).await?;
///     }
///     HandlerAction::done()
/// }
/// ```
#[cfg(feature = "command")]
#[proc_macro_attribute]
pub fn command(args: TokenStream, item: TokenStream) -> TokenStream {
	use convert_case::{Case, Casing};
	let args = parse_macro_input!(args as CommandArgs);
	let input_fn = parse_macro_input!(item as ItemFn);
	let fn_name = &input_fn.sig.ident;
	let fn_vis = &input_fn.vis;
	let fn_sig = &input_fn.sig;
	let fn_block = &input_fn.block;

	let is_async = input_fn.sig.asyncness.is_some();
	if !is_async {
		return syn::Error::new_spanned(&input_fn.sig, "呜哇~杂鱼函数连async都不会用吗？杂鱼~")
			.to_compile_error()
			.into();
	}

	if input_fn.sig.inputs.len() != 2 {
		return syn::Error::new_spanned(
			&input_fn.sig.inputs,
			"呜哇~命令函数必须有两个参数：&BotContext, &MessageContext！杂鱼~",
		)
		.to_compile_error()
		.into();
	}

	let mut params = input_fn.sig.inputs.iter();

	if let Some(syn::FnArg::Typed(pat_type)) = params.next() {
		let ty = &*pat_type.ty;
		let ty_str = quote!(#ty).to_string();
		if !ty_str.contains("BotContext") {
			return syn::Error::new_spanned(ty, "呜哇~第一个参数必须是 &BotContext 类型！杂鱼~")
				.to_compile_error()
				.into();
		}
	}
	if let Some(syn::FnArg::Typed(pat_type)) = params.next() {
		let ty = &*pat_type.ty;
		let ty_str = quote!(#ty).to_string();
		if !ty_str.contains("MessageContext") {
			return syn::Error::new_spanned(
				ty,
				"呜哇~第二个参数必须是 &MessageContext 类型！杂鱼~",
			)
			.to_compile_error()
			.into();
		}
	}

	let struct_name_str = {
		let fn_name_str = fn_name.to_string();
		let pascal_case_name = fn_name_str.to_case(Case::Pascal);
		format!("{}Command", pascal_case_name)
	};

	let command_name = &args.name;
	let command_rank = &args.rank;
	let command_desc = &args.desc;
	let command_permission = &args.permission;
	let command_alias = if args.alias.is_empty() {
		quote! { None }
	} else {
		let aliases = &args.alias;
		quote! { Some(vec![#(#aliases),*]) }
	};
	let mut arg_defs: Vec<proc_macro2::TokenStream> = Vec::new();

	for arg_def in &args.args {
		arg_defs.push(arg_def.to_tokens());
	}

	let plugin_name = quote! { env!("CARGO_PKG_NAME") };

	let struct_name = Ident::new(&struct_name_str, fn_name.span());

	let expanded = quote! {
		pub struct #struct_name;

		#fn_vis #fn_sig #fn_block

		#[::puniyu_plugin::async_trait]
		impl ::puniyu_plugin::CommandBuilder for #struct_name {
			fn name(&self) -> &'static str {
				#command_name
			}

			fn description(&self) -> Option<&'static str> {
				if #command_desc.is_empty() {
					None
				} else {
					Some(#command_desc)
				}
			}

			fn args(&self) -> Vec<::puniyu_plugin::Arg> {
				vec![#(#arg_defs),*]
			}

			fn rank(&self) -> u64 {
				#command_rank.to_string().parse().unwrap_or(100)
			}

			fn alias(&self) -> Option<Vec<&'static str>> {
				#command_alias
			}

			fn permission(&self) -> ::puniyu_plugin::Permission {
				#command_permission.parse().unwrap_or_default()
			}

			async fn run(&self, bot: &::puniyu_plugin::BotContext, ev: &::puniyu_plugin::MessageContext) -> ::puniyu_plugin::HandlerResult {
				#fn_name(bot, ev).await
			}
		}

		::puniyu_plugin::inventory::submit! {
			crate::CommandRegistry {
				plugin_name: #plugin_name,
				builder: || -> Box<dyn ::puniyu_plugin::CommandBuilder> { Box::new(#struct_name {}) },
			}
		}
	};

	TokenStream::from(expanded)
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
pub fn task(args: TokenStream, item: TokenStream) -> TokenStream {
	use convert_case::{Case, Casing};
	use croner::Cron;
	use std::str::FromStr;
	let args = parse_macro_input!(args as TaskArgs);
	let input_fn = parse_macro_input!(item as ItemFn);
	let fn_name = &input_fn.sig.ident;
	let fn_vis = &input_fn.vis;
	let fn_sig = &input_fn.sig;
	let fn_block = &input_fn.block;

	let is_async = input_fn.sig.asyncness.is_some();
	if !is_async {
		return syn::Error::new_spanned(&input_fn.sig, "呜哇~杂鱼函数连async都不会用吗？杂鱼~")
			.to_compile_error()
			.into();
	}

	if !input_fn.sig.inputs.is_empty() {
		return syn::Error::new_spanned(
			&input_fn.sig.inputs,
			"呜哇~杂鱼函数居然还想带参数？不行不行！杂鱼~",
		)
		.to_compile_error()
		.into();
	}

	let cron_value = args.cron.value();
	if Cron::from_str(&cron_value).is_err() {
		return syn::Error::new_spanned(&args.cron, "呜哇~cron表达式都不会写吗？杂鱼~")
			.to_compile_error()
			.into();
	}

	let plugin_name = quote! { env!("CARGO_PKG_NAME") };

	let cron_expr = &args.cron;

	let task_name =
		if args.name.value().is_empty() { fn_name.to_string() } else { args.name.value() };
	let struct_name_str = {
		let fn_name_str = fn_name.to_string();
		let pascal_case_name = fn_name_str.to_case(Case::Pascal);
		format!("{}Task", pascal_case_name)
	};
	let struct_name = Ident::new(&struct_name_str, fn_name.span());

	let expanded = quote! {
		pub struct #struct_name;

		#fn_vis #fn_sig #fn_block

		#[::puniyu_plugin::async_trait]
		impl ::puniyu_plugin::TaskBuilder for #struct_name {
			fn name(&self) -> &'static str {
				#task_name
			}

			fn cron(&self) -> &'static str {
				#cron_expr
			}

			async fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
				#fn_name().await
			}
		}

		::puniyu_plugin::inventory::submit! {
			crate::TaskRegistry  {
				plugin_name: #plugin_name,
				builder: || -> Box<dyn ::puniyu_plugin::TaskBuilder> { Box::new(#struct_name {}) },
			}
		}
	};

	TokenStream::from(expanded)
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
#[cfg(feature = "plugin")]
#[proc_macro_attribute]
pub fn server(_args: TokenStream, item: TokenStream) -> TokenStream {
	let input_fn = if let Ok(fn_item) = syn::parse::<ItemFn>(item.clone()) {
		fn_item
	} else {
		return syn::Error::new_spanned(
			proc_macro2::TokenStream::from(item),
			"呜哇~这个宏只能用在函数上！杂鱼~",
		)
		.to_compile_error()
		.into();
	};

	let fn_name = &input_fn.sig.ident;
	let fn_vis = &input_fn.vis;
	let fn_sig = &input_fn.sig;
	let fn_block = &input_fn.block;

	if input_fn.sig.inputs.len() != 1 {
		return syn::Error::new_spanned(
			&input_fn.sig,
			"呜哇~函数必须接收一个参数 &mut ServiceConfig！杂鱼~",
		)
		.to_compile_error()
		.into();
	}

	let plugin_name = quote! { env!("CARGO_PKG_NAME") };

	let expanded = quote! {
		#fn_vis #fn_sig #fn_block

		::puniyu_plugin::inventory::submit! {
			crate::ServerRegistry {
				plugin_name: #plugin_name,
				builder: || -> ::puniyu_plugin::ServerType { ::std::sync::Arc::new(#fn_name) },
			}
		}
	};

	TokenStream::from(expanded)
}
