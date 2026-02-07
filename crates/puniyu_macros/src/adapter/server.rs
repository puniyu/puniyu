use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

pub fn server(item: TokenStream) -> TokenStream {
	let item = parse_macro_input!(item as ItemFn);
	let fn_sig = &item.sig;
	let fn_name = &fn_sig.ident;
	let fn_inputs = &fn_sig.inputs;
	if fn_inputs.len() != 1 {
		return syn::Error::new_spanned(
			fn_sig,
			format!(
				"function `{}` must have exactly 1 parameters, found {}",
				fn_name,
				fn_inputs.len()
			),
		)
		.to_compile_error()
		.into();
	}

	if let Some(first_param) = fn_sig.inputs.first() {
		if let syn::FnArg::Typed(pat_type) = first_param {
			let ty = &*pat_type.ty;
			let is_valid_type = matches!(
				ty,
				syn::Type::Reference(type_ref)
					if type_ref.mutability.is_some()
						&& matches!(
							&*type_ref.elem,
							syn::Type::Path(type_path)
								if type_path.path.segments.last().map(|seg| seg.ident == "ServiceConfig").unwrap_or(false)
						)
			);

			if !is_valid_type {
				return syn::Error::new_spanned(
					pat_type,
					format!(
						"function `{}` parameter must be of type `&mut ServiceConfig`, found `{}`",
						fn_name,
						quote::quote! { #ty }
					),
				)
				.to_compile_error()
				.into();
			}
		} else {
			return syn::Error::new_spanned(
				first_param,
				format!("function `{}` parameter must be a typed parameter", fn_name),
			)
			.to_compile_error()
			.into();
		}
	} else {
		return syn::Error::new_spanned(fn_sig, "function signature has no parameters")
			.to_compile_error()
			.into();
	}

	let adapter_name = quote! { env!("CARGO_PKG_NAME") };
	let expanded = quote! {
		#item

		::puniyu_adapter::__private::inventory::submit! {
			crate::ServerRegistry {
				adapter_name: #adapter_name,
				builder: || -> ::puniyu_adapter::__private::ServerFunction { ::std::sync::Arc::new(#fn_name) },
			}
		}
	};
	expanded.into()
}
