use proc_macro::TokenStream;
use quote::quote;
#[cfg(any(feature = "plugin", feature = "command", feature = "adapter"))]
use syn::ItemFn;
#[cfg(any(feature = "command", feature = "task"))]
use syn::{Token, parse::Parse, parse::ParseStream, parse_macro_input, punctuated::Punctuated};

#[cfg(any(feature = "command", feature = "task"))]
use syn::Ident;

#[cfg(feature = "adapter")]
#[proc_macro_attribute]
pub fn adapter_config(args: TokenStream, item: TokenStream) -> TokenStream {
		use convert_case::{Case, Casing};
	let input_struct = if let Ok(struct_item) = syn::parse::<syn::ItemStruct>(item.clone()) {
		struct_item
	} else {
		return syn::Error::new_spanned(
			proc_macro2::TokenStream::from(item),
			"这个宏只能用在结构体上！",
		)
		.to_compile_error()
		.into();
	};

	let struct_name = &input_struct.ident;

	let config_name = if args.is_empty() {
		struct_name.to_string().to_case(Case::Lower)
	} else {
		let name_lit: syn::LitStr = syn::parse(args).expect("配置名称必须是字符串字面量");
		name_lit.value()
	};
	let adapter_name = env!("CARGO_PKG_NAME");

	let expanded = quote! {
		#input_struct

		impl ::puniyu_adapter::Config for #struct_name {
			fn name(&self) -> &'static str {
				#config_name
			}

			fn config(&self) -> ::puniyu_adapter::toml::Value {
				::puniyu_adapter::toml::to_value(Self::default())
					.unwrap_or(::puniyu_adapter::toml::Value::Null)
			}
		}

		impl #struct_name {
			pub fn get() -> Self {
				::puniyu_adapter::read_config::<#struct_name>(::puniyu_adapter::ADAPTER_CONFIG_DIR.as_path(), #config_name).unwrap_or_default()
			}
		}

		::puniyu_core::inventory::submit! {
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
	let input_struct = if let Ok(struct_item) = syn::parse::<syn::ItemStruct>(item.clone()) {
		struct_item
	} else {
		return syn::Error::new_spanned(
			proc_macro2::TokenStream::from(item),
			"这个宏只能用在结构体上！",
		)
		.to_compile_error()
		.into();
	};

	let struct_name = &input_struct.ident;
	let adapter_struct_name = Ident::new("Adapter", proc_macro2::Span::call_site());

	let expanded = quote! {
		#input_struct

		pub struct #adapter_struct_name;

		#[::puniyu_core::async_trait]
		impl ::puniyu_adapter::AdapterBuilder for #adapter_struct_name {
			fn info(&self) -> ::puniyu_adapter::AdapterInfo {
				::puniyu_adapter::AdapterBuilder::info(&#struct_name)
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
		::puniyu_core::inventory::collect!(ConfigRegistry);
	};

	TokenStream::from(expanded)
}

#[cfg(feature = "plugin")]
#[proc_macro_attribute]
pub fn plugin_config(args: TokenStream, item: TokenStream) -> TokenStream {
			use convert_case::{Case, Casing};
	let input_struct = if let Ok(struct_item) = syn::parse::<syn::ItemStruct>(item.clone()) {
		struct_item
	} else {
		return syn::Error::new_spanned(
			proc_macro2::TokenStream::from(item),
			"这个宏只能用在结构体上！",
		)
		.to_compile_error()
		.into();
	};

	let struct_name = &input_struct.ident;

	let config_name = if args.is_empty() {
		struct_name.to_string().to_case(Case::Lower)
	} else {
		let name_lit: syn::LitStr = syn::parse(args).expect("配置名称必须是字符串字面量");
		name_lit.value()
	};
	let plugin_name = env!("CARGO_PKG_NAME");

	let expanded = quote! {
		#input_struct

		impl ::puniyu_plugin::Config for #struct_name {
			fn name(&self) -> &'static str {
				#config_name
			}

			fn config(&self) -> ::puniyu_plugin::toml::Value {
				::puniyu_plugin::toml::to_value(Self::default())
					.unwrap_or(::puniyu_plugin::toml::Value::Null)
			}
		}

		impl #struct_name {
			pub fn get() -> Self {
				::puniyu_plugin::read_config::<#struct_name>(::puniyu_plugin::PLUGIN_CONFIG_DIR.as_path(), #config_name).unwrap_or_default()
			}
		}

		::puniyu_core::inventory::submit! {
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

#[cfg(feature = "plugin")]
#[derive(Default)]
struct PluginArg {
	desc: Option<syn::LitStr>,
}

#[cfg(feature = "plugin")]
impl Parse for PluginArg {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		if input.is_empty() {
			return Ok(PluginArg::default());
		}

		let fields = Punctuated::<syn::MetaNameValue, Token![,]>::parse_terminated(input)?;
		let mut args = PluginArg::default();

		for field in fields {
			let key = field
				.path
				.get_ident()
				.ok_or_else(|| syn::Error::new_spanned(&field.path, "呜哇~不支持的字段名！杂鱼~"))?
				.to_string();

			match key.as_str() {
				"desc" => {
					if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. }) =
						&field.value
					{
						args.desc = Some(lit_str.clone());
					} else {
						return Err(syn::Error::new_spanned(
							&field.value,
							"呜哇~desc 必须是字符串！杂鱼~",
						));
					}
				}
				_ => {
					return Err(syn::Error::new_spanned(
						&field.path,
						format!("呜哇~不支持的字段 '{}'！杂鱼~", key),
					));
				}
			}
		}

		Ok(args)
	}
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
			"杂鱼！这个宏只能用在函数上，不能用在结构体！笨蛋！",
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

	let is_async = fn_sig.asyncness.is_some();
	if !is_async {
		return syn::Error::new_spanned(fn_sig, "诶嘿~杂鱼函数连async都不会用吗？")
			.to_compile_error()
			.into();
	}

	let plugin_name = env!("CARGO_PKG_NAME");
	let version_major = env!("CARGO_PKG_VERSION_MAJOR");
	let version_minor = env!("CARGO_PKG_VERSION_MINOR");
	let version_patch = env!("CARGO_PKG_VERSION_PATCH");
	let version_string = env!("CARGO_PKG_VERSION");
	let plugin_author = {
		let authors = env!("CARGO_PKG_AUTHORS");
		if authors.is_empty() { "Unknown" } else { authors }
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

		use puniyu_plugin::{APP_NAME, async_trait};

		#[async_trait]
		impl ::puniyu_plugin::PluginBuilder for #struct_name {
			fn name(&self) -> &'static str {
				#plugin_name
			}

			fn version(&self) -> ::puniyu_plugin::Version {
				Version {
					major: #version_major.to_string(),
					minor: #version_minor.to_string(),
					patch: #version_patch.to_string(),
				}
			}

			fn author(&self) -> &'static str {
				#plugin_author
			}

			fn abi_version(&self) -> &'static str {
				::puniyu_plugin::ABI_VERSION
			}

			fn description(&self) -> &'static str {
				#plugin_desc
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
		::puniyu_core::inventory::collect!(TaskRegistry);

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
		pub extern "C" fn setup_app_name(name: String) {
			 ::puniyu_plugin::APP_NAME.get_or_init(|| name);
		}

	};

	TokenStream::from(expanded)
}

