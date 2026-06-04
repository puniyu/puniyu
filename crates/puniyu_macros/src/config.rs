use convert_case::{Case, Casing};
use quote::quote;
use syn::DeriveInput;

pub enum ConfigContext {
	Plugin,
	Adapter,
}

pub fn derive_config_impl(input: DeriveInput, context: ConfigContext) -> proc_macro2::TokenStream {
	let struct_name = &input.ident;
	let mut container_name_override: Option<String> = None;
	for attr in &input.attrs {
		if attr.path().is_ident("config")
			&& let Ok(syn::Meta::NameValue(nv)) = attr.parse_args::<syn::Meta>()
			&& nv.path.is_ident("name")
			&& let syn::Expr::Lit(lit) = &nv.value
			&& let syn::Lit::Str(s) = &lit.lit
		{
			container_name_override = Some(s.value());
		}
	}

	let config_name = container_name_override
		.map(|name| name.to_case(Case::Snake))
		.unwrap_or_else(|| struct_name.to_string().to_case(Case::Snake));
	let config_file_name = format!("{config_name}.toml");
	let serialize_error = format!("Failed to serialize {config_name} to TOML string");

	let (path_mod, toml_mod) = match context {
		ConfigContext::Plugin => {
			(quote!(::puniyu_plugin::path::plugin), quote!(::puniyu_plugin::toml))
		}
		ConfigContext::Adapter => {
			(quote!(::puniyu_adapter::path::adapter), quote!(::puniyu_adapter::toml))
		}
	};

	quote! {
		impl ::puniyu_core::config::Config for #struct_name {
			fn name(&self) -> &str {
				#config_name
			}

			fn path(&self) -> ::std::path::PathBuf {
				#path_mod::config_dir()
					.join(env!("CARGO_PKG_NAME"))
					.join(#config_file_name)
			}

			fn to_value(&self) -> #toml_mod::Value {
				#toml_mod::from_str(
					&#toml_mod::to_string(self).expect(#serialize_error),
				)
				.expect("Failed to parse TOML string to Value")
			}
		}
	}
}
