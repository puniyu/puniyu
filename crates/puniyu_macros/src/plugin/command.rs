use super::arg::Arg;
use syn::{Token, braced, bracketed, parse::Parse, parse::ParseStream, punctuated::Punctuated};

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

		let mut cmd_args = CommandArgs::default();

		while !input.is_empty() {
			let key: syn::Ident = input.parse()?;
			input.parse::<Token![=]>()?;

			match key.to_string().as_str() {
				"name" => {
					cmd_args.name = input.parse()?;
				}
				"desc" => {
					cmd_args.desc = input.parse()?;
				}
				"rank" => {
					cmd_args.rank = input.parse()?;
				}
				"args" => {
					let content;
					bracketed!(content in input);
					cmd_args.args = parse_args_array(&content)?;
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

fn parse_args_array(input: ParseStream) -> syn::Result<Vec<Arg>> {
	let mut args = Vec::new();

	while !input.is_empty() {
		let arg = parse_single_arg(input)?;
		args.push(arg);

		if input.peek(Token![,]) {
			input.parse::<Token![,]>()?;
		}
	}

	Ok(args)
}

fn parse_single_arg(input: ParseStream) -> syn::Result<Arg> {
	let mut arg = Arg::default();

	if input.peek(syn::LitStr) {
		arg.name = input.parse()?;
	} else if input.peek(syn::token::Paren) {
		let content;
		syn::parenthesized!(content in input);
		let elems = Punctuated::<syn::Expr, Token![,]>::parse_terminated(&content)?;
		arg.parse_tuple(&elems)?;
	} else if input.peek(syn::token::Brace) {
		let content;
		braced!(content in input);
		parse_arg_object(&content, &mut arg)?;
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

fn parse_arg_object(input: ParseStream, arg: &mut Arg) -> syn::Result<()> {
	while !input.is_empty() {
		let key_str = if input.peek(Token![type]) {
			input.parse::<Token![type]>()?;
			"type".to_string()
		} else {
			let key: syn::Ident = input.parse()?;
			key.to_string()
		};
		input.parse::<Token![=]>()?;

		match key_str.as_str() {
			"name" => {
				arg.name = input.parse()?;
			}
			"r#type" | "type" => {
				let lit_str: syn::LitStr = input.parse()?;
				let valid_types = ["string", "int", "float", "bool"];
				if !valid_types.contains(&lit_str.value().as_str()) {
					return Err(syn::Error::new_spanned(
						&lit_str,
						format!("呜哇~type 必须是 {:?} 之一！杂鱼~", valid_types),
					));
				}
				arg.arg_type = lit_str;
			}
			"mode" => {
				let lit_str: syn::LitStr = input.parse()?;
				let valid_modes = ["positional", "named"];
				if !valid_modes.contains(&lit_str.value().as_str()) {
					return Err(syn::Error::new_spanned(
						&lit_str,
						format!("呜哇~mode 必须是 {:?} 之一！杂鱼~", valid_modes),
					));
				}
				arg.mode = lit_str;
			}
			"required" => {
				let lit_bool: syn::LitBool = input.parse()?;
				arg.required = lit_bool.value;
			}
			"default" => {
				arg.default = Some(input.parse()?);
			}
			"desc" => {
				arg.desc = input.parse()?;
			}
			_ => {
				return Err(syn::Error::new(
					input.span(),
					format!("呜哇~不支持的字段 '{}'！杂鱼~", key_str),
				));
			}
		}

		if input.peek(Token![,]) {
			input.parse::<Token![,]>()?;
		}
	}

	Ok(())
}
