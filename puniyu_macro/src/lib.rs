mod utils;

use crate::utils::parse_fields;
use croner::Cron;
use proc_macro::TokenStream;
use quote::quote;
use regex::Regex;
use std::{env, str::FromStr, sync::LazyLock};
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

static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(^|[-_])([a-z])").unwrap());

#[proc_macro_attribute]
pub fn plugin(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as PluginArgs);
    let input_fn = parse_macro_input!(item as ItemFn);
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

    let crate_name = match env::var("PLUGIN_NAME") {
        Ok(name) => name,
        Err(_) => {
            return syn::Error::new_spanned(fn_sig, "呜哇~PLUGIN_NAME都没有设置！杂鱼程序员！")
                .to_compile_error()
                .into();
        }
    };

    let crate_version = match env::var("PLUGIN_VERSION") {
        Ok(version) => version,
        Err(_) => {
            return syn::Error::new_spanned(fn_sig, "诶？PLUGIN_VERSION都没设置！笨蛋！")
                .to_compile_error()
                .into();
        }
    };

    let crate_author = match env::var("PLUGIN_AUTHOR") {
        Ok(author) => author,
        Err(_) => {
            return syn::Error::new_spanned(fn_sig, "哼！连PLUGIN_AUTHOR都不设置！杂鱼~")
                .to_compile_error()
                .into();
        }
    };

    let rustc_version = match env::var("PLUGIN_RUSTC_VERSION") {
        Ok(author) => author,
        Err(_) => {
            return syn::Error::new_spanned(fn_sig, "呜~PLUGIN_RUSTC_VERSION都忘了！真是的！")
                .to_compile_error()
                .into();
        }
    };
    let name = match &args.name {
        Some(name) => quote! { #name },
        None => quote! { #crate_name },
    };

    let version = match &args.version {
        Some(version) => quote! { #version },
        None => quote! { #crate_version },
    };
    let author = match &args.author {
        Some(author) => quote! { #author },
        None => quote! { #crate_author },
    };

    if !crate_name.starts_with("puniyu_plugin_") {
        return syn::Error::new_spanned(
            fn_name,
            format!(
                "呜哇~杂鱼插件名！必须用'puniyu_plugin_'开头啦！你这个'{}'是什么啦！",
                crate_name
            ),
        )
        .to_compile_error()
        .into();
    }

    let plugin_part = if let Some(stripped) = crate_name.strip_prefix("puniyu_plugin_") {
        stripped
    } else {
        &crate_name
    };

    let pascal_case_name = RE
        .replace_all(plugin_part, |caps: &regex::Captures| caps[2].to_uppercase())
        .to_string();

    let struct_name_str = pascal_case_name + "Plugin";
    let struct_name = Ident::new(&struct_name_str, fn_name.span());

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
        quote! { Box::pin(#fn_name()) }
    };

    let expanded = quote! {
        pub struct #struct_name;

        #fn_vis #fn_sig #fn_block

        impl ::puniyu_registry::plugin::PluginInfo for #struct_name {
            fn name(&self) -> &'static str {
                #name
            }

            fn version(&self) -> &'static str {
                #version
            }

            fn author(&self) -> &'static str {
                #author
            }

            fn rustc_version(&self) -> &'static str {
                #rustc_version
            }

            fn init(&self) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ()> + Send + 'static>> {
               #init_call
            }
        }
    };

    TokenStream::from(expanded)
}

struct TaskArgs {
    name: Option<syn::LitStr>,
    cron: syn::LitStr,
}

impl Default for TaskArgs {
    fn default() -> Self {
        Self {
            name: None,
            cron: syn::LitStr::new("", proc_macro2::Span::call_site()),
        }
    }
}

impl TaskArgs {
    fn set_name(args: &mut Self, value: syn::LitStr) -> syn::Result<()> {
        args.name = Some(value);
        Ok(())
    }

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
        let args = parse_fields(
            &fields,
            &[
                ("name", Self::set_name as FieldSetter<Self>),
                ("cron", Self::set_cron as FieldSetter<Self>),
            ],
        )?;

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

    let plugin_name = match env::var("PLUGIN_NAME") {
        Ok(name) => {
            if let Some(stripped) = name.strip_prefix("puniyu_plugin_") {
                stripped.to_string()
            } else {
                name
            }
        }
        Err(_) => "Unknown".to_string(),
    };
    let task_name = match &args.name {
        Some(name) => quote! { #name },
        None => {
            let full_name = format!("{}Plugin:{}", plugin_name, fn_name);
            quote! { #full_name }
        }
    };

    let cron_expr = &args.cron;

    let struct_name_str = {
        let fn_name_str = fn_name.to_string();
        let pascal_case_name = RE
            .replace_all(&fn_name_str, |caps: &regex::Captures| {
                caps[2].to_uppercase()
            })
            .to_string();
        format!("{}Task", pascal_case_name)
    };
    let struct_name = Ident::new(&struct_name_str, fn_name.span());

    let expanded = quote! {
        pub struct #struct_name;

        #fn_vis #fn_sig #fn_block

        impl ::puniyu_registry::task::TaskInfo for #struct_name {
            fn name(&self) -> &'static str {
                #task_name
            }

            fn cron(&self) -> &'static str {
                #cron_expr
            }

            fn run(&self) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ()> + Send + 'static>> {
                Box::pin(#fn_name())
            }
        }
    };

    TokenStream::from(expanded)
}
