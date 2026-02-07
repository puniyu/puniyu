use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

pub fn api(item: TokenStream) -> TokenStream {
	let item = parse_macro_input!(item as ItemFn);
	let fn_sig = &item.sig;
	let fn_name = &fn_sig.ident;

	match &fn_sig.output {
		syn::ReturnType::Type(_, return_type) => {
			let is_adapter_api = if let syn::Type::Path(type_path) = &**return_type {
				if let Some(segment) = type_path.path.segments.last() {
					segment.ident == "AdapterApi"
				} else {
					false
				}
			} else {
				false
			};
			if !is_adapter_api {
				return syn::Error::new_spanned(
					return_type,
					"Function must return `AdapterApi` type",
				)
					.to_compile_error()
					.into();
			}
		}
		syn::ReturnType::Default => {
			return syn::Error::new_spanned(
				fn_sig,
				"Function must have an explicit return type",
			)
				.to_compile_error()
				.into();
		}
	}

	let adapter_name = quote! { env!("CARGO_PKG_NAME") };
	let expanded = quote! {
        #item
		::puniyu_adapter::__private::inventory::submit! {
			crate::ApiRegistry {
				adapter_name: #adapter_name,
				builder: #fn_name(),
			}
		}
    };

	TokenStream::from(expanded)
}
