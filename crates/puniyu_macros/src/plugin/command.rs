use super::arg::Arg;
use syn::{Token, braced, bracketed, parse::Parse, parse::ParseStream, punctuated::Punctuated};

pub struct CommandArgs {
	pub name: syn::LitStr,
	pub rank: syn::LitInt,
	pub desc: syn::LitStr,
	pub args: Vec<Arg>,
	pub alias: Vec<syn::LitStr>,
}

impl Default for CommandArgs {
	fn default() -> Self {
		Self {
			name: syn::LitStr::new("", proc_macro2::Span::call_site()),
			rank: syn::LitInt::new("500", proc_macro2::Span::call_site()),
			desc: syn::LitStr::new("", proc_macro2::Span::call_site()),
			args: Vec::new(),
			alias: Vec::new(),
		}
	}
}

impl Parse for CommandArgs {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		if input.is_empty() {
			return Err(syn::Error::new(input.span(), "呜~至少给人家一些参数嘛！杂鱼~"));
		}

		let mut cmd_args = CommandArgs::default();

		while !input.is_empty() {
			let key: syn::Ident = input.parse()?;
			input.parse::<Token![=]>()?;

			match key.to_string().as_str() {
				"name" => cmd_args.name = input.parse()?,
				"desc" => cmd_args.desc = input.parse()?,
				"rank" => cmd_args.rank = input.parse()?,
				"args" => {
					let content;
					bracketed!(content in input);
					cmd_args.args = parse_arg_list(&content)?;
				}
				"alias" => {
					let content;
					bracketed!(content in input);
					cmd_args.alias = parse_alias(&content)?;
				}
				_ => {
					return Err(syn::Error::new_spanned(
						&key,
						format!("呜哇~不支持的字段 '{}'！杂鱼~", key),
					));
				}
			}

			if input.peek(Token![,]) {
				input.parse::<Token![,]>()?;
			}
		}

		if cmd_args.name.value().is_empty() {
			return Err(syn::Error::new(input.span(), "呜哇~name都不给！杂鱼！"));
		}

		Ok(cmd_args)
	}
}

fn parse_arg_list(input: ParseStream) -> syn::Result<Vec<Arg>> {
	let mut args = Vec::new();
	let mut seen_names = std::collections::HashSet::new();

	while !input.is_empty() {
		let arg = parse_arg_item(input)?;
		let name = arg.name.value();

		if !seen_names.insert(name.clone()) {
			return Err(syn::Error::new_spanned(
				&arg.name,
				format!("呜哇~参数 '{}' 重复了！杂鱼~", name),
			));
		}

		args.push(arg);

		if input.peek(Token![,]) {
			input.parse::<Token![,]>()?;
		}
	}

	Ok(args)
}

fn parse_arg_item(input: ParseStream) -> syn::Result<Arg> {
	let mut arg = Arg::default();

	if input.peek(syn::LitStr) {
		arg.name = input.parse()?;
	} else if input.peek(syn::token::Paren) {
		let content;
		syn::parenthesized!(content in input);
		let elems = Punctuated::<syn::Expr, Token![,]>::parse_terminated(&content)?;
		arg.parse_from_tuple(&elems)?;
	} else if input.peek(syn::token::Brace) {
		let content;
		braced!(content in input);
		arg.parse_from_object(&content)?;
	} else {
		return Err(syn::Error::new(
			input.span(),
			"呜哇~参数格式错误！支持: \"name\", (元组), 或 { 对象 } 格式！杂鱼~",
		));
	}

	if arg.name.value().is_empty() {
		return Err(syn::Error::new(input.span(), "呜哇~参数必须指定 name！杂鱼~"));
	}

	Ok(arg)
}

fn parse_alias(input: ParseStream) -> syn::Result<Vec<syn::LitStr>> {
	let mut alias = Vec::new();

	while !input.is_empty() {
		let item: syn::LitStr = input.parse()?;
		alias.push(item);

		if input.peek(Token![,]) {
			input.parse::<Token![,]>()?;
		}
	}

	Ok(alias)
}
