mod command;
pub use command::command;
mod task;
pub use task::task;
mod config;
pub use config::derive_plugin_config;

use crate::{
	PluginArg,
	common::{ensure_lifecycle, extract_type_from_impl},
};
use quote::quote;
use syn::{Ident, ImplItem, ItemImpl, ItemStruct};

pub fn plugin_struct(item: ItemStruct, cfg: PluginArg) -> proc_macro2::TokenStream {
	let user_struct_name = &item.ident;

	let plugin_desc = match cfg.desc {
		Some(desc) => quote!(Some(#desc)),
		None => quote!(None),
	};

	let plugin_prefix = match cfg.prefix {
		Some(prefix) => quote!(Some(#prefix)),
		None => quote!(None),
	};

	let config_body = cfg.config.map(|config_ty| {
		quote! {
			fn __puniyu_configs() -> ::std::vec::Vec<::std::sync::Arc<dyn ::puniyu_plugin::config::Config>> {
				#config_ty::configs()
			}
		}
	});

	quote! {
		#item

		impl #user_struct_name {
			fn __puniyu_description() -> Option<&'static str> {
				#plugin_desc
			}

			fn __puniyu_prefix() -> Option<&'static str> {
				#plugin_prefix
			}

			#config_body
		}

		pub(crate) struct TaskRegistry {
			plugin_name: &'static str,
			builder: fn() -> ::std::sync::Arc<dyn ::puniyu_plugin::task::Task>,
		}
		::puniyu_plugin::inventory::collect!(crate::TaskRegistry);

		pub(crate) struct CommandRegistry {
			plugin_name: &'static str,
			builder: fn() -> ::std::sync::Arc<dyn ::puniyu_plugin::command::Command>,
		}
		::puniyu_plugin::inventory::collect!(crate::CommandRegistry);

		pub(crate) struct ConfigRegistry {
			plugin_name: &'static str,
			builder: fn() -> ::std::sync::Arc<dyn ::puniyu_plugin::config::Config>,
		}
		::puniyu_plugin::inventory::collect!(crate::ConfigRegistry);
	}
}

pub fn plugin_impl(mut item: ItemImpl, cfg: PluginArg) -> proc_macro2::TokenStream {
	let host_name = match extract_type_from_impl(&item) {
		Ok(name) => name,
		Err(err) => return err.to_compile_error(),
	};

	if cfg.desc.is_some() || cfg.prefix.is_some() || cfg.config.is_some() {
		return syn::Error::new(
			host_name.span(),
			"desc, prefix and config must be declared on #[plugin] struct",
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
			if let Err(err) = ensure_lifecycle(method, "puniyu_plugin::result::Result") {
				return err.to_compile_error();
			}
			if load_fn_name.is_some() {
				return syn::Error::new(method.sig.ident.span(), "duplicate #[on_load] function")
					.to_compile_error();
			}
			load_fn_name = Some(method.sig.ident.clone());
		}

		if is_on_unload {
			if let Err(err) = ensure_lifecycle(method, "puniyu_plugin::result::Result") {
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
			async fn on_load(&self) -> ::puniyu_plugin::result::Result {
				#host_name::#name().await
			}
		}
	});
	let unload_override = unload_fn_name.map(|name| {
		quote! {
			async fn on_unload(&self) -> ::puniyu_plugin::result::Result {
				#host_name::#name().await
			}
		}
	});
	let server_override = server_fn_name.map(|name| {
		quote! {
			fn server(&self) -> Option<::puniyu_plugin::server::ServerFunction> {
				#host_name::#name()
			}
		}
	});
	let config_override = config_fn_name.map(|name| {
		quote! {
			fn config(&self) -> ::std::vec::Vec<::std::sync::Arc<dyn ::puniyu_plugin::config::Config>> {
				#host_name::#name()
			}
		}
	});

	quote! {
		#item

		pub struct Plugin;

		#[::puniyu_plugin::async_trait]
		impl ::puniyu_plugin::Plugin for Plugin {
			fn name(&self) -> &str {
				env!("CARGO_PKG_NAME")
			}

			fn version(&self) -> ::puniyu_plugin::Version {
				::puniyu_plugin::Version {
					major: env!("CARGO_PKG_VERSION_MAJOR").parse::<u64>().unwrap(),
					minor: env!("CARGO_PKG_VERSION_MINOR").parse::<u64>().unwrap(),
					patch: env!("CARGO_PKG_VERSION_PATCH").parse::<u64>().unwrap(),
				}
			}

			fn description(&self) -> Option<&str> {
				#host_name::__puniyu_description()
			}

			fn prefix(&self) -> Option<&str> {
				#host_name::__puniyu_prefix()
			}

			fn tasks(&self) -> ::std::vec::Vec<::std::sync::Arc<dyn ::puniyu_plugin::task::Task>> {
				::puniyu_plugin::inventory::iter::<crate::TaskRegistry>()
					.filter(|task| task.plugin_name == env!("CARGO_PKG_NAME"))
					.map(|task| (task.builder)())
					.collect()
			}

			fn commands(&self) -> ::std::vec::Vec<::std::sync::Arc<dyn ::puniyu_plugin::command::Command>> {
				::puniyu_plugin::inventory::iter::<crate::CommandRegistry>()
					.filter(|command| command.plugin_name == env!("CARGO_PKG_NAME"))
					.map(|command| (command.builder)())
					.collect()
			}

			#config_override
			#server_override
			#load_override
			#unload_override
		}
	}
}
