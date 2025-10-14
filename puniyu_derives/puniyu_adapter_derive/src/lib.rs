use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn adapter(_: TokenStream, item: TokenStream) -> TokenStream {
	let input_struct = if let Ok(struct_item) = syn::parse::<syn::ItemStruct>(item.clone()) {
		struct_item
	} else {
		return syn::Error::new_spanned(
			proc_macro2::TokenStream::from(item),
			"这个宏只能用在结构体上！",
		)
		.to_compile_error()
		.into();
	};

	let struct_name = &input_struct.ident;

	let expanded = quote! {
		#input_struct

		use std::sync::{OnceLock, Mutex, Arc};
		use puniyu_core::APP_NAME;

		#[unsafe(no_mangle)]
		pub extern "C" fn get_adapter_info() -> *mut dyn ::puniyu_core::adapter::AdapterBuilder {
			Box::into_raw(Box::new(#struct_name {}))
		}

		#[unsafe(no_mangle)]
		pub extern "C" fn setup_app_name(name: &str) {
			 APP_NAME.get_or_init(|| name.to_string());
		}

		#[unsafe(no_mangle)]
		pub extern "C" fn setup_event_bus(bus: Arc<Mutex<::puniyu_core::adapter::EventBus>>) {
			 EVENT_BUS.get_or_init(|| bus);
		}
	};

	TokenStream::from(expanded)
}
