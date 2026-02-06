use convert_case::{Case, Casing};
use croner::Cron;
use darling::ast::NestedMeta;
use darling::{Error, FromMeta};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use std::str::FromStr;
use syn::{ItemFn, parse_macro_input};

#[derive(Debug, Default, FromMeta)]
struct TaskArgs {
	pub name: Option<String>,
	pub cron: String,
}

pub fn task(args: TokenStream, item: TokenStream) -> TokenStream {
	let attr_args = match NestedMeta::parse_meta_list(args.into()) {
		Ok(v) => v,
		Err(e) => return TokenStream::from(Error::from(e).write_errors()),
	};

	let item = parse_macro_input!(item as ItemFn);
	let fn_name = &item.sig.ident;

	if item.sig.asyncness.is_none() {
		return syn::Error::new_spanned(&item.sig, "function must be async")
			.to_compile_error()
			.into();
	}
	if !item.sig.inputs.is_empty() {
		return syn::Error::new_spanned(
			&item.sig.inputs,
			"Task function should not have parameters",
		)
		.to_compile_error()
		.into();
	}

	let args = match TaskArgs::from_list(&attr_args) {
		Ok(v) => v,
		Err(e) => return TokenStream::from(e.write_errors()),
	};

	let cron_expr = &args.cron;
	if Cron::from_str(cron_expr).is_err() {
		return syn::Error::new_spanned(cron_expr, "Invalid cron expression format")
			.to_compile_error()
			.into();
	}

	let plugin_name = quote! { env!("CARGO_PKG_NAME") };
	let task_name = match args.name {
		Some(name) => quote! { #name },
		None => {
			use convert_case::{Case, Casing};
			let name = fn_name.to_string().to_case(Case::Lower);
			quote! { #name }
		}
	};
	let struct_name_str = {
		let fn_name_str = fn_name.to_string();
		let pascal_case_name = fn_name_str.to_case(Case::Pascal);
		format!("{}Task", pascal_case_name)
	};
	let struct_name = Ident::new(&struct_name_str, fn_name.span());

	let expanded = quote! {
		#item

		#[allow(non_camel_case_types)]
		struct #struct_name;

		#[::puniyu_plugin::private::async_trait]
		impl ::puniyu_plugin::private::Task for #struct_name {
			fn name(&self) -> &str {
				#task_name
			}

			fn cron(&self) -> &str {
				#cron_expr
			}

			async fn run(&self) -> ::puniyu_plugin::private::HandlerResult {
				#fn_name().await
			}
		}

		::puniyu_plugin::private::inventory::submit! {
			crate::TaskRegistry {
				plugin_name: #plugin_name,
				builder: || -> Box<dyn ::puniyu_plugin::private::Task> {
					Box::new(#struct_name {})
				},
			}
		}
	};

	expanded.into()
}
