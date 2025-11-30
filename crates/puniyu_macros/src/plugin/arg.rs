use quote::quote;
use syn::{Token, parse::Parse, parse::ParseStream, punctuated::Punctuated};

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
	pub fn parse_tuple(&mut self, elems: &Punctuated<syn::Expr, Token![,]>) -> syn::Result<()> {
		let mut iter = elems.iter();
		
		if let Some(syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. })) = iter.next() {
			self.name = lit_str.clone();
		} else {
			return Err(syn::Error::new_spanned(elems, "呜哇~第一个元素必须是参数名(字符串)！杂鱼~"));
		}
		
		if let Some(syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. })) = iter.next() {
			let valid_types = ["string", "int", "float", "bool"];
			if !valid_types.contains(&lit_str.value().as_str()) {
				return Err(syn::Error::new_spanned(lit_str, format!("呜哇~类型必须是 {:?} 之一！杂鱼~", valid_types)));
			}
			self.arg_type = lit_str.clone();
		}
		
		if let Some(syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Bool(lit_bool), .. })) = iter.next() {
			self.required = lit_bool.value;
		}
		
		if let Some(expr) = iter.next() {
			self.default = Some(expr.clone());
		}
		
		if let Some(syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. })) = iter.next() {
			self.desc = lit_str.clone();
		}
		
		if let Some(syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. })) = iter.next() {
			let valid_modes = ["positional", "named"];
			if !valid_modes.contains(&lit_str.value().as_str()) {
				return Err(syn::Error::new_spanned(lit_str, format!("呜哇~mode 必须是 {:?} 之一！杂鱼~", valid_modes)));
			}
			self.mode = lit_str.clone();
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
			Some(expr) => {
				match arg_type_str.as_str() {
					"int" => quote! { Some(::puniyu_plugin::ArgValue::Int(#expr as i64)) },
					"float" => quote! { Some(::puniyu_plugin::ArgValue::Float(#expr as f64)) },
					"bool" => quote! { Some(::puniyu_plugin::ArgValue::Bool(#expr)) },
					_ => quote! { Some(::puniyu_plugin::ArgValue::String(#expr.to_string())) },
				}
			}
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

impl Parse for Arg {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		if input.is_empty() {
			return Err(syn::Error::new(input.span(), "参数必须指定 name"));
		}
		let fields = Punctuated::<syn::MetaNameValue, Token![,]>::parse_terminated(input)?;
		let mut arg = Arg::default();

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
						arg.name = lit_str.clone();
					} else {
						return Err(syn::Error::new_spanned(&field.value, "呜哇~name 必须是字符串！杂鱼~"));
					}
				}
				"arg_type" | "type" => {
					if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. }) =
						&field.value
					{
						let valid_types = ["string", "int", "float", "bool"];
						if !valid_types.contains(&lit_str.value().as_str()) {
							return Err(syn::Error::new_spanned(
								&field.value,
								format!("呜哇~arg_type 必须是 {:?} 之一！杂鱼~", valid_types),
							));
						}
						arg.arg_type = lit_str.clone();
					} else {
						return Err(syn::Error::new_spanned(&field.value, "呜哇~arg_type 必须是字符串！杂鱼~"));
					}
				}
				"required" => {
					if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Bool(lit_bool), .. }) =
						&field.value
					{
						arg.required = lit_bool.value;
					} else {
						return Err(syn::Error::new_spanned(&field.value, "呜哇~required 必须是布尔值！杂鱼~"));
					}
				}
				"default" => {
					arg.default = Some(field.value.clone());
				}
				"desc" => {
					if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. }) =
						&field.value
					{
						arg.desc = lit_str.clone();
					} else {
						return Err(syn::Error::new_spanned(&field.value, "呜哇~desc 必须是字符串！杂鱼~"));
					}
				}
				"mode" => {
					if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. }) =
						&field.value
					{
						let valid_modes = ["positional", "named"];
						if !valid_modes.contains(&lit_str.value().as_str()) {
							return Err(syn::Error::new_spanned(
								&field.value,
								format!("呜哇~mode 必须是 {:?} 之一！杂鱼~", valid_modes),
							));
						}
						arg.mode = lit_str.clone();
					} else {
						return Err(syn::Error::new_spanned(&field.value, "呜哇~mode 必须是字符串！杂鱼~"));
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

		if arg.name.value().is_empty() {
			return Err(syn::Error::new(input.span(), "参数必须指定 name"));
		}

		Ok(arg)
	}
}
