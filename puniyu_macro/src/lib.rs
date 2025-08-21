use proc_macro::TokenStream;
use quote::quote;
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
        return syn::Error::new_spanned(
            fn_sig,
            "plugin attribute can only be applied to async function",
        )
        .to_compile_error()
        .into();
    }

    let crate_name = env!("CARGO_PKG_NAME");

    let struct_name_str = fn_name.to_string();

    let pascal_case_name: String = struct_name_str
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect();

    let struct_name_str = pascal_case_name + "Plugin";
    let struct_name = Ident::new(&struct_name_str, fn_name.span());

    let name = match &args.name {
        Some(name) => quote! { #name },
        None => quote! { #crate_name },
    };

    let version = match &args.version {
        Some(version) => quote! { #version },
        None => quote! { env!("CARGO_PKG_VERSION") },
    };

    let author = match &args.author {
        Some(author) => quote! { #author },
        None => quote! { env!("CARGO_PKG_AUTHORS") },
    };

    let rustc_version = quote! { env!("CARGO_PKG_RUST_VERSION") };

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
