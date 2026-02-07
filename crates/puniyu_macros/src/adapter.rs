#[cfg(feature = "config")]
mod config;
#[cfg(feature = "config")]
pub use config::config;
#[cfg(feature = "adapter")]
mod api;
#[cfg(feature = "adapter")]
pub use api::api;
#[cfg(feature = "hook")]
mod hook;
#[cfg(feature = "hook")]
pub use hook::hook;
#[cfg(feature = "server")]
mod server;
#[cfg(feature = "server")]
pub use server::server;
mod info;

#[cfg(feature = "adapter")]
#[derive(Debug, darling::FromMeta)]
struct AdapterArg {
	info: syn::Path,
}

#[cfg(feature = "adapter")]
impl quote::ToTokens for AdapterArg {
	fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
		self.info.to_tokens(tokens);
	}
}
#[cfg(feature = "adapter")]
pub fn adapter(
	args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	use darling::ast::NestedMeta;
	use darling::{Error, FromMeta};
	use proc_macro2::Ident;
	use quote::quote;
	use syn::{ItemFn, parse_macro_input};

	let attr_args = match NestedMeta::parse_meta_list(args.into()) {
		Ok(v) => v,
		Err(e) => return proc_macro::TokenStream::from(Error::from(e).write_errors()),
	};
	let item = parse_macro_input!(item as ItemFn);
	let fn_sig = &item.sig;
	let fn_name = &fn_sig.ident;
	let fn_block = &item.block;
	let is_async = fn_sig.asyncness.is_some();
	if !is_async {
		return syn::Error::new_spanned(fn_sig, "Function must be async").to_compile_error().into();
	}

	match &fn_sig.output {
		syn::ReturnType::Type(_, return_type) => {
			let is_adapter_api = if let syn::Type::Path(type_path) = &**return_type {
				if let Some(segment) = type_path.path.segments.last() {
					segment.ident == "HandlerResult"
				} else {
					false
				}
			} else {
				false
			};
			if !is_adapter_api {
				return syn::Error::new_spanned(
					return_type,
					"Function must return `HandlerResult` type",
				)
				.to_compile_error()
				.into();
			}
		}
		syn::ReturnType::Default => {
			return syn::Error::new_spanned(fn_sig, "Function must have an explicit return type")
				.to_compile_error()
				.into();
		}
	}

	let args = match AdapterArg::from_list(&attr_args) {
		Ok(v) => v,
		Err(e) => return proc_macro::TokenStream::from(e.write_errors()),
	};
	let adapter_info = quote! {
		(#args.info)()
	};

	let adapter_name = quote! { env!("CARGO_PKG_NAME") };
	let struct_name = Ident::new("Adapter", proc_macro2::Span::call_site());
	let init_call = if fn_block.stmts.is_empty() {
		quote! {
			<Self as ::puniyu_adapter::__private::Adapter>::init(self)
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

		#[::puniyu_adapter::__private::async_trait]
		impl ::puniyu_adapter::__private::Adapter for #struct_name {
			fn info(&self) -> ::puniyu_adapter::__private::AdapterInfo {
				#adapter_info
			}

			fn api(&self) -> ::puniyu_adapter::__private::AdapterApi {
				let api = ::puniyu_adapter::__private::inventory::iter::<crate::ApiRegistry>
					.into_iter()
					.find(|registry| registry.adapter_name == #adapter_name);

				match api {
					Some(registry) => (registry.builder)(),
					None => {
						compile_error!(concat!("No ApiRegistry found for adapter: ", env!("CARGO_PKG_NAME")));
					}
				}
			}

			fn config(&self) -> Vec<Box<dyn ::puniyu_adapter::__private::Config>> {
				::puniyu_adapter::__private::inventory::iter::<crate::ConfigRegistry>
					.into_iter()
					.filter(|registry| registry.adapter_name == #adapter_name)
					.map(|registry| (registry.builder)())
					.collect()
			}

			fn hooks(&self) -> Vec<Box<dyn ::puniyu_adapter::__private::Hook>> {
				::puniyu_adapter::__private::inventory::iter::<crate::HookRegistry>
					.into_iter()
					.filter(|registry| registry.adapter_name == #adapter_name)
					.map(|registry| (registry.builder)())
					.collect()
			}

			fn server(&self) -> Option<::puniyu_adapter::__private::ServerFunction> {
				let servers: Vec<_> = ::puniyu_adapter::__private::inventory::iter::<crate::ServerRegistry>
					.into_iter()
					.filter(|server| server.adapter_name == #adapter_name)
					.map(|server| (server.builder)())
					.collect();

				if !servers.is_empty() {
					Some(::std::sync::Arc::new(move |cfg: &mut ::puniyu_adapter::__private::ServiceConfig| {
						servers.iter().for_each(|server| server(cfg));
					}))
				} else {
					None
				}
			}

			async fn init(&self) -> ::puniyu_adapter::__private::HandlerResult {
				#init_call.await
			}
		}

		/// 配置注册表
		pub(crate) struct ConfigRegistry {
			adapter_name: &'static str,
			/// 配置构造器
			builder: fn() -> Box<dyn ::puniyu_adapter::__private::Config>,
		}
		::puniyu_adapter::__private::inventory::collect!(crate::ConfigRegistry);

		/// 钩子注册注册表
		pub(crate) struct HookRegistry {
			adapter_name: &'static str,
			/// 钩子构造器
			builder: fn() -> Box<dyn ::puniyu_adapter::__private::Hook>,
		}
		::puniyu_adapter::__private::inventory::collect!(crate::HookRegistry);
				/// 服务器注册表
		pub(crate) struct ServerRegistry {
			/// 插件名称
			adapter_name: &'static str,
			/// 服务器配置构造器
			builder: fn() -> ::puniyu_adapter::__private::ServerFunction,
		}
		::puniyu_adapter::__private::inventory::collect!(crate::ServerRegistry);

		pub(crate) struct ApiRegistry {
			/// 插件名称
			adapter_name: &'static str,
			/// 服务器配置构造器
			builder: ::puniyu_adapter::__private::AdapterApi,
		}
		::puniyu_adapter::__private::inventory::collect!(crate::ApiRegistry);
	};

	proc_macro::TokenStream::from(expanded)
}
