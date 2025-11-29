mod arg;
mod command;
mod task;

pub use command::CommandArgs;
pub use task::TaskArgs;

use syn::{Token, parse::Parse, parse::ParseStream, punctuated::Punctuated};

#[derive(Default)]
pub struct PluginArg {
	pub desc: Option<syn::LitStr>,
}

impl Parse for PluginArg {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		if input.is_empty() {
			return Ok(PluginArg::default());
		}

		let fields = Punctuated::<syn::MetaNameValue, Token![,]>::parse_terminated(input)?;
		let mut args = PluginArg::default();

		for field in fields {
			let key = field
				.path
				.get_ident()
				.ok_or_else(|| syn::Error::new_spanned(&field.path, "呜哇~不支持的字段名！杂鱼~"))?
				.to_string();

			match key.as_str() {
				"desc" => {
					if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. }) =
						&field.value
					{
						args.desc = Some(lit_str.clone());
					} else {
						return Err(syn::Error::new_spanned(
							&field.value,
							"呜哇~desc 必须是字符串！杂鱼~",
						));
					}
				}
				_ => {
					return Err(syn::Error::new_spanned(
						&field.path,
						format!("呜哇~不支持的字段 '{}'！杂鱼~", key),
					));
				}
			}
		}

		Ok(args)
	}
}
