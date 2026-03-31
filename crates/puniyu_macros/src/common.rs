use zyn::{
	ToTokens,
	syn::{self, spanned::Spanned},
};

pub(crate) fn validate_async(fn_sig: &syn::Signature) -> syn::Result<()> {
	if fn_sig.asyncness.is_none() {
		return Err(syn::Error::new(fn_sig.span(), "function must be async"));
	}

	Ok(())
}

pub(crate) fn validate_return_type(fn_sig: &syn::Signature, expected: &str) -> syn::Result<()> {
	let expected = expected.replace(' ', "");

	match &fn_sig.output {
		syn::ReturnType::Type(_, ty) => {
			let is_expected = match ty.as_ref() {
				syn::Type::Path(type_path) => {
					let actual = type_path.path.to_token_stream().to_string().replace(' ', "");

					actual == expected || actual == format!("::{expected}")
				}
				_ => false,
			};

			if is_expected {
				Ok(())
			} else {
				Err(syn::Error::new(ty.span(), format!("function must return `{expected}`")))
			}
		}
		syn::ReturnType::Default => {
			Err(syn::Error::new(fn_sig.span(), format!("function must return `{expected}`")))
		}
	}
}
