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

pub(crate) fn validate_hook_args(fn_sig: &syn::Signature) -> syn::Result<()> {
	let fn_name = &fn_sig.ident;
	let inputs = &fn_sig.inputs;
	if inputs.len() != 1 {
		return Err(syn::Error::new(
			fn_sig.span(),
			format!("function `{}` must have exactly 1 parameter, found {}", fn_name, inputs.len()),
		));
	}

	let arg = inputs.first().unwrap();
	let pat_type = match arg {
		syn::FnArg::Typed(pat_type) => pat_type,
		syn::FnArg::Receiver(_) => {
			return Err(syn::Error::new(arg.span(), "hook function parameter must not be `self`"));
		}
	};

	let inner_type = match pat_type.ty.as_ref() {
		syn::Type::Reference(type_ref) => &type_ref.elem,
		_ => {
			return Err(syn::Error::new(
				pat_type.ty.span(),
				"hook function parameter must be a reference: `&puniyu_plugin::hook::HookType` or `&puniyu_adapter::hook::HookType`",
			));
		}
	};

	let is_valid = match inner_type.as_ref() {
		syn::Type::Path(type_path) => {
			let path_str = type_path.path.to_token_stream().to_string().replace(' ', "");
			matches!(
				path_str.as_str(),
				"HookType"
					| "puniyu_plugin::hook::HookType"
					| "::puniyu_plugin::hook::HookType"
					| "puniyu_adapter::hook::HookType"
					| "::puniyu_adapter::hook::HookType"
			)
		}
		_ => false,
	};

	if !is_valid {
		return Err(syn::Error::new(
			inner_type.span(),
			"hook function parameter type must be `puniyu_plugin::hook::HookType` or `puniyu_adapter::hook::HookType`",
		));
	}

	Ok(())
}

pub(crate) fn validate_return_type(fn_sig: &syn::Signature, expected: &str) -> syn::Result<()> {
	let expected = expected.replace(' ', "");
	let err = |span| Err(syn::Error::new(span, format!("function must return `{expected}`")));

	let syn::ReturnType::Type(_, ty) = &fn_sig.output else {
		return err(fn_sig.span());
	};
	let syn::Type::Path(type_path) = ty.as_ref() else {
		return err(ty.span());
	};
	let actual = type_path.path.to_token_stream().to_string().replace(' ', "");
	if actual == expected || actual == format!("::{expected}") { Ok(()) } else { err(ty.span()) }
}
