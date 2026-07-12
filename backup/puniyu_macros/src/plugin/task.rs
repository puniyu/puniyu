use crate::{common::{build_wrapper_name, ensure_async, ensure_no_params, ensure_return_type, to_snake_case}, TaskArgs};
use croner::Cron;
use quote::quote;
use std::str::FromStr;
use syn::ItemFn;

pub fn task(item: ItemFn, cfg: TaskArgs) -> proc_macro2::TokenStream {
	let fn_sig = item.sig.clone();
	if let Err(err) = ensure_async(&fn_sig) {
		return err.to_compile_error();
	}
	if let Err(err) = ensure_no_params(&fn_sig) {
		return err.to_compile_error();
	}
	if let Err(err) = ensure_return_type(&fn_sig, "puniyu_plugin::result::Result") {
		return err.to_compile_error();
	}

	let fn_name = &item.sig.ident;
	let cron_expr = cfg.cron;
	if Cron::from_str(&cron_expr.value()).is_err() {
		return syn::Error::new_spanned(&cron_expr, "Invalid cron expression format")
			.to_compile_error();
	}

	let task_name = cfg.name.unwrap_or_else(|| to_snake_case(fn_name));
	let struct_name = build_wrapper_name(fn_name, "Task");

	quote! {
		#item

		struct #struct_name;

		#[::puniyu_plugin::async_trait]
		impl ::puniyu_plugin::Task for #struct_name {
			fn name(&self) -> &'static str {
				#task_name
			}

			fn cron(&self) -> &'static str {
				#cron_expr
			}

			#[inline]
			async fn execute(&self) -> ::puniyu_plugin::result::Result {
				#fn_name().await
			}
		}

		crate::__puniyu_submit!(task, #struct_name);
	}
}
