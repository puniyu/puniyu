use crate::common::ensure_no_params;
use quote::quote;
use syn::ItemFn;

pub fn api_fn(item: ItemFn) -> proc_macro2::TokenStream {
	if let Err(err) = ensure_no_params(&item.sig) {
		return err.to_compile_error();
	}

	let fn_name = &item.sig.ident;
	quote! {
		#item

		crate::__puniyu_submit!(api, #fn_name);
	}
}
