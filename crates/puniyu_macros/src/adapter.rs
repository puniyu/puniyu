mod config;
pub use config::config;
mod hook;
pub use hook::hook;

use crate::AdapterArgs;
use crate::common::{validate_async, validate_return_type};
use zyn::{ToTokens, zyn};
pub fn adapter(item: zyn::syn::ItemFn, cfg: AdapterArgs) -> zyn::TokenStream {
	if let Err(err) = validate_async(&item.sig) {
		return err.to_compile_error();
	}
	if let Err(err) = validate_return_type(&item.sig, "puniyu_adapter::Result") {
		return err.to_compile_error();
	}

	let adapter_name = zyn! { env!("CARGO_PKG_NAME") };
	let fn_name = &item.sig.ident;
	let init_impl = if item.block.stmts.is_empty() {
		zyn! {}
	} else {
		zyn! {
			#[inline]
			async fn init(&self) -> ::puniyu_adapter::Result {
				{{ fn_name }}().await
			}
		}
	};

	zyn! {
		{{ item }}

		pub struct Adapter;

		#[::puniyu_adapter::__private::async_trait]
		impl ::puniyu_adapter::__private::Adapter for Adapter {
			fn info(&self) -> ::puniyu_adapter::AdapterInfo {
				{{ &cfg.info }}()
			}
			fn runtime(&self) -> ::std::sync::Arc<dyn ::puniyu_adapter::__private::FrameworkRuntime> {
				{{ &cfg.runtime }}()
			}
			fn config(&self) -> Vec<::std::sync::Arc<dyn ::puniyu_adapter::__private::Config>> {
				::puniyu_adapter::__private::inventory::iter::<crate::ConfigRegistry>
					.into_iter()
					.filter(|registry| registry.adapter_name == {{ adapter_name}})
					.map(|registry| (registry.builder)())
					.collect()
			}

			fn hook(&self) -> Vec<::std::sync::Arc<dyn ::puniyu_adapter::__private::Hook>> {
				::puniyu_adapter::__private::inventory::iter::<crate::HookRegistry>
					.into_iter()
					.filter(|registry| registry.adapter_name == {{ adapter_name }})
					.map(|registry| (registry.builder)())
					.collect()
			}

			fn server(&self) -> Option<::puniyu_adapter::__private::ServerFunction> {
					@if(cfg.server.is_some()){
						::puniyu_adapter::__private::ServerFunction::new(&cfg.server)
					}
					@else{
						None
					}
			}

			{{ init_impl }}
		}

		/// 配置注册表
		pub(crate) struct ConfigRegistry {
			adapter_name: &'static str,
			/// 配置构造器
			builder: fn() -> ::std::sync::Arc<dyn ::puniyu_adapter::__private::Config>,
		}
		::puniyu_adapter::__private::inventory::collect!(crate::ConfigRegistry);

		/// 钩子注册注册表
		pub(crate) struct HookRegistry {
			adapter_name: &'static str,
			/// 钩子构造器
			builder: fn() -> ::std::sync::Arc<dyn ::puniyu_adapter::__private::Hook>,
		}
		::puniyu_adapter::__private::inventory::collect!(crate::HookRegistry);

	}
	.to_token_stream()
}
