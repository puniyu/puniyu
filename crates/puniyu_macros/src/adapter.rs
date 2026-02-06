#[cfg(feature = "config")]
mod config;
#[cfg(feature = "config")]
pub use config::config;
#[cfg(feature = "hook")]
mod hook;
#[cfg(feature = "hook")]
pub use hook::hook;

#[cfg(feature = "adapter")]
pub fn adapter(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	use proc_macro2::Ident;
	use quote::quote;
	use syn::{ItemStruct, parse_macro_input};
	let item = parse_macro_input!(item as ItemStruct);
	let fn_name = &item.ident;

	let adapter_name = quote! { env!("CARGO_PKG_NAME") };
	let struct_name = Ident::new("Adapter", proc_macro2::Span::call_site());
	let version_major = quote! { env!("CARGO_PKG_VERSION_MAJOR").parse::<u16>().unwrap() };
	let version_minor = quote! { env!("CARGO_PKG_VERSION_MINOR").parse::<u16>().unwrap() };
	let version_patch = quote! { env!("CARGO_PKG_VERSION_PATCH").parse::<u16>().unwrap() };
	let adapter_author = quote! {
		{
			let authors = env!("CARGO_PKG_AUTHORS");
			if authors.is_empty() {
				None
			} else {
				let first_author = authors.split(':').next().unwrap_or(authors);
				Some(first_author)
			}
		}
	};
	let expanded = quote! {
		#item

		pub struct #struct_name;

		#[::puniyu_adapter::private::async_trait]
		impl ::puniyu_adapter::private::Adapter for #struct_name {
			fn name(&self) -> &str {
				#adapter_name
			}

			fn version(&self) -> ::puniyu_adapter::private::Version {
				::puniyu_adapter::private::Version {
					major: #version_major,
					minor: #version_minor,
					patch: #version_patch,
				}
			}

			fn author(&self) -> Option<&str> {
				#adapter_author
			}

			fn api(&self) -> ::puniyu_adapter::private::AdapterApi {
				::puniyu_adapter::private::Adapter::api(&#fn_name)
			}

			fn config(&self) -> Vec<Box<dyn ::puniyu_adapter::private::Config>> {
				::puniyu_adapter::private::inventory::iter::<crate::ConfigRegistry>
					.into_iter()
					.filter(|registry| registry.adapter_name == #adapter_name)
					.map(|registry| (registry.builder)())
					.collect()
			}

			fn hooks(&self) -> Vec<Box<dyn ::puniyu_adapter::private::Hook>> {
				::puniyu_adapter::private::inventory::iter::<crate::HookRegistry>
					.into_iter()
					.filter(|registry| registry.adapter_name == #adapter_name)
					.map(|registry| (registry.builder)())
					.collect()
			}

			fn server(&self) -> Option<::puniyu_adapter::private::ServerFunction> {
				::puniyu_adapter::private::AdapterBuilder::server(&#fn_name)
			}

			async fn init(&self) -> ::puniyu_adapter::private::Result<()> {
				::puniyu_adapter::private::AdapterBuilder::init(&#fn_name).await
			}
		}

		/// 配置注册表
		pub(crate) struct ConfigRegistry {
			adapter_name: &'static str,
			/// 配置构造器
			builder: fn() -> Box<dyn ::puniyu_adapter::private::Config>,
		}
		::puniyu_adapter::private::inventory::collect!(ConfigRegistry);

		/// 钩子注册注册表
		pub(crate) struct HookRegistry {
			adapter_name: &'static str,
			/// 钩子构造器
			builder: fn() -> Box<dyn ::puniyu_adapter::private::Hook>,
		}
		::puniyu_adapter::private::inventory::collect!(HookRegistry);
	};

	proc_macro::TokenStream::from(expanded)
}
