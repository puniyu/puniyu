use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

pub fn server(item: TokenStream) -> TokenStream {
	let item = parse_macro_input!(item as ItemFn);
	let fn_name = &item.sig.ident;
	let fn_sig = &item.sig;
	if fn_sig.inputs.len() != 1 {
		return syn::Error::new_spanned(
			fn_sig,
			"Function must accept exactly one parameter: &mut ServiceConfig",
		)
		.to_compile_error()
		.into();
	}
	let plugin_name = quote! { env!("CARGO_PKG_NAME") };
	let expanded = quote! {
		#item

		::puniyu_plugin::private::inventory::submit! {
			crate::ServerRegistry {
				plugin_name: #plugin_name,
				builder: || -> ::puniyu_plugin::private::ServerType { ::std::sync::Arc::new(#fn_name) },
			}
		}
	};
	expanded.into()
}