#[cfg(feature = "command")]
struct CommandArgs {
	name: syn::LitStr,
	args: syn::ExprArray,
	rank: syn::LitInt,
	desc: syn::LitStr,
}

#[cfg(feature = "command")]
impl CommandArgs {
	pub fn set_name(args: &mut Self, value: syn::LitStr) -> syn::Result<()> {
		args.name = value;
		Ok(())
	}

	pub fn set_args(args: &mut Self, value: syn::ExprArray) -> syn::Result<()> {
		args.args = value;
		Ok(())
	}

	pub fn set_rank(args: &mut Self, value: syn::LitInt) -> syn::Result<()> {
		args.rank = value;
		Ok(())
	}

	pub fn set_desc(args: &mut Self, value: syn::LitStr) -> syn::Result<()> {
		args.desc = value;
		Ok(())
	}
}

#[cfg(feature = "command")]
impl Default for CommandArgs {
	fn default() -> Self {
		Self {
			name: syn::LitStr::new("", proc_macro2::Span::call_site()),
			args: syn::ExprArray {
				attrs: Vec::new(),
				bracket_token: syn::token::Bracket::default(),
				elems: Punctuated::new(),
			},
			rank: syn::LitInt::new("100", proc_macro2::Span::call_site()),
			desc: syn::LitStr::new("", proc_macro2::Span::call_site()),
		}
	}
}

