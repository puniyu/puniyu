pub(crate) fn validate_return_type(
	fn_sig: &syn::Signature,
	expected_type: &str,
) -> proc_macro2::TokenStream {
	match &fn_sig.output {
		syn::ReturnType::Type(_, return_type) => {
			let type_matches = check_simple_type_match(return_type, expected_type);

			if !type_matches {
				return syn::Error::new_spanned(
					return_type,
					format!("Function must return `{}` type", expected_type),
				)
				.to_compile_error();
			}
		}
		syn::ReturnType::Default => {
			return syn::Error::new_spanned(fn_sig, "Function must have an explicit return type")
				.to_compile_error();
		}
	}

	quote::quote! {}
}

fn check_simple_type_match(return_type: &syn::Type, expected_type: &str) -> bool {
	if let syn::Type::Path(type_path) = return_type {
		if let Some(segment) = type_path.path.segments.last()
			&& segment.ident == expected_type
		{
			return true;
		}

		let full_path = type_path
			.path
			.segments
			.iter()
			.map(|seg| seg.ident.to_string())
			.collect::<Vec<_>>()
			.join("::");

		if full_path.ends_with(expected_type) {
			return true;
		}
	}

	false
}

pub(crate) fn validate_async(fn_sig: &syn::Signature) -> proc_macro2::TokenStream {
	if fn_sig.asyncness.is_none() {
		return syn::Error::new_spanned(fn_sig, "Function must be async").to_compile_error();
	}

	quote::quote! {}
}
