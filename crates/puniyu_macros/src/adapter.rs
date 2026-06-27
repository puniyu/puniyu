mod api;
pub use api::api_fn;

use crate::AdapterArgs;
use quote::quote;
use syn::ItemStruct;

pub fn adapter_struct(item: ItemStruct, _cfg: AdapterArgs) -> proc_macro2::TokenStream {
	if let Err(err) = crate::common::ensure_valid_host(&item.ident, "Adapter") {
		return err.to_compile_error();
	}

	quote! {
		#item

		pub(crate) struct ConfigRegistry {
			pub(crate) adapter_name: &'static str,
			pub(crate) handler: fn() -> ::std::vec::Vec<::std::sync::Arc<dyn ::puniyu_adapter::config::Config>>,
		}
		impl ConfigRegistry {
			pub(crate) fn get() -> ::std::vec::Vec<::std::sync::Arc<dyn ::puniyu_adapter::config::Config>> {
				::puniyu_adapter::inventory::iter::<Self>()
					.filter(|r| r.adapter_name == env!("CARGO_PKG_NAME"))
					.flat_map(|r| (r.handler)())
					.collect()
			}
		}
		::puniyu_adapter::inventory::collect!(crate::ConfigRegistry);

		pub(crate) struct OnLoadRegistry {
			pub(crate) handler: fn() -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ::puniyu_adapter::result::Result> + Send>>,
		}
		impl OnLoadRegistry {
			pub(crate) async fn execute() -> ::puniyu_adapter::result::Result {
				if let Some(entry) = ::puniyu_adapter::inventory::iter::<Self>().next() {
					(entry.handler)().await?;
				}
				Ok(())
			}
		}
		::puniyu_adapter::inventory::collect!(crate::OnLoadRegistry);

		pub(crate) struct OnUnloadRegistry {
			pub(crate) handler: fn() -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ::puniyu_adapter::result::Result> + Send>>,
		}
		impl OnUnloadRegistry {
			pub(crate) async fn execute() -> ::puniyu_adapter::result::Result {
				if let Some(entry) = ::puniyu_adapter::inventory::iter::<Self>().next() {
					(entry.handler)().await?;
				}
				Ok(())
			}
		}
		::puniyu_adapter::inventory::collect!(crate::OnUnloadRegistry);

		pub(crate) struct ServerRegistry {
			pub(crate) handler: fn() -> Option<::puniyu_adapter::server::ServerFunction>,
		}
		impl ServerRegistry {
			pub(crate) fn get() -> Option<::puniyu_adapter::server::ServerFunction> {
				::puniyu_adapter::inventory::iter::<Self>()
					.next()
					.and_then(|r| (r.handler)())
			}
		}
		::puniyu_adapter::inventory::collect!(crate::ServerRegistry);

		pub(crate) struct ApiRegistry {
			pub(crate) adapter_name: &'static str,
			pub(crate) handler: fn() -> ::std::sync::Arc<dyn ::puniyu_adapter::AdapterApi>,
		}
		impl ApiRegistry {
			pub(crate) fn get_api() -> ::std::sync::Arc<dyn ::puniyu_adapter::AdapterApi> {
				::puniyu_adapter::inventory::iter::<Self>()
					.find(|r| r.adapter_name == env!("CARGO_PKG_NAME"))
					.map(|r| (r.handler)())
					.expect("no AdapterApi registered, use #[api] to register")
			}
		}
		::puniyu_adapter::inventory::collect!(crate::ApiRegistry);

		macro_rules! __puniyu_submit {
			(on_load, $fn:ident) => {
				::puniyu_adapter::inventory::submit! {
					crate::OnLoadRegistry {
						handler: || -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ::puniyu_adapter::result::Result> + Send>> {
							Box::pin($fn())
						}
					}
				}
			};
			(on_unload, $fn:ident) => {
				::puniyu_adapter::inventory::submit! {
					crate::OnUnloadRegistry {
						handler: || -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ::puniyu_adapter::result::Result> + Send>> {
							Box::pin($fn())
						}
					}
				}
			};
			(server, $fn:ident) => {
				::puniyu_adapter::inventory::submit! {
					crate::ServerRegistry { handler: $fn }
				}
			};
			(config, $fn:ident) => {
				::puniyu_adapter::inventory::submit! {
					crate::ConfigRegistry {
						adapter_name: env!("CARGO_PKG_NAME"),
						handler: $fn
					}
				}
			};
			(api, $fn:ident) => {
				::puniyu_adapter::inventory::submit! {
					crate::ApiRegistry {
						adapter_name: env!("CARGO_PKG_NAME"),
						handler: $fn
					}
				}
			};
		}
		pub(crate) use __puniyu_submit;

		pub struct Adapter;

		#[::puniyu_adapter::async_trait::async_trait]
		impl ::puniyu_adapter::AdapterApi for Adapter {
			#[inline]
			async fn send_message(
				&self,
				contact: &::puniyu_adapter::contact::ContactType<'_>,
				message: &::puniyu_adapter::message::Message,
			) -> ::puniyu_adapter::result::Result<::puniyu_adapter::SendMsgType> {
				crate::ApiRegistry::get_api().send_message(contact, message).await
			}

			#[inline]
			fn adapter_info(&self) -> ::puniyu_adapter::AdapterInfo {
				crate::ApiRegistry::get_api().adapter_info()
			}

			#[inline]
			fn account_info(&self) -> ::puniyu_adapter::account::AccountInfo {
				crate::ApiRegistry::get_api().account_info()
			}

			#[inline]
			async fn call_api(
				&self,
				action: &str,
				params: ::puniyu_adapter::serde_json::Value,
			) -> ::puniyu_adapter::result::Result<::puniyu_adapter::Response<::puniyu_adapter::serde_json::Value>> {
				crate::ApiRegistry::get_api().call_api(action, params).await
			}

			#[::puniyu_adapter::async_trait::async_trait]
			impl ::puniyu_adapter::Adapter for Adapter {
				#[inline]
				fn server(&self) -> Option<::puniyu_adapter::server::ServerFunction> {
					crate::ServerRegistry::get()
				}

				#[inline]
				fn config(&self) -> ::std::vec::Vec<::std::sync::Arc<dyn ::puniyu_adapter::config::Config>> {
					crate::ConfigRegistry::get()
				}

				#[inline]
				async fn on_load(&self) -> ::puniyu_adapter::result::Result {
					crate::OnLoadRegistry::execute().await
				}

				#[inline]
				async fn on_unload(&self) -> ::puniyu_adapter::result::Result {
					crate::OnUnloadRegistry::execute().await
				}
			}
		}

	}
}
