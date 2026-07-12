//! # puniyu_macros
//!
//! `puniyu_macros` 提供 puniyu 生态中的过程宏，用于声明插件、适配器、命令、任务和配置，减少 trait 实现与 `inventory` 注册样板代码。
//!
//! ## 插件侧宏
//!
//! - [`plugin`]：声明 `Plugin` 结构体
//! - [`command`]：声明命令处理函数
//! - [`arg`]：为命令补充参数描述
//! - [`task`]：声明定时任务函数
//! - [`on_load`]：声明插件加载生命周期钩子
//! - [`on_unload`]：声明插件卸载生命周期钩子
//! - [`server`]：声明服务函数
//! - [`config`]：声明配置函数
//! - `#[derive(PluginConfig)]`：为插件配置结构体派生 `puniyu_config::Config`
//!
//! ## 适配器侧宏
//!
//! - [`adapter`]：声明 `Adapter` 结构体
//! - [`api`]：声明适配器 API 实例
//! - [`on_load`]：声明适配器加载生命周期钩子
//! - [`on_unload`]：声明适配器卸载生命周期钩子
//! - [`server`]：声明服务函数
//! - [`config`]：声明配置函数
//! - `#[derive(AdapterConfig)]`：为适配器配置结构体派生 `puniyu_config::Config`
//!
//! ## 编译期校验
//!
//! 大多数函数类宏会在编译期校验以下内容：
//!
//! - 被标注项是否为 `async fn`
//! - 函数参数和返回类型是否满足约束
//! - 属性参数是否使用合法的 key-value 形式
//! - `cron`、`permission` 等枚举值或格式是否有效

mod adapter;
mod common;
mod config;
mod plugin;
mod types;
pub(crate) use types::*;

fn parse_attr<T: syn::parse::Parse>(
	tokens: proc_macro::TokenStream,
) -> Result<T, proc_macro::TokenStream> {
	syn::parse(tokens).map_err(|err| err.to_compile_error().into())
}

fn arg_marker(args: proc_macro::TokenStream) -> proc_macro2::TokenStream {
	let args: proc_macro2::TokenStream = args.into();
	quote::quote!(#[__puniyu_arg(#args)])
}

#[proc_macro_attribute]
pub fn adapter(
	args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let cfg = match AdapterArgs::parse_tokens(args.into()) {
		Ok(cfg) => cfg,
		Err(err) => return err.to_compile_error().into(),
	};
	let item_ts: proc_macro2::TokenStream = item.clone().into();
	if let Ok(item) = syn::parse2::<syn::ItemStruct>(item_ts) {
		return adapter::adapter_struct(item, cfg).into();
	}
	syn::Error::new(proc_macro2::Span::call_site(), "#[adapter] must be used on a struct")
		.to_compile_error()
		.into()
}

#[proc_macro_attribute]
pub fn plugin(
	args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let cfg = match PluginArg::parse_tokens(args.into()) {
		Ok(cfg) => cfg,
		Err(err) => return err.to_compile_error().into(),
	};
	let item_ts: proc_macro2::TokenStream = item.clone().into();
	if let Ok(item) = syn::parse2::<syn::ItemStruct>(item_ts) {
		return plugin::plugin_struct(item, cfg).into();
	}
	syn::Error::new(proc_macro2::Span::call_site(), "#[plugin] must be used on a struct")
		.to_compile_error()
		.into()
}

#[proc_macro_attribute]
pub fn command(
	args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let item = match parse_attr::<syn::ItemFn>(item) {
		Ok(item) => item,
		Err(err) => return err,
	};
	let cfg = match CommandArgs::parse_tokens(args.into()) {
		Ok(cfg) => cfg,
		Err(err) => return err.to_compile_error().into(),
	};
	plugin::command(item, cfg).into()
}

#[proc_macro_attribute]
pub fn arg(
	args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	if let Err(err) = ArgType::parse_tokens(args.clone().into()) {
		return err.to_compile_error().into();
	}
	let marker = arg_marker(args);
	let item: proc_macro2::TokenStream = item.into();
	quote::quote!(#marker #item).into()
}

#[proc_macro_attribute]
pub fn task(
	args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let item = match parse_attr::<syn::ItemFn>(item) {
		Ok(item) => item,
		Err(err) => return err,
	};
	let cfg = match TaskArgs::parse_tokens(args.into()) {
		Ok(cfg) => cfg,
		Err(err) => return err.to_compile_error().into(),
	};
	plugin::task(item, cfg).into()
}

#[proc_macro_attribute]
pub fn on_load(
	_args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let item = match parse_attr::<syn::ItemFn>(item) {
		Ok(item) => item,
		Err(err) => return err,
	};
	if let Err(err) = common::ensure_lifecycle_fn(&item, "result::Result") {
		return err.to_compile_error().into();
	}
	let fn_name = &item.sig.ident;
	quote::quote! {
		#item

		crate::__puniyu_submit!(on_load, #fn_name);
	}
	.into()
}

#[proc_macro_attribute]
pub fn on_unload(
	_args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let item = match parse_attr::<syn::ItemFn>(item) {
		Ok(item) => item,
		Err(err) => return err,
	};
	if let Err(err) = common::ensure_lifecycle_fn(&item, "result::Result") {
		return err.to_compile_error().into();
	}
	let fn_name = &item.sig.ident;
	quote::quote! {
		#item

		crate::__puniyu_submit!(on_unload, #fn_name);
	}
	.into()
}

#[proc_macro_attribute]
pub fn server(
	_args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let item = match parse_attr::<syn::ItemFn>(item) {
		Ok(item) => item,
		Err(err) => return err,
	};
	if let Err(err) = common::ensure_no_params(&item.sig) {
		return err.to_compile_error().into();
	}
	let fn_name = &item.sig.ident;
	quote::quote! {
		#item

		crate::__puniyu_submit!(server, #fn_name);
	}
	.into()
}

#[proc_macro_attribute]
pub fn plugin_config(
	args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	config::config_impl(args, item, config::ConfigContext::Plugin)
}

#[proc_macro_attribute]
pub fn adapter_config(
	args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	config::config_impl(args, item, config::ConfigContext::Adapter)
}

#[proc_macro_attribute]
pub fn api(
	_args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let item = match parse_attr::<syn::ItemFn>(item) {
		Ok(item) => item,
		Err(err) => return err,
	};
	adapter::api_fn(item).into()
}
