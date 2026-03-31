use crate::ConfigArgs;
use zyn::{ToTokens, zyn};

pub fn config(item: zyn::syn::ItemStruct, cfg: ConfigArgs) -> zyn::TokenStream {
	let struct_name = &item.ident;
	let config_name = match &cfg.name {
		Some(name) => zyn! { {{ name | snake | str }}},
		None => zyn! { {{ struct_name | snake | str }}},
	};
	let config_file_name = match &cfg.name {
		Some(name) => zyn! { {{ name | snake | fmt: "{}.toml" }}},
		None => zyn! { {{ struct_name | snake | fmt: "{}.toml" }}},
	};
	let serialize_error = match &cfg.name {
		Some(name) => zyn! { {{ name | snake | fmt: "Failed to serialize {} to TOML string" }}},
		None => zyn! { {{ struct_name | snake | fmt: "Failed to serialize {} to TOML string" }}},
	};

	let adapter_name = zyn! { ::std::env!("CARGO_PKG_NAME") };

	zyn! {
		{{ item }}

		const _: () = {
			fn assert_config_traits<T>()
			where
				T: ::std::default::Default,
			{
			}

			let _ = assert_config_traits::<{{ struct_name }}>;
		};

		impl ::puniyu_adapter::__private::Config for {{ struct_name }} {
			fn config(&self) -> ::puniyu_adapter::__private::ConfigInfo {
				::puniyu_adapter::__private::ConfigInfo {
					name: ::std::string::String::from({{ config_name }}),
					path: ::puniyu_adapter::path::adapter::config_dir()
						.join({{ adapter_name }})
						.join({{ config_file_name }}),
					value: ::puniyu_adapter::__private::toml::from_str(
						&::puniyu_adapter::__private::toml::to_string(self).expect({{ serialize_error }}),
					).expect("Failed to parse TOML string to Value"),
				}
			}
		}

		::puniyu_adapter::__private::inventory::submit! {
			crate::ConfigRegistry {
				adapter_name: {{ adapter_name }},
				builder: || -> ::std::sync::Arc<dyn ::puniyu_adapter::__private::Config> {
					::std::sync::Arc::new( {{ struct_name}}::default())
				}
			}
		}
	}
	.into_token_stream()
}
