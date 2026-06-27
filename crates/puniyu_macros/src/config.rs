use convert_case::Casing;
use quote::quote;
use syn::DeriveInput;

pub enum ConfigContext {
	Plugin,
	Adapter,
}

pub fn config_impl(
	args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
	context: ConfigContext,
) -> proc_macro::TokenStream {
	let input = match syn::parse::<DeriveInput>(item) {
		Ok(input) => input,
		Err(err) => return err.to_compile_error().into(),
	};

	let name_override = if args.is_empty() {
		None
	} else if let Ok(lit) = syn::parse::<syn::LitStr>(args.clone()) {
		Some(lit.value())
	} else if let Ok(nv) = syn::parse::<syn::MetaNameValue>(args) {
		if nv.path.is_ident("name") {
			if let syn::Expr::Lit(lit) = &nv.value {
				if let syn::Lit::Str(s) = &lit.lit {
					Some(s.value())
				} else {
					return syn::Error::new_spanned(&nv.value, "expected string literal")
						.to_compile_error()
						.into();
				}
			} else {
				return syn::Error::new_spanned(&nv.value, "expected string literal")
					.to_compile_error()
					.into();
			}
		} else {
			return syn::Error::new_spanned(&nv.path, "expected `name`").to_compile_error().into();
		}
	} else {
		return syn::Error::new(
			proc_macro2::Span::call_site(),
			"expected #[config], #[config(\"name\")] or #[config(name = \"name\")]",
		)
		.to_compile_error()
		.into();
	};

	let name =
		name_override.unwrap_or_else(|| input.ident.to_string().to_case(convert_case::Case::Snake));
	config_struct(input, context, name).into()
}

fn config_struct(
	input: DeriveInput,
	context: ConfigContext,
	name: String,
) -> proc_macro2::TokenStream {
	let struct_name = &input.ident;
	let config_name = name;
	let config_file_name = format!("{config_name}.toml");
	let serialize_error = format!("Failed to serialize {config_name} to TOML string");

	let (crate_path, path_sub) = match context {
		ConfigContext::Plugin => (quote!(::puniyu_plugin), quote!(plugin)),
		ConfigContext::Adapter => (quote!(::puniyu_adapter), quote!(adapter)),
	};

	quote! {
		#input

		impl #crate_path::config::Config for #struct_name {
			fn name(&self) -> &str {
				#config_name
			}

			fn path(&self) -> ::std::path::PathBuf {
				#crate_path::path::#path_sub::config_dir()
					.join(env!("CARGO_PKG_NAME").replace('-', "_").to_lowercase())
					.join(#config_file_name)
			}

			fn to_value(&self) -> #crate_path::toml::Value {
				#crate_path::toml::from_str(
					&#crate_path::toml::to_string(self).expect(#serialize_error),
				)
				.expect("Failed to parse TOML string to Value")
			}
		}

		impl #struct_name {
			pub fn get() -> Self {
				let path = #crate_path::path::#path_sub::config_dir()
					.join(env!("CARGO_PKG_NAME").replace('-', "_").to_lowercase())
					.join(#config_file_name);
				#crate_path::config::ConfigRegistry::get_with_path(&path)
					.and_then(|v| v.try_into().ok())
					.unwrap_or_default()
			}
		}

		crate::__puniyu_submit!(config, || -> ::std::vec::Vec<::std::sync::Arc<dyn #crate_path::config::Config>> {
			::std::vec![::std::sync::Arc::new(#struct_name::default())]
		});
	}
}
