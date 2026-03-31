use std::str::FromStr;
use crate::{
	TaskArgs,
	common::{validate_async, validate_return_type},
};
use croner::Cron;
use zyn::{ToTokens, zyn};

pub fn task(item: zyn::syn::ItemFn, cfg: TaskArgs) -> zyn::TokenStream {
	let fn_sig = item.sig.clone();
	if let Err(err) = validate_async(&fn_sig) {
		return err.to_compile_error();
	}
	if let Err(err) = validate_return_type(&fn_sig, "puniyu_plugin::Result") {
		return err.to_compile_error();
	}
	let fn_name = &item.sig.ident;

	let cron_expr = &cfg.cron;
	if Cron::from_str(cron_expr).is_err() {
		return zyn::syn::Error::new_spanned(cron_expr, "Invalid cron expression format")
			.to_compile_error();
	}

	let plugin_name = zyn! { env!("CARGO_PKG_NAME") };
	let task_name = match cfg.name {
		Some(name) => zyn! { {{ name }} },
		None => {
			zyn! { {{ fn_name | snake | str }}}
		}
	};
	let struct_name = zyn! { {{ fn_name | pascal | ident: "{}Task" | str }} };
	zyn! {
			{{ item }}

			struct {{ struct_name }}


			#[::puniyu_plugin::__private::async_trait]
			impl ::puniyu_plugin::___private::Task for {{ struct_name } {
				fn name(&self) -> &str {
					#task_name
				}

				fn cron(&self) -> &str {
					#cron_expr
				}

				#[inline]
				async fn run(&self) -> ::puniyu_plugin::Result {
					{{ fn_name}}().await
				}
			}

			::puniyu_plugin::__private::inventory::submit! {
				crate::TaskRegistry {
					plugin_name: {{ plugin_name}},
					builder: || -> ::std::sync::Arc<dyn ::puniyu_plugin::__private::Task> {
						::std::sync::Arc::new({{ struct_name }} {})
					},
				}
			}
		}
	}
	.to_token_stream()
}
