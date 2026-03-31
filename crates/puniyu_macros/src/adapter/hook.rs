use crate::{
	HookArgs,
	common::{validate_async, validate_return_type},
};
use zyn::{ToTokens, syn::spanned::Spanned, zyn};

pub fn hook(item: zyn::syn::ItemFn, cfg: HookArgs) -> zyn::TokenStream {
	let fn_sig = item.sig.clone();
	if let Err(err) = validate_async(&fn_sig) {
		return err.to_compile_error();
	}
	if let Err(err) = validate_return_type(&fn_sig, "puniyu_adapter::Result") {
		return err.to_compile_error();
	}

	let fn_name = &fn_sig.ident;
	let struct_name = zyn! {{ fn_name | fmt: "{}Hook" | pascal }};
	let adapter_name = zyn! { ::std::env!("CARGO_PKG_NAME") };
	let hook_name = match &cfg.name {
		Some(name) => zyn! { {{ name | str }} },
		_ => zyn! { {{ fn_name | lower | str }} },
	};
	let hook_type = match &cfg.hook_type {
		Some(type_str) => {
			let parts: Vec<&str> = type_str.split('.').collect();
			match parts.as_slice() {
				["event"] => zyn! {
					::puniyu_adapter::hook::HookType::Event(
						::puniyu_adapter::hook::HookEventType::default()
					)
				},
				["event", "message"] => zyn! {
					::puniyu_adapter::hook::HookType::Event(
						::puniyu_adapter::hook::HookEventType::Message
					)
				},
				["event", "notion"] => zyn! {
					::puniyu_adapter::hook::HookType::Event(
						::puniyu_adapter::hook::HookEventType::Notion
					)
				},
				["event", "request"] => zyn! {
					::puniyu_adapter::hook::HookType::Event(
						::puniyu_adapter::hook::HookEventType::Request
					)
				},
				["event", "all"] => zyn! {
					::puniyu_adapter::hook::HookType::Event(
						::puniyu_adapter::hook::HookEventType::All
					)
				},
				["status"] => zyn! {
					::puniyu_adapter::hook::HookType::Status(
						::puniyu_adapter::hook::StatusType::default()
					)
				},
				["status", "start"] => zyn! {
					::puniyu_adapter::hook::HookType::Status(
						::puniyu_adapter::hook::StatusType::Start
					)
				},
				["status", "stop"] => zyn! {
					::puniyu_adapter::hook::HookType::Status(
						::puniyu_adapter::hook::StatusType::Stop
					)
				},
				["event", subtype] => {
					let err_msg = format!(
						"Invalid event subtype '{}'. Valid event subtypes are: 'message', 'notion', 'request', 'all'. \
						Examples: 'event.message', 'event.notion'",
						subtype
					);
					return zyn::syn::Error::new(fn_sig.span(), err_msg).to_compile_error();
				}
				["status", subtype] => {
					let err_msg = format!(
						"Invalid status subtype '{}'. Valid status subtypes are: 'start', 'stop'. \
						Examples: 'status.start', 'status.stop'",
						subtype
					);
					return zyn::syn::Error::new(fn_sig.span(), err_msg).to_compile_error();
				}
				[category, _] => {
					let err_msg = format!(
						"Invalid hook category '{}'. Valid categories are: 'event', 'status'. \
						Examples: 'event.message', 'status.start'",
						category
					);
					return zyn::syn::Error::new(fn_sig.span(), err_msg).to_compile_error();
				}
				[category] => {
					let err_msg = format!(
						"Invalid hook category '{}'. Valid categories are: 'event', 'status'. \
						Examples: 'event', 'event.message', 'status.start'",
						category
					);
					return zyn::syn::Error::new(fn_sig.span(), err_msg).to_compile_error();
				}
				_ => {
					let err_msg = format!(
						"Invalid hook type format '{}'. Expected format: 'category' or 'category.subtype'. \
						Examples: 'event', 'event.message', 'status.start'",
						type_str
					);
					return zyn::syn::Error::new(fn_sig.span(), err_msg).to_compile_error();
				}
			}
		}
		None => zyn! { ::puniyu_adapter::hook::HookType::default() },
	};
	let hook_priority = match &cfg.priority {
		Some(priority) => zyn! { {{ priority }} },
		_ => zyn! { 500 },
	};

	zyn! {
		#item

		struct {{ struct_name }};

		#[::puniyu_adapter::__private::async_trait]
		impl ::puniyu_adapter::__private::Hook for {{ struct_name }} {
			fn name(&self) -> &'static str {
				#hook_name
			}

			fn r#type(&self) -> ::puniyu_adapter::hook::HookType {
				#hook_type
			}

			fn priority(&self) -> u32 {
				#hook_priority
			}

			#[inline]
			async fn run(
				&self,
				event: Option<&Event>,
			) -> ::puniyu_adapter::Result {
				#fn_name(event).await
			}
		}

		::puniyu_adapter::__private::inventory::submit! {
			crate::HookRegistry {
				adapter_name: {{ adapter_name }},
				builder: || -> ::std::sync::Arc<dyn ::puniyu_adapter::__private::Hook> {
					::std::sync::Arc::new(#struct_name {})
				}
			}
		}
	}
	.to_token_stream()
}
