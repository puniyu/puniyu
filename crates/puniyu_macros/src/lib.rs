mod common;
use proc_macro::TokenStream;
use quote::quote;
use std::env;
#[cfg(feature = "plugin")]
use syn::ItemFn;

#[cfg(any(feature = "command", feature = "task"))]
use syn::{Token, parse::Parse, parse::ParseStream, parse_macro_input, punctuated::Punctuated};

#[cfg(any(feature = "command", feature = "task"))]
use syn::Ident;

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

	let adapter_name = match env::var("ADAPTER_NAME") {
		Ok(name) => name,
		Err(_) => {
			return syn::Error::new_spanned(
				&input_struct,
				"呜哇~ADAPTER_NAME都没有设置！杂鱼程序员！",
			)
			.to_compile_error()
			.into();
		}
	};
	let _ = match env::var("ADAPTER_VERSION") {
		Ok(version) => version,
		Err(_) => {
			return syn::Error::new_spanned(
				&input_struct,
				"呜哇~ADAPTER_VERSION都没有设置！杂鱼程序员！",
			)
			.to_compile_error()
			.into();
		}
	};
	let _ = match env::var("ADAPTER_AUTHOR") {
		Ok(author) => quote! { #author },
		Err(_) => {
			return syn::Error::new_spanned(
				&input_struct,
				"呜哇~ADAPTER_AUTHOR都没有设置！杂鱼程序员！",
			)
			.to_compile_error()
			.into();
		}
	};

	let struct_name = &input_struct.ident;

	let expanded = quote! {
		#input_struct

		use std::sync::{OnceLock, Mutex, Arc};

		#[cfg(feature = "cdylib")]
		#[unsafe(no_mangle)]
		pub extern "C" fn get_adapter_info() -> *mut dyn ::puniyu_adapter::AdapterBuilder {
			Box::into_raw(Box::new(#struct_name {}))
		}

		#[cfg(feature = "cdylib")]
		#[unsafe(no_mangle)]
		pub extern "C" fn setup_app_name(name: String) {
			 puniyu_adapter::APP_NAME.get_or_init(|| name);
		}

		#[cfg(feature = "cdylib")]
		#[unsafe(no_mangle)]
		pub extern "C" fn setup_event_bus(bus: Arc<Mutex<::puniyu_adapter::EventBus>>) {
			puniyu_adapter::setup_event_bus(bus);
		}

		#[macro_export]
		macro_rules! info {
				($($arg:tt)*) => {
					{
						use ::puniyu_adapter::logger::OwoColorize;
						let prefix = "adapter".fg_rgb::<176,196,222>();
						let func_name = #adapter_name.fg_rgb::<255,192,203>();
						::puniyu_adapter::logger::info!("[{}:{}] {}", prefix,func_name, format!($($arg)*))
					}
				};
		}

			#[macro_export]
			macro_rules! warn {
				($($arg:tt)*) => {
					{
						use ::puniyu_adapter::logger::OwoColorize;
						let prefix = "adapter".fg_rgb::<176,196,222>();
						let func_name = #adapter_name.fg_rgb::<255,192,203>();
						::puniyu_adapter::logger::warn!("[{}:{}] {}", prefix,func_name, format!($($arg)*))
					}
				};
			}

			#[macro_export]
			macro_rules! error {
				($($arg:tt)*) => {
				{
						use ::puniyu_adapter::logger::OwoColorize;
						let prefix = "adapter".fg_rgb::<176,196,222>();
						let func_name = #adapter_name.fg_rgb::<255,192,203>();
						::puniyu_adapter::logger::error!("[{}:{}] {}", prefix,func_name, format_args!($($arg)*))
					}
				};
			}

			#[macro_export]
			macro_rules! debug {
				($($arg:tt)*) => {
					{
						use ::puniyu_adapter::logger::OwoColorize;
						let prefix = "adapter".fg_rgb::<176,196,222>();
						let func_name = #adapter_name.fg_rgb::<255,192,203>();
						::puniyu_adapter::logger::debug!("[{}:{}] {}", prefix,func_name, format_args!($($arg)*))
					}
				};
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
///
/// # 示例
/// ## 最小化示例
/// ```rust, ignore
/// use puniyu_plugin_derive::plugin;
///
/// #[plugin]
/// pub async fn hello() {} // 默认会实现一个 log::info!("{} v{} 初始化完成",plugin_name, plugin_version);
/// ```
/// ## 完整示例
/// ```rust, ignore
/// use puniyu_plugin_derive::plugin;
///
/// #[plugin(name = "puniyu_plugin_hello", version = "0.1.0", author = "wuliya")]
/// pub async fn hello() {
///     println!("hello world");
/// }
/// ```
#[cfg(feature = "plugin")]
#[proc_macro_attribute]
pub fn plugin(_: TokenStream, item: TokenStream) -> TokenStream {
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

	let is_async = fn_sig.asyncness.is_some();
	if !is_async {
		return syn::Error::new_spanned(fn_sig, "诶嘿~杂鱼函数连async都不会用吗？")
			.to_compile_error()
			.into();
	}

	let plugin_name = match env::var("PLUGIN_NAME") {
		Ok(name) => name,
		Err(_) => {
			return syn::Error::new_spanned(fn_sig, "呜哇~PLUGIN_NAME都没有设置！杂鱼程序员！")
				.to_compile_error()
				.into();
		}
	};
	let plugin_version = match env::var("PLUGIN_VERSION") {
		Ok(version) => version,
		Err(_) => {
			return syn::Error::new_spanned(fn_sig, "呜哇~PLUGIN_VERSION都没有设置！杂鱼程序员！")
				.to_compile_error()
				.into();
		}
	};
	let plugin_author = match env::var("PLUGIN_AUTHOR") {
		Ok(author) => quote! { #author },
		Err(_) => {
			return syn::Error::new_spanned(fn_sig, "呜哇~PLUGIN_AUTHOR都没有设置！杂鱼程序员！")
				.to_compile_error()
				.into();
		}
	};

	// 检查插件名称是否符合命名规则
	if !plugin_name.starts_with("puniyu_plugin_") {
		return syn::Error::new_spanned(
			fn_name,
			format!(
				"呜哇~杂鱼插件名！必须用'puniyu_plugin_'开头啦！你这个'{}'是什么啦！",
				plugin_name
			),
		)
		.to_compile_error()
		.into();
	}

	let struct_name = Ident::new("Plugin", fn_name.span());

	// 默认初始化函数
	let init_call = if fn_block.stmts.is_empty() {
		quote! {
			async {
				puniyu_plugin::logger::info!(
					"{} v{} 初始化完成",
					#plugin_name,
					#plugin_version,
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

	// 生成最终的代码
	let expanded = quote! {
		pub struct #struct_name;

		#fn_vis #fn_sig #fn_block

		use puniyu_plugin::{APP_NAME, async_trait};

		#[async_trait]
		impl ::puniyu_plugin::PluginBuilder for #struct_name {
			fn name(&self) -> &'static str {
				#plugin_name
			}

			fn version(&self) -> &'static str {
				#plugin_version
			}

			fn author(&self) -> &'static str {
				#plugin_author
			}

			fn abi_version(&self) -> &'static str {
				::puniyu_plugin::ABI_VERSION
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

			async fn init(&self) -> Result<(), Box<dyn std::error::Error>>{
			   #init_call.await
			}
		}
			#[macro_export]
			macro_rules! info {
				($($arg:tt)*) => {
					{
						use ::puniyu_plugin::logger::OwoColorize;
						let prefix = "plugin".fg_rgb::<176,196,222>();
						let func_name = #plugin_name.fg_rgb::<255,192,203>();
						::puniyu_plugin::logger::info!("[{}:{}] {}", prefix,func_name, format!($($arg)*))
					}
				};
			}

			#[macro_export]
			macro_rules! warn {
				($($arg:tt)*) => {
					{
						use ::puniyu_plugin::logger::OwoColorize;
						let prefix = "plugin".fg_rgb::<176,196,222>();
						let func_name = #plugin_name.fg_rgb::<255,192,203>();
						::puniyu_plugin::logger::warn!("[{}:{}] {}", prefix,func_name, format!($($arg)*))
					}
				};
			}

			#[macro_export]
			macro_rules! error {
				($($arg:tt)*) => {
				{
						use ::puniyu_plugin::logger::OwoColorize;
						let prefix = "plugin".fg_rgb::<176,196,222>();
						let func_name = #plugin_name.fg_rgb::<255,192,203>();
						::puniyu_plugin::logger::error!("[{}:{}] {}", prefix,func_name, format_args!($($arg)*))
					}
				};
			}

			#[macro_export]
			macro_rules! debug {
				($($arg:tt)*) => {
					{
						use ::puniyu_plugin::logger::OwoColorize;
						let prefix = "plugin".fg_rgb::<176,196,222>();
						let func_name = #plugin_name.fg_rgb::<255,192,203>();
						::puniyu_plugin::logger::debug!("[{}:{}] {}", prefix,func_name, format_args!($($arg)*))
					}
				};
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
	use crate::common::get_plugin_name;
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
			"呜哇~命令函数必须有两个参数：&Bot, &EventContext！笨蛋！",
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

	let crate_name = match get_plugin_name(fn_sig) {
		Ok(name) => name,
		Err(err) => return err.to_compile_error().into(),
	};

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

			fn rank(&self) -> usize {
				#command_rank.to_string().parse().unwrap_or(100)
			}

			async fn run(&self, bot: &Bot, ev: &EventContext) -> ::puniyu_plugin::HandlerResult {
				#fn_name(bot, ev).await
			}
		}

		::puniyu_core::inventory::submit! {
			crate::CommandRegistry {
				plugin_name: #crate_name,
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
	use crate::common::get_plugin_name;
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

	let crate_name = match get_plugin_name(fn_sig) {
		Ok(name) => name,
		Err(err) => return err.to_compile_error().into(),
	};

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
			plugin_name: #crate_name,
			builder: || -> Box<dyn ::puniyu_plugin::TaskBuilder> { Box::new(#struct_name {}) },
		}
	}

	};

	TokenStream::from(expanded)
}
