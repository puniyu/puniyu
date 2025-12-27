use quote::quote;
use syn::{Token, ext::IdentExt, parse::ParseStream, punctuated::Punctuated};

pub const VALID_ARG_TYPES: [&str; 4] = ["string", "int", "float", "bool"];
pub const VALID_ARG_MODES: [&str; 2] = ["positional", "named"];
pub const SUPPORTED_FIELDS: &str = "name, arg_type, mode, required, default, desc";

#[derive(Clone)]
pub struct Arg {
	pub name: syn::LitStr,
	pub arg_type: syn::LitStr,
	pub mode: syn::LitStr,
	pub required: bool,
	pub default: Option<syn::Expr>,
	pub desc: syn::LitStr,
}

impl Default for Arg {
	fn default() -> Self {
		Self {
			name: syn::LitStr::new("", proc_macro2::Span::call_site()),
			arg_type: syn::LitStr::new("string", proc_macro2::Span::call_site()),
			mode: syn::LitStr::new("positional", proc_macro2::Span::call_site()),
			required: true,
			default: None,
			desc: syn::LitStr::new("", proc_macro2::Span::call_site()),
		}
	}
}

impl Arg {
	pub fn parse_from_tuple(
		&mut self,
		elems: &Punctuated<syn::Expr, Token![,]>,
	) -> syn::Result<()> {
		let mut iter = elems.iter();

		if let Some(syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. })) = iter.next()
		{
			self.name = lit_str.clone();
		} else {
			return Err(syn::Error::new_spanned(
				elems,
				"呜哇~第一个元素必须是参数名(字符串)！杂鱼~",
			));
		}

		if let Some(syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. })) = iter.next()
		{
			if !VALID_ARG_TYPES.contains(&lit_str.value().as_str()) {
				return Err(syn::Error::new_spanned(
					lit_str,
					format!("呜哇~类型必须是 {:?} 之一！杂鱼~", VALID_ARG_TYPES),
				));
			}
			self.arg_type = lit_str.clone();
		}

		if let Some(syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Bool(lit_bool), .. })) =
			iter.next()
		{
			self.required = lit_bool.value;
		}

		if let Some(expr) = iter.next() {
			self.default = Some(expr.clone());
		}

		if let Some(syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. })) = iter.next()
		{
			self.desc = lit_str.clone();
		}

		if let Some(syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. })) = iter.next()
		{
			if !VALID_ARG_MODES.contains(&lit_str.value().as_str()) {
				return Err(syn::Error::new_spanned(
					lit_str,
					format!("呜哇~mode 必须是 {:?} 之一！杂鱼~", VALID_ARG_MODES),
				));
			}
			self.mode = lit_str.clone();
		}

		Ok(())
	}

	pub fn parse_from_object(&mut self, input: ParseStream) -> syn::Result<()> {
		while !input.is_empty() {
			let key = input.call(syn::Ident::parse_any)?;
			input.parse::<Token![=]>()?;

			match key.to_string().as_str() {
				"name" => self.name = input.parse()?,
				"arg_type" => {
					let lit_str: syn::LitStr = input.parse()?;
					if !VALID_ARG_TYPES.contains(&lit_str.value().as_str()) {
						return Err(syn::Error::new_spanned(
							&lit_str,
							format!("呜哇~arg_type 必须是 {:?} 之一！杂鱼~", VALID_ARG_TYPES),
						));
					}
					self.arg_type = lit_str;
				}
				"mode" => {
					let lit_str: syn::LitStr = input.parse()?;
					if !VALID_ARG_MODES.contains(&lit_str.value().as_str()) {
						return Err(syn::Error::new_spanned(
							&lit_str,
							format!("呜哇~mode 必须是 {:?} 之一！杂鱼~", VALID_ARG_MODES),
						));
					}
					self.mode = lit_str;
				}
				"required" => {
					let lit_bool: syn::LitBool = input.parse()?;
					self.required = lit_bool.value;
				}
				"default" => self.default = Some(input.parse()?),
				"desc" => self.desc = input.parse()?,
				_ => {
					return Err(syn::Error::new_spanned(
						&key,
						format!("呜哇~不支持的字段 '{}'！支持: {} 杂鱼~", key, SUPPORTED_FIELDS),
					));
				}
			}

			if input.peek(Token![,]) {
				input.parse::<Token![,]>()?;
			}
		}
		Ok(())
	}

	pub fn to_tokens(&self) -> proc_macro2::TokenStream {
		let name = &self.name;
		let arg_type_str = self.arg_type.value();
		let arg_type = match arg_type_str.as_str() {
			"int" => quote! { ::puniyu_plugin::ArgType::Int },
			"float" => quote! { ::puniyu_plugin::ArgType::Float },
			"bool" => quote! { ::puniyu_plugin::ArgType::Bool },
			_ => quote! { ::puniyu_plugin::ArgType::String },
		};
		let mode_str = self.mode.value();
		let mode = match mode_str.as_str() {
			"named" => quote! { ::puniyu_plugin::ArgMode::Named },
			_ => quote! { ::puniyu_plugin::ArgMode::Positional },
		};
		let required = self.required;
		let desc = &self.desc;
		let default_value = match &self.default {
			Some(expr) => match arg_type_str.as_str() {
				"int" => quote! { Some(::puniyu_plugin::ArgValue::Int(#expr as i64)) },
				"float" => quote! { Some(::puniyu_plugin::ArgValue::Float(#expr as f64)) },
				"bool" => quote! { Some(::puniyu_plugin::ArgValue::Bool(#expr)) },
				_ => quote! { Some(::puniyu_plugin::ArgValue::String(#expr.to_string())) },
			},
			None => quote! { None },
		};
		quote! {
			::puniyu_plugin::Arg {
				name: #name,
				arg_type: #arg_type,
				mode: #mode,
				required: #required,
				default: #default_value,
				description: if #desc.is_empty() { None } else { Some(#desc) },
			}
		}
	}
}
