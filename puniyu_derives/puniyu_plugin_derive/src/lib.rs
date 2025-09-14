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
#[derive(Default)]
struct PluginArgs {
    name: Option<syn::LitStr>,
    version: Option<syn::LitStr>,
    author: Option<syn::LitStr>,
}

impl Parse for PluginArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(PluginArgs::default());
        }

        let fields = Punctuated::<syn::MetaNameValue, Token![,]>::parse_terminated(input)?;
        parse_fields(
            &fields,
            &[
                ("name", Self::set_name as FieldSetter<Self>),
                ("version", Self::set_version as FieldSetter<Self>),
                ("author", Self::set_author as FieldSetter<Self>),
            ],
        )
    }
}

impl PluginArgs {
    fn set_name(args: &mut Self, value: syn::LitStr) -> syn::Result<()> {
        args.name = Some(value);
        Ok(())
    }

    fn set_version(args: &mut Self, value: syn::LitStr) -> syn::Result<()> {
        args.version = Some(value);
        Ok(())
    }

    fn set_author(args: &mut Self, value: syn::LitStr) -> syn::Result<()> {
        args.author = Some(value);
        Ok(())
    }
}

/// 注册插件
/// 此宏包含以下检测：
/// 1. 函数是否为异步函数
/// 2. 插件名称，插件版本，插件作者
/// 3. 插件名称是否规范
///
/// # 参数
///
/// * `name` - 插件名称
/// * `version` - 插件版本
/// * `author` - 插件作者
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
/// ```rust
/// use puniyu_plugin_derive::plugin;
///
/// #[plugin(name = "puniyu_plugin_hello", version = "0.1.0", author = "wuliya")]
/// pub async fn hello() {
///     println!("hello world");
/// }
/// ```
#[proc_macro_attribute]
pub fn plugin(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as PluginArgs);

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

    // 获取插件名称
    let plugin_name = match &args.name {
        Some(name) => name.value(),
        None => match get_plugin_name(fn_sig) {
            Ok(name) => name,
            Err(err) => return err,
        },
    };

    // 获取插件版本
    let plugin_version = match &args.version {
        Some(version) => version.value(),
        None => match env::var("PLUGIN_VERSION") {
            Ok(version) => version,
            Err(_) => {
                return syn::Error::new_spanned(fn_sig, "诶？PLUGIN_VERSION都没设置！笨蛋！")
                    .to_compile_error()
                    .into();
            }
        },
    };

    // 获取插件作者
    let plugin_author = match &args.author {
        Some(author) => author.value(),
        None => match env::var("PLUGIN_AUTHOR") {
            Ok(author) => author,
            Err(_) => {
                return syn::Error::new_spanned(fn_sig, "哼！连PLUGIN_AUTHOR都不设置！杂鱼~")
                    .to_compile_error()
                    .into();
            }
        },
    };

    // 生成插件元信息
    let name = match &args.name {
        Some(name) => quote! { #name },
        None => quote! { #plugin_name },
    };

    let version = match &args.version {
        Some(version) => quote! { #version },
        None => quote! { #plugin_version },
    };

    let author = match &args.author {
        Some(author) => quote! { #author },
        None => quote! { #plugin_author },
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
            Box::pin(async {
                log::info!(
                    "{} v{} 初始化完成",
                    #name,
                    #version
                );
            })
        }
    } else {
        quote! {
            Box::pin(async {
                #fn_name().await
            })
        }
    };

    // 生成最终的代码
    let expanded = quote! {
        pub struct #struct_name;

        #fn_vis #fn_sig #fn_block

        impl ::puniyu_registry::plugin::builder::PluginBuilder for #struct_name {
            fn name(&self) -> &'static str {
                #name
            }

            fn version(&self) -> &'static str {
                #version
            }

            fn author(&self) -> &'static str {
                #author
            }

            fn abi_version(&self) -> &'static str {
                ::puniyu_registry::VERSION
            }

            fn tasks(&self) -> Vec<Box<dyn ::puniyu_registry::plugin::task::builder::TaskBuilder>> {
                let plugin_name = self.name();
                ::puniyu_registry::inventory::iter::<TaskRegistry>
                    .into_iter()
                    .filter(|task| task.plugin_name == plugin_name)
                    .map(|task| (task.builder)())
                    .collect()
            }

            fn commands(&self) -> Vec<Box<dyn ::puniyu_registry::plugin::command::builder::CommandBuilder>> {
                let plugin_name = self.name();
                ::puniyu_registry::inventory::iter::<CommandRegistry>
                    .into_iter()
                    .filter(|command| command.plugin_name == plugin_name)
                    .map(|command| (command.builder)())
                    .collect()
            }

            fn init(&self) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ()> + Send + 'static>> {
               #init_call
            }
        }
        /// 插件注册表
        pub struct PluginRegistry {
            /// 插件构造器
            builder: fn() -> Box<dyn ::puniyu_registry::plugin::builder::PluginBuilder>,
        }
        ::puniyu_registry::inventory::collect!(PluginRegistry);

        /// 定时计划注册表
        pub struct TaskRegistry {
            /// 插件名称
            plugin_name: &'static str,
            /// 任务构造器
            builder: fn() -> Box<dyn ::puniyu_registry::plugin::task::builder::TaskBuilder>,
        }
        ::puniyu_registry::inventory::collect!(TaskRegistry);

        pub struct CommandRegistry {
            plugin_name: &'static str,
            /// 命令构造器
            builder: fn() -> Box<dyn ::puniyu_registry::plugin::command::builder::CommandBuilder>,
        }
        ::puniyu_registry::inventory::collect!(CommandRegistry);

        ::puniyu_registry::inventory::submit! {
            PluginRegistry {
                builder: || -> Box<dyn ::puniyu_registry::plugin::builder::PluginBuilder> { Box::new(#struct_name {}) },
            }
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn plugin_info() -> *mut dyn puniyu_registry::plugin::builder::PluginBuilder {
            Box::into_raw(Box::new(#struct_name {}))
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn setup_logger(logger: &::puniyu_registry::logger::SharedLogger) {
            ::puniyu_registry::logger::setup_shared_logger(logger);
        }

    };

    TokenStream::from(expanded)
}
struct TaskArgs {
    cron: syn::LitStr,
}

impl Default for TaskArgs {
    fn default() -> Self {
        Self {
            cron: syn::LitStr::new("", proc_macro2::Span::call_site()),
        }
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
            return Err(syn::Error::new(
                input.span(),
                "呜~至少给人家一个cron表达式嘛！杂鱼~",
            ));
        }

        let fields = Punctuated::<syn::MetaNameValue, Token![,]>::parse_terminated(input)?;
        let args = parse_fields(&fields, &[("cron", Self::set_cron as FieldSetter<Self>)])?;

        if args.cron.value().is_empty() {
            return Err(syn::Error::new(
                input.span(),
                "诶嘿~cron表达式都不给！杂鱼程序员！",
            ));
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

    impl ::puniyu_registry::plugin::task::builder::TaskBuilder for #struct_name {
        fn name(&self) -> &'static str {
            stringify!(#fn_name)
        }

        fn cron(&self) -> &'static str {
            #cron_expr
        }

        fn run(&self) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ()> + Send + 'static>> {
            Box::pin(#fn_name())
        }
    }

    ::puniyu_registry::inventory::submit! {
        crate::TaskRegistry  {
            plugin_name: #crate_name,
            builder: || -> Box<dyn ::puniyu_registry::plugin::task::builder::TaskBuilder> { Box::new(#struct_name {}) },
        }
    }

    };

    TokenStream::from(expanded)
}
