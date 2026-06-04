use convert_case::{Case, Casing};
use quote::ToTokens;
use syn::{FnArg, Ident, ImplItemFn, ItemImpl, ItemStruct, Result, Signature, Type, spanned::Spanned};


/// 函数必须是 async。
pub(crate) fn ensure_async(fn_sig: &Signature) -> Result<()> {
	if fn_sig.asyncness.is_none() {
		return Err(syn::Error::new(fn_sig.span(), "function must be async"));
	}
	Ok(())
}

/// 函数必须无参数。
pub(crate) fn ensure_no_params(fn_sig: &Signature) -> Result<()> {
	if !fn_sig.inputs.is_empty() {
		return Err(syn::Error::new(fn_sig.span(), "function must not have parameters"));
	}
	Ok(())
}

/// 函数返回类型必须是 `expected`。
pub(crate) fn ensure_return_type(fn_sig: &Signature, expected: &str) -> Result<()> {
	let expected = expected.replace(' ', "");
	let err = |span| Err(syn::Error::new(span, format!("function must return `{expected}`")));

	let syn::ReturnType::Type(_, ty) = &fn_sig.output else {
		return err(fn_sig.span());
	};
	let Type::Path(type_path) = ty.as_ref() else {
		return err(ty.span());
	};
	let actual = type_path.path.to_token_stream().to_string().replace(' ', "");
	if actual == expected || actual == format!("::{expected}") {
		Ok(())
	} else {
		err(ty.span())
	}
}

pub(crate) fn ensure_lifecycle(fn_item: &ImplItemFn, expected: &str) -> Result<()> {
	ensure_async(&fn_item.sig)?;
	ensure_no_params(&fn_item.sig)?;
	ensure_return_type(&fn_item.sig, expected)
}


pub(crate) fn ensure_valid_host(item: &ItemStruct, reserved: &str) -> Result<()> {
	if item.ident == reserved {
		return Err(syn::Error::new(
			item.ident.span(),
			format!(
				"struct name `{reserved}` is reserved for the generated wrapper; rename your struct"
			),
		));
	}
	Ok(())
}


pub(crate) fn ensure_single_ref_param<'a>(
	fn_sig: &'a Signature,
	receiver_err: &str,
) -> Result<&'a Type> {
	if fn_sig.inputs.len() != 1 {
		return Err(syn::Error::new(
			fn_sig.span(),
			format!(
				"function `{}` must have exactly 1 parameter, found {}",
				fn_sig.ident,
				fn_sig.inputs.len()
			),
		));
	}

	let arg = fn_sig.inputs.first().expect("checked len");
	let pat_type = match arg {
		FnArg::Typed(pat_type) => pat_type,
		FnArg::Receiver(_) => return Err(syn::Error::new(arg.span(), receiver_err)),
	};

	match pat_type.ty.as_ref() {
		Type::Reference(type_ref) => Ok(type_ref.elem.as_ref()),
		_ => {
			Err(syn::Error::new(pat_type.ty.span(), "function parameter must be a reference type"))
		}
	}
}


pub(crate) fn build_wrapper_name(fn_name: &Ident, suffix: &str) -> Ident {
	Ident::new(
		&format!("{}{}", fn_name.to_string().to_case(Case::Pascal), suffix),
		fn_name.span(),
	)
}


pub(crate) fn to_snake_case(ident: &Ident) -> String {
	ident.to_string().to_case(Case::Snake)
}



pub(crate) fn extract_type_from_impl(item: &ItemImpl) -> Result<Ident> {
	let Type::Path(type_path) = item.self_ty.as_ref() else {
		return Err(syn::Error::new(item.self_ty.span(), "impl target must be a named type"));
	};
	let Some(segment) = type_path.path.segments.last() else {
		return Err(syn::Error::new(type_path.path.span(), "impl target must not be empty"));
	};
	Ok(segment.ident.clone())
}

/// 类型路径末尾段是否与 `expected_segments` 末尾一致。
/// 用于判断 `puniyu_plugin::result::Result` 和 `::puniyu_plugin::result::Result` 是否匹配。
pub(crate) fn type_ends_with(ty: &Type, expected_segments: &[&str]) -> bool {
	let Type::Path(type_path) = ty else {
		return false;
	};
	let actual: Vec<String> = type_path
		.path
		.segments
		.iter()
		.map(|segment| segment.ident.to_string())
		.collect();
	if actual == expected_segments.iter().map(|s| (*s).to_string()).collect::<Vec<_>>() {
		return true;
	}
	if actual
		.last()
		.is_some_and(|segment| segment == expected_segments.last().copied().unwrap_or_default())
		&& expected_segments.len() == 1
	{
		return true;
	}
	false
}


