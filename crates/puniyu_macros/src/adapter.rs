use crate::{
	AdapterArgs,
	common::{ensure_lifecycle, ensure_valid_host, extract_type_from_impl},
};
use quote::quote;
use syn::{Ident, ImplItem, ItemImpl, ItemStruct};
mod config;
pub use config::derive_adapter_config;

pub fn adapter_struct(item: ItemStruct, cfg: AdapterArgs) -> proc_macro2::TokenStream {
	if let Err(err) = ensure_valid_host(&item, "Adapter") {
		return err.to_compile_error();
	}

	let user_struct_name = &item.ident;

	let config_body = cfg.config.map(|config_ty| {
		quote! {
			fn __puniyu_configs() -> ::std::vec::Vec<::std::sync::Arc<dyn ::puniyu_adapter::config::Config>> {
				#config_ty::configs()
			}
		}
	});

	quote! {
		#item

		impl #user_struct_name {
			#config_body
		}

		pub(crate) struct ConfigRegistry {
			adapter_name: &'static str,
			builder: fn() -> ::std::sync::Arc<dyn ::puniyu_adapter::config::Config>,
		}
		::puniyu_adapter::inventory::collect!(crate::ConfigRegistry);
	}
}

pub fn adapter_impl(mut item: ItemImpl, cfg: AdapterArgs) -> proc_macro2::TokenStream {
	let host_name = match extract_type_from_impl(&item) {
		Ok(name) => name,
		Err(err) => return err.to_compile_error(),
	};

	if cfg.config.is_some() {
		return syn::Error::new(
			host_name.span(),
			"config = ... must be declared on #[adapter] struct",
		)
		.to_compile_error();
	}

	let mut load_fn_name: Option<Ident> = None;
	let mut unload_fn_name: Option<Ident> = None;
	let mut server_fn_name: Option<Ident> = None;
	let mut config_fn_name: Option<Ident> = None;

	for impl_item in &mut item.items {
		let ImplItem::Fn(method) = impl_item else {
			continue;
		};
		let mut retained = Vec::new();
		let mut is_on_load = false;
		let mut is_on_unload = false;
		let mut is_server = false;
		let mut is_config = false;
		for attr in method.attrs.drain(..) {
			if attr.path().is_ident("on_load") {
				is_on_load = true;
			} else if attr.path().is_ident("on_unload") {
				is_on_unload = true;
			} else if attr.path().is_ident("server") {
				is_server = true;
			} else if attr.path().is_ident("config") {
				is_config = true;
			} else {
				retained.push(attr);
			}
		}
		method.attrs = retained;

		if is_on_load {
			if let Err(err) = ensure_lifecycle(method, "puniyu_adapter::result::Result") {
				return err.to_compile_error();
			}
			if load_fn_name.is_some() {
				return syn::Error::new(method.sig.ident.span(), "duplicate #[on_load] function")
					.to_compile_error();
			}
			load_fn_name = Some(method.sig.ident.clone());
		}

		if is_on_unload {
			if let Err(err) = ensure_lifecycle(method, "puniyu_adapter::result::Result") {
				return err.to_compile_error();
			}
			if unload_fn_name.is_some() {
				return syn::Error::new(method.sig.ident.span(), "duplicate #[on_unload] function")
					.to_compile_error();
			}
			unload_fn_name = Some(method.sig.ident.clone());
		}

		if is_server {
			if server_fn_name.is_some() {
				return syn::Error::new(method.sig.ident.span(), "duplicate #[server] function")
					.to_compile_error();
			}
			server_fn_name = Some(method.sig.ident.clone());
		}

		if is_config {
			if config_fn_name.is_some() {
				return syn::Error::new(method.sig.ident.span(), "duplicate #[config] function")
					.to_compile_error();
			}
			config_fn_name = Some(method.sig.ident.clone());
		}
	}

	let load_override = load_fn_name.map(|name| {
		quote! {
			async fn on_load(&self) -> ::puniyu_adapter::result::Result {
				#host_name::#name().await
			}
		}
	});
	let unload_override = unload_fn_name.map(|name| {
		quote! {
			async fn on_unload(&self) -> ::puniyu_adapter::result::Result {
				#host_name::#name().await
			}
		}
	});
	let server_override = server_fn_name.map(|name| {
		quote! {
			fn server(&self) -> Option<::puniyu_adapter::server::ServerFunction> {
				#host_name::#name()
			}
		}
	});
	let config_override = config_fn_name.map(|name| {
		quote! {
			fn config(&self) -> ::std::vec::Vec<::std::sync::Arc<dyn ::puniyu_adapter::config::Config>> {
				#host_name::#name()
			}
		}
	});

	quote! {
		#item

		pub struct Adapter;

		#[::puniyu_adapter::async_trait::async_trait]
		impl ::puniyu_adapter::Adapter for Adapter {
			#config_override
			#server_override
			#load_override
			#unload_override
		}
	}
}
