#[cfg(feature = "config")]
mod config;
#[cfg(feature = "config")]
pub use config::config;

#[cfg(feature = "adapter")]
pub fn adapter(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	use proc_macro2::Ident;
	use quote::quote;
	use syn::{ItemStruct, parse_macro_input};
	let item = parse_macro_input!(item as ItemStruct);
	let struct_name = &item.ident;

	let adapter_struct_name = Ident::new("Adapter", proc_macro2::Span::call_site());
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

		pub struct #adapter_struct_name;

		#[::puniyu_adapter::private::async_trait]
		impl ::puniyu_adapter::private::AdapterBuilder for #adapter_struct_name {
			fn name(&self) -> &str {
				::puniyu_adapter::private::AdapterBuilder::name(&#struct_name)
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
				::puniyu_adapter::private::AdapterBuilder::api(&#struct_name)
			}

			fn config(&self) -> Vec<Box<dyn ::puniyu_adapter::private::Config>> {
				::puniyu_adapter::private::inventory::iter::<crate::ConfigRegistry>
					.into_iter()
					.map(|registry| (registry.builder)())
					.collect::<Vec<_>>()
					.into()
			}

			fn hooks(&self) -> Vec<Box<dyn ::puniyu_adapter::private::HookBuilder>> {
				::puniyu_adapter::private::inventory::iter::<crate::HookRegistry>
					.into_iter()
					.map(|registry| (registry.builder)())
					.collect::<Vec<_>>()
					.into()
			}

			fn server(&self) -> Option<::puniyu_adapter::private::ServerType> {
				::puniyu_adapter::private::AdapterBuilder::server(&#struct_name)
			}

			async fn init(&self) -> ::puniyu_adapter::private::Result<()> {
				::puniyu_adapter::private::AdapterBuilder::init(&#struct_name).await
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
			builder: fn() -> Box<dyn ::puniyu_adapter::private::HookBuilder>,
		}
		::puniyu_adapter::private::inventory::collect!(HookRegistry);
	};

	proc_macro::TokenStream::from(expanded)
}
