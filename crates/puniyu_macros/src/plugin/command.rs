use convert_case::{Case, Casing};
use darling::ast::NestedMeta;
use darling::util::PathList;
use darling::{Error, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::Ident;
use syn::{ItemFn, parse_macro_input};

#[derive(Debug, FromMeta, Default)]
struct Arg {
	pub name: String,
	#[darling(rename = "type")]
	pub r#type: Option<String>,
	pub mode: Option<String>,
	pub required: Option<bool>,
	pub desc: Option<String>,
}

#[derive(Debug, FromMeta, Default)]
struct CommandArgs {
	pub name: String,
	pub rank: Option<u32>,
	pub desc: Option<String>,
	pub alias: Option<PathList>,
	pub permission: Option<String>,
}
pub fn command(args: TokenStream, item: TokenStream) -> TokenStream {
	let attr_args = match NestedMeta::parse_meta_list(args.into()) {
		Ok(v) => v,
		Err(e) => return TokenStream::from(Error::from(e).write_errors()),
	};
	let item = parse_macro_input!(item as ItemFn);
	let fn_name = &item.sig.ident;

	let struct_name_str = {
		let fn_name_str = fn_name.to_string();
		let pascal_case_name = fn_name_str.to_case(Case::Pascal);
		format!("{}Command", pascal_case_name)
	};

	let args = match CommandArgs::from_list(&attr_args) {
		Ok(v) => v,
		Err(e) => return TokenStream::from(e.write_errors()),
	};

	let mut command_arg_list = Vec::<Arg>::new();

	for attr in &item.attrs {
		if attr.path().is_ident("arg") {
			let meta_list: Vec<NestedMeta> = match attr.parse_args_with(
				syn::punctuated::Punctuated::<NestedMeta, syn::Token![,]>::parse_terminated,
			) {
				Ok(punctuated) => punctuated.into_iter().collect(),
				Err(e) => return TokenStream::from(Error::from(e).write_errors()),
			};

			match Arg::from_list(&meta_list) {
				Ok(arg) => command_arg_list.push(arg),
				Err(e) => return TokenStream::from(e.write_errors()),
			}
		}
	}

	let command_name = &args.name;
	let command_rank = match args.rank {
		Some(rank) => quote! { #rank },
		None => quote! { 500 },
	};
	let command_desc = match &args.desc {
		Some(desc) => quote! { Some(#desc) },
		None => quote! { None },
	};
	let command_permission = match &args.permission {
		Some(perm) => quote! { #perm.parse().unwrap_or(::puniyu_plugin::private::Permission::All) },
		None => quote! { ::puniyu_plugin::private::Permission::All },
	};
	let command_alias = match &args.alias {
		Some(aliases) if !aliases.is_empty() => {
			quote! { vec![#(#aliases),*] }
		}
		_ => quote! { Vec::new() },
	};
	let plugin_name = quote! { env!("CARGO_PKG_NAME") };
	let struct_name = Ident::new(&struct_name_str, fn_name.span());

	let mut args_construction = Vec::new();

	for arg in &command_arg_list {
		let arg_name = &arg.name;
		let arg_type = arg.r#type.as_deref().unwrap_or("string");
		let arg_mode = arg.mode.as_deref().unwrap_or("positional");
		let arg_required = arg.required.unwrap_or(false);

		let constructor = match arg_type {
			"string" => quote! { ::puniyu_plugin::private::Arg::string(#arg_name) },
			"int" => quote! { ::puniyu_plugin::private::Arg::int(#arg_name) },
			"float" => quote! { ::puniyu_plugin::private::Arg::float(#arg_name) },
			"bool" => quote! { ::puniyu_plugin::private::Arg::bool(#arg_name) },
			_ => {
				return TokenStream::from(
					Error::custom(format!(
						"Unsupported argument type '{}'. Supported types: string, int, float, bool",
						arg_type
					))
					.write_errors(),
				);
			}
		};

		let mode_method = match arg_mode {
			"named" => quote! { .named() },
			"positional" => quote! { .positional() },
			_ => {
				return TokenStream::from(
					Error::custom(format!(
						"Invalid arg mode '{}'. Must be 'positional' or 'named'",
						arg_mode
					))
					.write_errors(),
				);
			}
		};

		let required_method = if arg_required {
			quote! { .required() }
		} else {
			quote! { .optional() }
		};

		let desc_method = arg.desc.as_ref().map(|desc| quote! { .description(#desc) });

		args_construction.push(quote! { #constructor #mode_method #required_method #desc_method });
	}

	let expanded = quote! {
		#item

		#[allow(non_camel_case_types)]
		struct #struct_name;

		#[::puniyu_plugin::private::async_trait]
		impl ::puniyu_plugin::private::CommandBuilder for #struct_name {
			fn name(&self) -> &'static str {
				#command_name
			}

			fn description(&self) -> Option<&'static str> {
				#command_desc
			}

			fn rank(&self) -> u32 {
				#command_rank
			}

			fn args(&'_ self) -> Vec<::puniyu_plugin::private::Arg<'static>> {
				vec![
					#(#args_construction,)*
				]
			}

			fn alias(&self) -> Vec<&'static str> {
				#command_alias
			}

			fn permission(&self) -> ::puniyu_plugin::private::Permission {
				#command_permission
			}

			async fn run(&self, bot: &::puniyu_plugin::private::BotContext, ev: &::puniyu_plugin::private::MessageContext) -> ::puniyu_plugin::private::HandlerResult<::puniyu_plugin::private::HandlerAction> {
				#fn_name(bot, ev).await
			}
		}

		::puniyu_plugin::private::inventory::submit! {
			crate::CommandRegistry {
				plugin_name: #plugin_name,
				builder: || -> Box<dyn ::puniyu_plugin::private::CommandBuilder> { Box::new(#struct_name {}) },
			}
		}
	};

	TokenStream::from(expanded)
}
