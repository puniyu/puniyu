#[cfg(feature = "config")]
mod config;

#[cfg(feature = "config")]
pub use config::config;
#[cfg(feature = "hook")]
mod hook;
#[cfg(feature = "hook")]
pub use hook::hook;
#[cfg(feature = "server")]
mod server;
#[cfg(feature = "server")]
pub use server::server;


use crate::common::{validate_async, validate_return_type};

#[cfg(feature = "adapter")]
pub fn adapter(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	use proc_macro2::Ident;
	use quote::quote;
	use syn::{ItemFn, parse_macro_input};
	use darling::FromMeta;
	use darling::ast::NestedMeta;

	#[derive(Debug, FromMeta)]
	struct AdapterArgs {
		info: syn::Path,
		api: syn::Path,
	}

	let attr_args = match NestedMeta::parse_meta_list(attr.into()) {
		Ok(v) => v,
		Err(e) => return proc_macro::TokenStream::from(darling::Error::from(e).write_errors()),
	};
	let args = match AdapterArgs::from_list(&attr_args) {
		Ok(v) => v,
		Err(e) => return proc_macro::TokenStream::from(e.write_errors()),
	};

	let item = parse_macro_input!(item as ItemFn);
	let fn_sig = &item.sig;
	let fn_name = &fn_sig.ident;
	let fn_block = &item.block;
	validate_async(fn_sig);
	validate_return_type(fn_sig, "Result");

	let info = &args.info;
	let api = &args.api;
	let adapter_name = quote! { env!("CARGO_PKG_NAME") };
	let struct_name = Ident::new("Adapter", proc_macro2::Span::call_site());
	let init_call = if fn_block.stmts.is_empty() {
		quote! {
			<Self as ::puniyu_adapter_api::__private::Adapter>::init(self)
		}
	} else {
		quote! {
			async {
				#fn_name().await
			}
		}
	};

	let expanded = quote! {
		#item

		pub struct #struct_name;

		#[::puniyu_adapter_api::__private::async_trait]
		impl ::puniyu_adapter_api::__private::Adapter for #struct_name {
			fn info(&self) -> ::puniyu_adapter_api::AdapterInfo {
				#info()
			}

			fn api(&self) -> ::puniyu_adapter_api::AdapterApi {
				#api()
			}

			fn config(&self) -> Vec<::std::sync::Arc<dyn ::puniyu_adapter_api::config::Config>> {
				::puniyu_adapter_api::__private::inventory::iter::<crate::ConfigRegistry>
					.into_iter()
					.filter(|registry| registry.adapter_name == #adapter_name)
					.map(|registry| (registry.builder)())
					.collect()
			}

			fn hooks(&self) -> Vec<::std::sync::Arc<dyn ::puniyu_adapter_api::hook::Hook>> {
				::puniyu_adapter_api::__private::inventory::iter::<crate::HookRegistry>
					.into_iter()
					.filter(|registry| registry.adapter_name == #adapter_name)
					.map(|registry| (registry.builder)())
					.collect()
			}

			fn server(&self) -> Option<::puniyu_adapter_api::server::ServerFunction> {
				let servers: Vec<_> = ::puniyu_adapter_api::__private::inventory::iter::<crate::ServerRegistry>
					.into_iter()
					.filter(|server| server.adapter_name == #adapter_name)
					.map(|server| (server.builder)())
					.collect();

				if !servers.is_empty() {
					Some(::std::sync::Arc::new(move |cfg: &mut ::puniyu_adapter_api::server::ServiceConfig| {
						servers.iter().for_each(|server| server(cfg));
					}))
				} else {
					None
				}
			}

			async fn init(&self) -> ::puniyu_adapter_api::Result {
				#init_call.await
			}
		}

		/// 配置注册表
		pub(crate) struct ConfigRegistry {
			adapter_name: &'static str,
			/// 配置构造器
			builder: fn() -> ::std::sync::Arc<dyn ::puniyu_adapter_api::config::Config>,
		}
		::puniyu_adapter_api::__private::inventory::collect!(crate::ConfigRegistry);

		/// 钩子注册注册表
		pub(crate) struct HookRegistry {
			adapter_name: &'static str,
			/// 钩子构造器
			builder: fn() -> ::std::sync::Arc<dyn ::puniyu_adapter_api::hook::Hook>,
		}
		::puniyu_adapter_api::__private::inventory::collect!(crate::HookRegistry);
		/// 服务器注册表
		pub(crate) struct ServerRegistry {
			/// 插件名称
			adapter_name: &'static str,
			/// 服务器配置构造器
			builder: fn() -> ::puniyu_adapter_api::server::ServerFunction,
		}
		::puniyu_adapter_api::__private::inventory::collect!(crate::ServerRegistry);
	};

	proc_macro::TokenStream::from(expanded)
}
