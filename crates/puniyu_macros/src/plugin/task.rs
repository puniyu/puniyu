use syn::{Token, parse::Parse, parse::ParseStream, punctuated::Punctuated};

pub struct TaskArgs {
	pub name: syn::LitStr,
	pub cron: syn::LitStr,
}

impl Default for TaskArgs {
	fn default() -> Self {
		Self {
			cron: syn::LitStr::new("", proc_macro2::Span::call_site()),
			name: syn::LitStr::new("", proc_macro2::Span::call_site()),
		}
	}
}

impl TaskArgs {
	fn set_cron(args: &mut Self, value: syn::LitStr) -> syn::Result<()> {
		args.cron = value;
		Ok(())
	}

	fn set_name(args: &mut Self, value: syn::LitStr) -> syn::Result<()> {
		args.name = value;
		Ok(())
	}
}

impl Parse for TaskArgs {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		if input.is_empty() {
			return Err(syn::Error::new(input.span(), "呜~至少给人家一个cron表达式嘛！杂鱼~"));
		}

		let fields = Punctuated::<syn::MetaNameValue, Token![,]>::parse_terminated(input)?;
		let mut args = TaskArgs::default();

		for field in fields {
			let key = field
				.path
				.get_ident()
				.ok_or_else(|| syn::Error::new_spanned(&field.path, "呜哇~不支持的字段名！杂鱼~"))?
				.to_string();

			match key.as_str() {
				"cron" => {
					if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. }) =
						&field.value
					{
						TaskArgs::set_cron(&mut args, lit_str.clone())?;
					} else {
						return Err(syn::Error::new_spanned(
							&field.value,
							"呜哇~cron 必须是字符串！杂鱼~",
						));
					}
				}
				"name" => {
					if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. }) =
						&field.value
					{
						TaskArgs::set_name(&mut args, lit_str.clone())?;
					} else {
						return Err(syn::Error::new_spanned(
							&field.value,
							"呜哇~name 必须是字符串！杂鱼~",
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
