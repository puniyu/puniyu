use convert_case::{Case, Casing};
use darling::ast::NestedMeta;
use darling::{Error, FromMeta};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[derive(FromMeta)]
struct HookArg {
	name: Option<String>,
	#[darling(rename = "type")]
	r#type: Option<String>,
	rank: Option<u64>,
}

pub fn hook(args: TokenStream, item: TokenStream) -> TokenStream {
	let attr_args = match NestedMeta::parse_meta_list(args.into()) {
		Ok(v) => v,
		Err(e) => return TokenStream::from(Error::from(e).write_errors()),
	};
	let item = parse_macro_input!(item as ItemFn);
	let args = match HookArg::from_list(&attr_args) {
		Ok(v) => v,
		Err(e) => return TokenStream::from(e.write_errors()),
	};

	let fn_name = &item.sig.ident;
	let struct_name_str = {
		let fn_name_str = fn_name.to_string();
		let pascal_case_name = fn_name_str.to_case(Case::Pascal);
		format!("{}Hook", pascal_case_name)
	};
	let struct_name = Ident::new(&struct_name_str, fn_name.span());
	let adapter_name = quote! { env!("CARGO_PKG_NAME") };
	let hook_name = match &args.name {
		Some(name) => quote! { #name },
		_ => quote! { stringify!(#fn_name) },
	};
	let hook_type = match &args.r#type {
		Some(type_str) => {
			let parts: Vec<&str> = type_str.split('.').collect();
			match parts.as_slice() {
				["event"] => quote! {
					::puniyu_adapter::private::HookType::Event(
						::puniyu_adapter::private::HookEventType::default()
					)
				},
				["event", "message"] => quote! {
					::puniyu_adapter::private::HookType::Event(
						::puniyu_adapter::private::HookEventType::Message
					)
				},
				["event", "notion"] => quote! {
					::puniyu_adapter::private::HookType::Event(
						::puniyu_adapter::private::HookEventType::Notion
					)
				},
				["event", "request"] => quote! {
					::puniyu_adapter::private::HookType::Event(
						::puniyu_adapter::private::HookEventType::Request
					)
				},
				["event", "all"] => quote! {
					::puniyu_adapter::private::HookType::Event(
						::puniyu_adapter::private::HookEventType::All
					)
				},
				["status"] => quote! {
					::puniyu_adapter::private::HookType::Status(
						::puniyu_adapter::private::StatusType::default()
					)
				},
				["status", "start"] => quote! {
					::puniyu_adapter::private::HookType::Status(
						::puniyu_adapter::private::StatusType::Start
					)
				},
				["status", "stop"] => quote! {
					::puniyu_adapter::private::HookType::Status(
						::puniyu_adapter::private::StatusType::Stop
					)
				},
				["event", subtype] => {
					let err_msg = format!(
						"Invalid event subtype '{}'. Valid event subtypes are: 'message', 'notion', 'request', 'all'. \
						Examples: 'event.message', 'event.notion'",
						subtype
					);
					return TokenStream::from(Error::custom(err_msg).write_errors());
				}
				["status", subtype] => {
					let err_msg = format!(
						"Invalid status subtype '{}'. Valid status subtypes are: 'start', 'stop'. \
						Examples: 'status.start', 'status.stop'",
						subtype
					);
					return TokenStream::from(Error::custom(err_msg).write_errors());
				}
				[category, _] => {
					let err_msg = format!(
						"Invalid hook category '{}'. Valid categories are: 'event', 'status'. \
						Examples: 'event.message', 'status.start'",
						category
					);
					return TokenStream::from(Error::custom(err_msg).write_errors());
				}
				[category] => {
					let err_msg = format!(
						"Invalid hook category '{}'. Valid categories are: 'event', 'status'. \
						Examples: 'event', 'event.message', 'status.start'",
						category
					);
					return TokenStream::from(Error::custom(err_msg).write_errors());
				}
				_ => {
					let err_msg = format!(
						"Invalid hook type format '{}'. Expected format: 'category' or 'category.subtype'. \
						Examples: 'event', 'event.message', 'status.start'",
						type_str
					);
					return TokenStream::from(Error::custom(err_msg).write_errors());
				}
			}
		}
		None => quote! { ::puniyu_adapter::private::HookType::default() },
	};
	let hook_rank = match &args.rank {
		Some(rank) => quote! { *#rank },
		_ => quote! { 500 },
	};

	let expanded = quote! {
		#item

		#[allow(non_camel_case_types)]
		struct #struct_name;

		#[::puniyu_adapter::private::async_trait]
		impl ::puniyu_adapter::private::HookBuilder for #struct_name {
			fn name(&self) -> &'static str {
				#hook_name
			}

			fn r#type(&self) -> ::puniyu_adapter::private::HookType {
				#hook_type
			}

			fn rank(&self) -> u64 {
				#hook_rank
			}

			async fn run(
				&self,
				event: Option<&Event>,
			) -> ::puniyu_adapter::private::HandlerResult {
				#fn_name(event).await
			}
		}

		::puniyu_adapter::private::inventory::submit! {
			crate::HookRegistry {
				adapter_name: #adapter_name,
				builder: || -> Box<dyn ::puniyu_adapter::private::HookBuilder> {
					Box::new(#struct_name {})
				}
			}
		}
	};
	TokenStream::from(expanded)
}
