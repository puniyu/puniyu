use proc_macro::TokenStream;
use quote::quote;
use std::env;

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

	let adapter_name = match env::var("ADAPTER_NAME") {
		Ok(name) => name,
		Err(_) => {
			return syn::Error::new_spanned(
				&input_struct,
				"呜哇~ADAPTER_NAME都没有设置！杂鱼程序员！",
			)
			.to_compile_error()
			.into();
		}
	};
	let _ = match env::var("ADAPTER_VERSION") {
		Ok(version) => version,
		Err(_) => {
			return syn::Error::new_spanned(
				&input_struct,
				"呜哇~ADAPTER_VERSION都没有设置！杂鱼程序员！",
			)
			.to_compile_error()
			.into();
		}
	};
	let _ = match env::var("ADAPTER_AUTHOR") {
		Ok(author) => quote! { #author },
		Err(_) => {
			return syn::Error::new_spanned(
				&input_struct,
				"呜哇~ADAPTER_AUTHOR都没有设置！杂鱼程序员！",
			)
			.to_compile_error()
			.into();
		}
	};

	let struct_name = &input_struct.ident;

	let expanded = quote! {
		#input_struct

		use std::sync::{OnceLock, Mutex, Arc};

		#[cfg(feature = "cdylib")]
		#[unsafe(no_mangle)]
		pub extern "C" fn get_adapter_info() -> *mut dyn ::puniyu_adapter::AdapterBuilder {
			Box::into_raw(Box::new(#struct_name {}))
		}

		#[cfg(feature = "cdylib")]
		#[unsafe(no_mangle)]
		pub extern "C" fn setup_app_name(name: String) {
			 puniyu_adapter::APP_NAME.get_or_init(|| name);
		}

		#[cfg(feature = "cdylib")]
		#[unsafe(no_mangle)]
		pub extern "C" fn setup_event_bus(bus: Arc<Mutex<::puniyu_core::adapter::EventBus>>) {
			puniyu_adapter::setup_event_bus(bus);
		}

		#[macro_export]
		macro_rules! info {
				($($arg:tt)*) => {
					{
						use ::puniyu_adapter::logger::OwoColorize;
						let prefix = "adapter".fg_rgb::<176,196,222>();
						let func_name = #adapter_name.fg_rgb::<255,192,203>();
						::puniyu_adapter::logger::info!("[{}:{}] {}", prefix,func_name, format!($($arg)*))
					}
				};
		}

			#[macro_export]
			macro_rules! warn {
				($($arg:tt)*) => {
					{
						use ::puniyu_adapter::logger::OwoColorize;
						let prefix = "adapter".fg_rgb::<176,196,222>();
						let func_name = #adapter_name.fg_rgb::<255,192,203>();
						::puniyu_adapter::logger::warn!("[{}:{}] {}", prefix,func_name, format!($($arg)*))
					}
				};
			}

			#[macro_export]
			macro_rules! error {
				($($arg:tt)*) => {
				{
						use ::puniyu_adapter::logger::OwoColorize;
						let prefix = "adapter".fg_rgb::<176,196,222>();
						let func_name = #adapter_name.fg_rgb::<255,192,203>();
						::puniyu_adapter::logger::error!("[{}:{}] {}", prefix,func_name, format_args!($($arg)*))
					}
				};
			}

			#[macro_export]
			macro_rules! debug {
				($($arg:tt)*) => {
					{
						use ::puniyu_adapter::logger::OwoColorize;
						let prefix = "adapter".fg_rgb::<176,196,222>();
						let func_name = #adapter_name.fg_rgb::<255,192,203>();
						::puniyu_adapter::logger::debug!("[{}:{}] {}", prefix,func_name, format_args!($($arg)*))
					}
				};
			}
	};

	TokenStream::from(expanded)
}
