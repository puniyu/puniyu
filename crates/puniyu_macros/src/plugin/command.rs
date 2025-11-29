use syn::{Token, parse::Parse, parse::ParseStream, punctuated::Punctuated};
use super::arg::Arg;

pub struct CommandArgs {
	pub name: syn::LitStr,
	pub rank: syn::LitInt,
	pub desc: syn::LitStr,
	pub args: Vec<Arg>,
}

impl Default for CommandArgs {
	fn default() -> Self {
		Self {
			name: syn::LitStr::new("", proc_macro2::Span::call_site()),
			rank: syn::LitInt::new("100", proc_macro2::Span::call_site()),
			desc: syn::LitStr::new("", proc_macro2::Span::call_site()),
			args: Vec::new(),
		}
	}
}

impl Parse for CommandArgs {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		if input.is_empty() {
			return Err(syn::Error::new(input.span(), "呜~至少给人家一些参数嘛！杂鱼~"));
		}

		let fields = Punctuated::<syn::MetaNameValue, Token![,]>::parse_terminated(input)?;
		let mut cmd_args = CommandArgs::default();
		for field in fields {
			let key = field
				.path
				.get_ident()
				.ok_or_else(|| syn::Error::new_spanned(&field.path, "呜哇~不支持的字段名！杂鱼~"))?
				.to_string();

			match key.as_str() {
				"name" => {
					if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. }) =
						&field.value
					{
						cmd_args.name = lit_str.clone();
					} else {
						return Err(syn::Error::new_spanned(
							&field.value,
							"呜哇~name 必须是字符串！杂鱼~",
						));
					}
				}
				"desc" => {
					if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. }) =
						&field.value
					{
						cmd_args.desc = lit_str.clone();
					} else {
						return Err(syn::Error::new_spanned(
							&field.value,
							"呜哇~desc 必须是字符串！杂鱼~",
						));
					}
				}
				"rank" => {
					if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(lit_int), .. }) =
						&field.value
					{
						cmd_args.rank = lit_int.clone();
					} else {
						return Err(syn::Error::new_spanned(
							&field.value,
							"呜哇~rank 必须是整数！杂鱼~",
						));
					}
				}
				"args" => {
					if let syn::Expr::Array(arr) = &field.value {
						for elem in &arr.elems {
							let arg = Arg::from_expr(elem)?;
							cmd_args.args.push(arg);
						}
					} else {
						return Err(syn::Error::new_spanned(
							&field.value,
							"呜哇~args 必须是数组格式！杂鱼~",
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

		if cmd_args.name.value().is_empty() {
			return Err(syn::Error::new(input.span(), "呜哇~name都不给！杂鱼！"));
		}

		Ok(cmd_args)
	}
}
