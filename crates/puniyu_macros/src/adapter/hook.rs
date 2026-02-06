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
	rank: Option<u32>,
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

	let fn_sig = &item.sig;
	let fn_name = &fn_sig.ident;
	let fn_inputs = &fn_sig.inputs;

	if item.sig.asyncness.is_none() {
		return syn::Error::new_spanned(&item.sig, "function must be async")
			.to_compile_error()
			.into();
	}

	if fn_inputs.len() != 1 {
		return syn::Error::new_spanned(
			fn_sig,
			format!(
				"function `{}` must have exactly 1 parameters, found {}",
				fn_name,
				fn_inputs.len()
			),
		)
		.to_compile_error()
		.into();
	}

	let first_param = fn_inputs.first().unwrap();
	if let syn::FnArg::Typed(pat_type) = first_param {
		let ty = &*pat_type.ty;
		let is_valid_type = match ty {
			syn::Type::Path(type_path) => {
				if let Some(last_seg) = type_path.path.segments.last() {
					if last_seg.ident == "Option" {
						if let syn::PathArguments::AngleBracketed(args) = &last_seg.arguments {
							if let Some(syn::GenericArgument::Type(syn::Type::Reference(
								type_ref,
							))) = args.args.first()
							{
								if let syn::Type::Path(inner_path) = &*type_ref.elem {
									inner_path
										.path
										.segments
										.last()
										.map(|seg| seg.ident == "Event")
										.unwrap_or(false)
								} else {
									false
								}
							} else {
								false
							}
						} else {
							false
						}
					} else {
						false
					}
				} else {
					false
				}
			}
			_ => false,
		};

		if !is_valid_type {
			return syn::Error::new_spanned(
				pat_type,
				format!(
					"function `{}` parameter must be of type `Option<&Event>`, found `{}`",
					fn_name,
					quote::quote! { #ty }
				),
			)
			.to_compile_error()
			.into();
		}
	} else {
		return syn::Error::new_spanned(
			first_param,
			format!("function `{}` parameter must be a typed parameter", fn_name),
		)
		.to_compile_error()
		.into();
	}

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
		impl ::puniyu_adapter::private::Hook for #struct_name {
			fn name(&self) -> &'static str {
				#hook_name
			}

			fn r#type(&self) -> ::puniyu_adapter::private::HookType {
				#hook_type
			}

			fn rank(&self) -> u32 {
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
				builder: || -> Box<dyn ::puniyu_adapter::private::Hook> {
					Box::new(#struct_name {})
				}
			}
		}
	};
	TokenStream::from(expanded)
}
