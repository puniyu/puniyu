mod utils;

use crate::utils::{get_plugin_name, parse_fields};
use convert_case::{Case, Casing};
use croner::Cron;
use proc_macro::TokenStream;
use quote::quote;
use std::{env, str::FromStr};
use syn::{
	self, Ident, ItemFn, Token, parse::Parse, parse::ParseStream, parse_macro_input,
	punctuated::Punctuated,
};
type FieldSetter<T> = fn(&mut T, syn::LitStr) -> syn::Result<()>;

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

	// 检查函数是否是异步的
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

	let struct_name = Ident::new("PluginInfo", fn_name.span());

	// 默认初始化函数
	let init_call = if fn_block.stmts.is_empty() {
		quote! {
			async {
				log::info!(
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

		use puniyu_core::{APP_NAME, async_trait};

		#[async_trait]
		impl ::puniyu_core::PluginBuilder for #struct_name {
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
				::puniyu_core::ABI_VERSION
			}

			fn tasks(&self) -> Vec<Box<dyn ::puniyu_core::TaskBuilder>> {
				let plugin_name = self.name();
				::puniyu_core::inventory::iter::<TaskRegistry>
					.into_iter()
					.filter(|task| task.plugin_name == plugin_name)
					.map(|task| (task.builder)())
					.collect()
			}

			fn commands(&self) -> Vec<Box<dyn ::puniyu_core::CommandBuilder>> {
				let plugin_name = self.name();
				::puniyu_core::inventory::iter::<CommandRegistry>
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
						use ::puniyu_core::logger::OwoColorize;
						let prefix = "plugin".fg_rgb::<176,196,222>();
						let func_name = #plugin_name.fg_rgb::<255,192,203>();
						::puniyu_core::logger::info!("[{}:{}] {}", prefix,func_name, format!($($arg)*))
					}
				};
			}

			#[macro_export]
			macro_rules! warn {
				($($arg:tt)*) => {
					{
						use ::puniyu_core::logger::OwoColorize;
						let prefix = "plugin".fg_rgb::<176,196,222>();
						let func_name = #plugin_name.fg_rgb::<255,192,203>();
						::puniyu_core::logger::warn!("[{}:{}] {}", prefix,func_name, format!($($arg)*))
					}
				};
			}

			#[macro_export]
			macro_rules! error {
				($($arg:tt)*) => {
				{
						use ::puniyu_core::logger::OwoColorize;
						let prefix = "plugin".fg_rgb::<176,196,222>();
						let func_name = #plugin_name.fg_rgb::<255,192,203>();
						::puniyu_core::logger::error!("[{}:{}] {}", prefix,func_name, format_args!($($arg)*))
					}
				};
			}

			#[macro_export]
			macro_rules! debug {
				($($arg:tt)*) => {
					{
						use ::puniyu_core::logger::OwoColorize;
						let prefix = "plugin".fg_rgb::<176,196,222>();
						let func_name = #plugin_name.fg_rgb::<255,192,203>();
						::puniyu_core::logger::debug!("[{}:{}] {}", prefix,func_name, format_args!($($arg)*))
					}
				};
			}

		/// 插件注册表
		pub struct PluginRegistry {
			/// 插件构造器
			builder: fn() -> Box<dyn ::puniyu_core::PluginBuilder>,
		}
		::puniyu_core::inventory::collect!(PluginRegistry);

		/// 定时计划注册表
		pub struct TaskRegistry {
			/// 插件名称
			plugin_name: &'static str,
			/// 任务构造器
			builder: fn() -> Box<dyn ::puniyu_core::TaskBuilder>,
		}
		::puniyu_core::inventory::collect!(TaskRegistry);

		pub struct CommandRegistry {
			plugin_name: &'static str,
			/// 命令构造器
			builder: fn() -> Box<dyn ::puniyu_core::CommandBuilder>,
		}
		::puniyu_core::inventory::collect!(CommandRegistry);

		::puniyu_core::inventory::submit! {
			PluginRegistry {
				builder: || -> Box<dyn ::puniyu_core::PluginBuilder> { Box::new(#struct_name {}) },
			}
		}

		#[unsafe(no_mangle)]
		pub unsafe extern "C" fn plugin_info() -> *mut dyn ::puniyu_core::PluginBuilder {
			Box::into_raw(Box::new(#struct_name {}))
		}

		#[unsafe(no_mangle)]
		pub unsafe extern "C" fn setup_logger(logger: &::puniyu_core::logger::SharedLogger) {
			::puniyu_core::logger::setup_shared_logger(logger);
		}

		#[unsafe(no_mangle)]
		pub extern "C" fn setup_app_name(name: String) {
			 APP_NAME.get_or_init(|| name);
		}

	};

	TokenStream::from(expanded)
}
struct TaskArgs {
	cron: syn::LitStr,
}

impl Default for TaskArgs {
	fn default() -> Self {
		Self { cron: syn::LitStr::new("", proc_macro2::Span::call_site()) }
	}
}

impl TaskArgs {
	fn set_cron(args: &mut Self, value: syn::LitStr) -> syn::Result<()> {
		args.cron = value;
		Ok(())
	}
}
impl Parse for TaskArgs {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		if input.is_empty() {
			return Err(syn::Error::new(input.span(), "呜~至少给人家一个cron表达式嘛！杂鱼~"));
		}

		let fields = Punctuated::<syn::MetaNameValue, Token![,]>::parse_terminated(input)?;
		let args = parse_fields(&fields, &[("cron", Self::set_cron as FieldSetter<Self>)])?;

		if args.cron.value().is_empty() {
			return Err(syn::Error::new(input.span(), "诶嘿~cron表达式都不给！杂鱼程序员！"));
		}

		Ok(args)
	}
}

#[proc_macro_attribute]
pub fn task(args: TokenStream, item: TokenStream) -> TokenStream {
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
		Err(err) => return err,
	};

	let cron_expr = &args.cron;

	let struct_name_str = {
		let fn_name_str = fn_name.to_string();
		let pascal_case_name = fn_name_str.to_case(Case::Pascal);
		format!("{}Task", pascal_case_name)
	};
	let struct_name = Ident::new(&struct_name_str, fn_name.span());

	let expanded = quote! {
	pub struct #struct_name;

	#fn_vis #fn_sig #fn_block

		#[::puniyu_core::async_trait]
		impl ::puniyu_core::TaskBuilder for #struct_name {
			fn name(&self) -> &'static str {
			stringify!(#fn_name)
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
			builder: || -> Box<dyn ::puniyu_core::TaskBuilder> { Box::new(#struct_name {}) },
		}
	}

	};

	TokenStream::from(expanded)
}