#[cfg(feature = "command")]
impl Parse for CommandArgs {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		if input.is_empty() {
			return Err(syn::Error::new(input.span(), "呜~至少给人家一些参数嘛！杂鱼~"));
		}

		let fields = Punctuated::<syn::MetaNameValue, Token![,]>::parse_terminated(input)?;
		let mut args = CommandArgs::default();
		for field in fields {
			let key = field
				.path
				.get_ident()
				.ok_or_else(|| syn::Error::new_spanned(&field.path, "呜哇~不支持的字段名！杂鱼~"))?
				.to_string();

			match key.as_str() {
				"name" => {
					if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. }) =
						&field.value
					{
						CommandArgs::set_name(&mut args, lit_str.clone())?;
					} else {
						return Err(syn::Error::new_spanned(
							&field.value,
							"呜哇~name 必须是字符串！杂鱼~",
						));
					}
				}
				"desc" => {
					if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. }) =
						&field.value
					{
						CommandArgs::set_desc(&mut args, lit_str.clone())?;
					} else {
						return Err(syn::Error::new_spanned(
							&field.value,
							"呜哇~desc 必须是字符串！杂鱼~",
						));
					}
				}
				"rank" => {
					if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(lit_int), .. }) =
						&field.value
					{
						CommandArgs::set_rank(&mut args, lit_int.clone())?;
					} else {
						return Err(syn::Error::new_spanned(
							&field.value,
							"呜哇~rank 必须是整数！杂鱼~",
						));
					}
				}
				"args" => {
					if let syn::Expr::Array(arr) = &field.value {
						CommandArgs::set_args(&mut args, arr.clone())?;
					} else {
						return Err(syn::Error::new_spanned(
							&field.value,
							"呜哇~args 必须是数组！杂鱼~",
						));
					}
				}
				_ => {
					return Err(syn::Error::new_spanned(
						&field.path,
						format!("呜哇~不支持的字段 '{}'！杂鱼~", key),
					));
				}
			}
		}

		if args.name.value().is_empty() {
			return Err(syn::Error::new(input.span(), "诶嘿~name都不给！杂鱼程序员！"));
		}

		Ok(args)
	}
}

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
		return syn::Error::new_spanned(&input_fn.sig, "诶嘿~杂鱼函数连async都不会用吗？")
			.to_compile_error()
			.into();
	}

	if input_fn.sig.inputs.len() != 2 {
		return syn::Error::new_spanned(
			&input_fn.sig.inputs,
			"呜哇~命令函数必须有两个参数：&BotContext, &MessageContext！笨蛋！",
		)
		.to_compile_error()
		.into();
	}

	let struct_name_str = {
		let fn_name_str = fn_name.to_string();
		let pascal_case_name = fn_name_str.to_case(Case::Pascal);
		format!("{}Command", pascal_case_name)
	};

	let command_name = &args.name;
	let command_args: Vec<syn::LitStr> = args
		.args
		.elems
		.iter()
		.filter_map(|expr| {
			if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. }) = expr {
				Some(lit_str.clone())
			} else {
				None
			}
		})
		.collect();
	let command_rank = &args.rank;
	let command_desc = &args.desc;

	let plugin_name = env!("CARGO_PKG_NAME");

	let struct_name = Ident::new(&struct_name_str, fn_name.span());

	let expanded = quote! {
		pub struct #struct_name;

		#fn_vis #fn_sig #fn_block

		use puniyu_core::async_trait;

		#[async_trait]
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

			fn args(&self) -> Vec<String> {
				vec![#(#command_args.to_string()),*]
			}

			fn rank(&self) -> u64 {
				#command_rank.to_string().parse().unwrap_or(100)
			}

			async fn run(&self, bot: &BotContext, ev: &MessageContext) -> ::puniyu_plugin::HandlerResult {
				#fn_name(bot, ev).await
			}
		}

		::puniyu_core::inventory::submit! {
			crate::CommandRegistry {
				plugin_name: #plugin_name,
				builder: || -> Box<dyn ::puniyu_plugin::CommandBuilder> { Box::new(#struct_name {}) },
			}
		}
	};

	TokenStream::from(expanded)
}

#[cfg(feature = "task")]
struct TaskArgs {
	name: syn::LitStr,
	cron: syn::LitStr,
}

#[cfg(feature = "task")]
impl Default for TaskArgs {
	fn default() -> Self {
		Self {
			cron: syn::LitStr::new("", proc_macro2::Span::call_site()),
			name: syn::LitStr::new("", proc_macro2::Span::call_site()),
		}
	}
}

#[cfg(feature = "task")]
impl TaskArgs {
	fn set_cron(args: &mut Self, value: syn::LitStr) -> syn::Result<()> {
		args.cron = value;
		Ok(())
	}

	fn set_name(args: &mut Self, value: syn::LitStr) -> syn::Result<()> {
		args.name = value;
		Ok(())
	}
}

#[cfg(feature = "task")]
impl Parse for TaskArgs {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		if input.is_empty() {
			return Err(syn::Error::new(input.span(), "呜~至少给人家一个cron表达式嘛！杂鱼~"));
		}

		let fields = Punctuated::<syn::MetaNameValue, Token![,]>::parse_terminated(input)?;
		let mut args = TaskArgs::default();

		for field in fields {
			let key = field
				.path
				.get_ident()
				.ok_or_else(|| syn::Error::new_spanned(&field.path, "呜哇~不支持的字段名！杂鱼~"))?
				.to_string();

			match key.as_str() {
				"cron" => {
					if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. }) =
						&field.value
					{
						TaskArgs::set_cron(&mut args, lit_str.clone())?;
					} else {
						return Err(syn::Error::new_spanned(
							&field.value,
							"呜哇~cron 必须是字符串！杂鱼~",
						));
					}
				}
				"name" => {
					if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. }) =
						&field.value
					{
						TaskArgs::set_name(&mut args, lit_str.clone())?;
					} else {
						return Err(syn::Error::new_spanned(
							&field.value,
							"呜哇~name 必须是字符串！杂鱼~",
						));
					}
				}
				_ => {
					return Err(syn::Error::new_spanned(
						&field.path,
						format!("呜哇~不支持的字段 '{}'！杂鱼~", key),
					));
				}
			}
		}

		Ok(args)
	}
}

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
		return syn::Error::new_spanned(&input_fn.sig, "诶嘿~杂鱼函数连async都不会用吗？")
			.to_compile_error()
			.into();
	}

	if !input_fn.sig.inputs.is_empty() {
		return syn::Error::new_spanned(
			&input_fn.sig.inputs,
			"呜哇~杂鱼函数居然还想带参数？不行不行！",
		)
		.to_compile_error()
		.into();
	}

	let cron_value = args.cron.value();
	if Cron::from_str(&cron_value).is_err() {
		return syn::Error::new_spanned(&args.cron, "呜哇~, cron表达式都不会写吗？真是杂鱼呢~")
			.to_compile_error()
			.into();
	}

	let plugin_name = env!("CARGO_PKG_NAME");

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

		use puniyu_core::async_trait;

		#[async_trait]
		impl ::puniyu_plugin::TaskBuilder for #struct_name {
			fn name(&self) -> &'static str {
				#task_name
			}

			fn cron(&self) -> &'static str {
				#cron_expr
			}

			async fn run(&self) {
				#fn_name().await;
			}
	}

	::puniyu_core::inventory::submit! {
		crate::TaskRegistry  {
			plugin_name: #plugin_name,
			builder: || -> Box<dyn ::puniyu_plugin::TaskBuilder> { Box::new(#struct_name {}) },
		}
	}

	};

	TokenStream::from(expanded)
}

/// 注册服务路由
/// 用于实现 Plugin trait 中的 server 方法
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
			"杂鱼！这个宏只能用在函数上！",
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
			"呜哇~函数必须接收一个参数 &mut ServiceConfig！",
		)
		.to_compile_error()
		.into();
	}

	let plugin_name = env!("CARGO_PKG_NAME");

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
