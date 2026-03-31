use crate::ConfigArgs;
use zyn::{ToTokens, zyn};

pub fn config(item: zyn::syn::ItemStruct, cfg: ConfigArgs) -> zyn::TokenStream {
	let struct_name = &item.ident;
	let config_name = match &cfg.name {
		Some(name) => zyn! { {{ name | snake | str }}},
		None => zyn! { {{ struct_name | snake | str }}},
	};
	let config_file_name = zyn! { {{ config_name | snake | fmt: "{}.toml" }}}.to_string();
	let serialize_error = match &cfg.name {
		Some(name) => zyn! { {{ name | snake | fmt: "Failed to serialize {} to TOML string" }}},
		None => zyn! { {{ struct_name | snake | fmt: "Failed to serialize {} to TOML string" }}},
	};

	let plugin_name = zyn! { env!("CARGO_PKG_NAME") };

	zyn! {
		{{ item }}

impl ::puniyu_plugin::__private::Config for {{ struct_name }} {
			fn config(&self) -> ::puniyu_plugin::__private::ConfigInfo {
				::puniyu_plugin::__private::ConfigInfo {
					name: ::std::string::String::from({{ config_name }}),
					path: ::puniyu_plugin::path::adapter::config_dir()
						.join({{ plugin_name }})
						.join({{ config_file_name }}),
					value: ::puniyu_plugin::__private::toml::from_str(
						&::puniyu_plugin::__private::toml::to_string(self).expect({{ serialize_error }}),
					).expect("Failed to parse TOML string to Value"),
				}
			}
		}

		::puniyu_plugin::__private::inventory::submit! {
			crate::ConfigRegistry {
				plugin_name: {{ plugin_name }},
				builder: || -> ::std::sync::Arc<dyn ::puniyu_plugin::__private::Config> {
					::std::sync::Arc::new( {{ struct_name}}::default())
				}
			}
		}
	}
	.into_token_stream()
}
