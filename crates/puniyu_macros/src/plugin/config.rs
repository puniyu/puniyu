use proc_macro::TokenStream;
use darling::ast::NestedMeta;
use darling::{Error, FromMeta};
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[derive(Debug, FromMeta)]
struct ConfigArgs {
	#[darling(default)]
	name: Option<String>,
}

pub fn config(args: TokenStream, item: TokenStream) -> TokenStream {
	let attr_args = match NestedMeta::parse_meta_list(args.into()) {
		Ok(v) => v,
		Err(e) => return TokenStream::from(Error::from(e).write_errors()),
	};
	let item = parse_macro_input!(item as ItemStruct);
    let struct_name = &item.ident;

	let args = match ConfigArgs::from_list(&attr_args) {
		Ok(v) => v,
		Err(e) => return TokenStream::from(e.write_errors()),
	};

	let config_name = match args.name {
		Some(name) => quote! { #name },
		None => {
			use convert_case::{Case, Casing};
			let name = struct_name.to_string().to_case(Case::Lower);
			quote! { #name }
		}
	};

    let plugin_name = quote! { env!("CARGO_PKG_NAME") };

    let expanded = quote! {
		#item

		impl ::puniyu_plugin::private::Config for #struct_name {
			fn name(&self) -> &'static str {
				#config_name
			}

			fn config(&self) -> ::puniyu_plugin::private::toml::Value {
				let config_str = ::puniyu_plugin::private::toml::to_string(&Self::default())
					.expect("Failed to serialize config");
				::puniyu_plugin::private::toml::from_str(&config_str)
					.expect("Failed to deserialize config")
			}
		}

		impl #struct_name {
			pub fn get() -> Self {
				use ::puniyu_plugin::private::PluginBuilder;
				let plugin_name = crate::Plugin {}.name().to_lowercase();
				let path = ::puniyu_plugin::private::PLUGIN_CONFIG_DIR.join(plugin_name).join(format!("{}.toml", #config_name));
				::puniyu_plugin::private::ConfigRegistry::get(&path)
					.and_then(|cfg| cfg.try_into::<#struct_name>().ok())
					.unwrap_or_default()
			}
		}

		::puniyu_plugin::private::inventory::submit! {
			crate::ConfigRegistry {
				plugin_name: #plugin_name,
				builder: || -> Box<dyn ::puniyu_plugin::private::Config> {
					Box::new(#struct_name::default())
				}
			}
		}
	};

    expanded.into()
}
