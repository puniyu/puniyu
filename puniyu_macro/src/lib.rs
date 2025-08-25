use proc_macro::TokenStream;
use quote::quote;
use regex::Regex;
use std::env;
use std::sync::LazyLock;
use syn::{
    self, Ident, ItemFn, Token, parse::Parse, parse::ParseStream, parse_macro_input,
    punctuated::Punctuated,
};

#[derive(Default)]
struct PluginArgs {
    name: Option<syn::LitStr>,
    version: Option<syn::LitStr>,
    author: Option<syn::LitStr>,
}

impl Parse for PluginArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut args = PluginArgs::default();

        if input.is_empty() {
            return Ok(args);
        }

        let fields = Punctuated::<syn::MetaNameValue, Token![,]>::parse_terminated(input)?;
        for field in fields {
            let key = field
                .path
                .get_ident()
                .ok_or_else(|| syn::Error::new_spanned(&field.path, "Expected identifier"))?;
            let value = match &field.value {
                syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit_str),
                    ..
                }) => lit_str.clone(),
                _ => {
                    return Err(syn::Error::new_spanned(
                        &field.value,
                        "Expected string literal",
                    ));
                }
            };

            match key.to_string().as_str() {
                "name" => args.name = Some(value),
                "version" => args.version = Some(value),
                "author" => args.author = Some(value),
                _ => return Err(syn::Error::new_spanned(key, "Unknown field")),
            }
        }

        Ok(args)
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
        return syn::Error::new_spanned(fn_sig, "插件入口函数必须为async函数")
            .to_compile_error()
            .into();
    }

    let crate_name = match env::var("PLUGIN_NAME") {
        Ok(name) => name,
        Err(_) => {
            return syn::Error::new_spanned(fn_sig, "PLUGIN_NAME未定义")
                .to_compile_error()
                .into();
        }
    };

    let crate_version = match env::var("PLUGIN_VERSION") {
        Ok(version) => version,
        Err(_) => {
            return syn::Error::new_spanned(fn_sig, "PLUGIN_VERSION未定义")
                .to_compile_error()
                .into();
        }
    };

    let crate_author = match env::var("PLUGIN_AUTHOR") {
        Ok(author) => author,
        Err(_) => {
            return syn::Error::new_spanned(fn_sig, "PLUGIN_AUTHOR未定义")
                .to_compile_error()
                .into();
        }
    };

    let rustc_version = match env::var("PLUGIN_RUSTC_VERSION") {
        Ok(author) => author,
        Err(_) => {
            return syn::Error::new_spanned(fn_sig, "PLUGIN_RUSTC_VERSION未定义")
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
            format!("插件名称必须以'puniyu_plugin_'开头，当前为'{}'", crate_name),
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
               Box::pin(#fn_name())
            }
        }
    };

    TokenStream::from(expanded)
}
