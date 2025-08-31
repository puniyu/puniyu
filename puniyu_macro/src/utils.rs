use proc_macro::TokenStream;
use std::env;
use syn::{Signature, Token, punctuated::Punctuated};

use once_cell::sync::OnceCell;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

pub(crate) fn parse_fields<T, F>(
    fields: &Punctuated<syn::MetaNameValue, Token![,]>,
    field_list: &[(&str, F)],
) -> syn::Result<T>
where
    T: Default,
    F: Fn(&mut T, syn::LitStr) -> syn::Result<()>,
{
    let mut args = T::default();

    for field in fields {
        let key = field
            .path
            .get_ident()
            .ok_or_else(|| syn::Error::new_spanned(&field.path, "呜~这里需要一个名字啦，杂鱼！"))?;
        let value = match &field.value {
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(lit_str),
                ..
            }) => lit_str.clone(),
            _ => {
                return Err(syn::Error::new_spanned(
                    &field.value,
                    "诶？这里要字符串啦，笨蛋",
                ));
            }
        };

        let key_str = key.to_string();
        let mut found = false;
        for (field_name, setter) in field_list {
            if &key_str == field_name {
                setter(&mut args, value)?;
                found = true;
                break;
            }
        }

        if !found {
            let supported_fields: Vec<&str> = field_list.iter().map(|(name, _)| *name).collect();
            return Err(syn::Error::new_spanned(
                key,
                format!(
                    "诶嘿~这是什么奇怪的字段啦！笨蛋！只支持这些啦: {}",
                    supported_fields.join(", ")
                ),
            ));
        }
    }

    Ok(args)
}

pub(crate) fn get_plugin_name(fn_sig: &Signature) -> Result<String, TokenStream> {
    match env::var("PLUGIN_NAME") {
        Ok(name) => Ok(name),
        Err(_) => Err(
            syn::Error::new_spanned(fn_sig, "呜哇~PLUGIN_NAME都没有设置！杂鱼程序员！")
                .to_compile_error()
                .into(),
        ),
    }
}

static REGISTERED_PLUGINS: OnceCell<Mutex<HashMap<String, HashSet<String>>>> = OnceCell::new();

pub(crate) fn register_plugin(fn_sig: &Signature, plugin_name: &str) -> Result<(), TokenStream> {
    let crate_name = env::var("CARGO_PKG_NAME").unwrap_or_else(|_| "unknown_crate".to_string());

    let registry = REGISTERED_PLUGINS.get_or_init(|| Mutex::new(HashMap::new()));
    let mut registry = registry.lock().unwrap();

    let plugins = registry.entry(crate_name.clone()).or_default();

    if !plugins.insert(plugin_name.to_string()) {
        return Err(
            syn::Error::new_spanned(
                fn_sig,
                format!(
                    "哼哼~ 杂鱼程序员哟，想偷偷给插件 '{}' 在crate '{}' 这里注册两次？达咩哟达咩达咩！这种错误太杂鱼啦~",
                    plugin_name, crate_name
                ),
            )
                .to_compile_error()
                .into(),
        );
    }

    Ok(())
}

pub(crate) fn is_plugin_registered(plugin_name: &str) -> bool {
    let crate_name = env::var("CARGO_PKG_NAME").unwrap_or_else(|_| "unknown_crate".to_string());

    let registry = REGISTERED_PLUGINS.get_or_init(|| Mutex::new(HashMap::new()));
    let registry = registry.lock().unwrap();

    if let Some(plugins) = registry.get(&crate_name) {
        plugins.contains(plugin_name)
    } else {
        false
    }
}
