use std::env;
use syn::{Signature, Token, punctuated::Punctuated};

pub type FieldSetter<T> = fn(&mut T, syn::LitStr) -> syn::Result<()>;
pub type FieldArraySetter<T> = fn(&mut T, syn::ExprArray) -> syn::Result<()>;
pub type FieldIntSetter<T> = fn(&mut T, syn::LitInt) -> syn::Result<()>;

pub fn parse_fields<T>(
	fields: &Punctuated<syn::MetaNameValue, Token![,]>,
	str_setters: &[(&str, FieldSetter<T>)],
	int_setters: &[(&str, FieldIntSetter<T>)],
	array_setters: &[(&str, FieldArraySetter<T>)],
) -> syn::Result<T>
where
	T: Default,
{
	let mut args = T::default();

	for field in fields {
		let key = field
			.path
			.get_ident()
			.ok_or_else(|| syn::Error::new_spanned(&field.path, "呜哇~不支持的字段名！杂鱼~"))?;
		let key_str = key.to_string();

		let value = &field.value;

		match value {
			syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. }) => {
				let setter = str_setters
					.iter()
					.find_map(|(k, f)| if k == &key_str { Some(f) } else { None })
					.ok_or_else(|| {
						syn::Error::new_spanned(
							key,
							format!("呜哇~不支持的字段 '{}'！杂鱼~", key_str),
						)
					})?;
				setter(&mut args, lit_str.clone())?;
			}
			syn::Expr::Array(arr) => {
				let setter = array_setters
					.iter()
					.find_map(|(k, f)| if k == &key_str { Some(f) } else { None })
					.ok_or_else(|| {
						syn::Error::new_spanned(
							key,
							format!("呜哇~不支持的字段 '{}'！杂鱼~", key_str),
						)
					})?;
				setter(&mut args, arr.clone())?;
			}
			syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(lit_int), .. }) => {
				let setter = int_setters
					.iter()
					.find_map(|(k, f)| if k == &key_str { Some(f) } else { None })
					.ok_or_else(|| {
						syn::Error::new_spanned(
							key,
							format!("呜哇~不支持的字段 '{}'！杂鱼~", key_str),
						)
					})?;
				setter(&mut args, lit_int.clone())?;
			}
			_ => {
				return Err(syn::Error::new_spanned(
					value,
					"呜哇~只支持字符串、数组和整数值！杂鱼~",
				));
			}
		}
	}

	Ok(args)
}

pub fn get_plugin_name(fn_sig: &Signature) -> syn::Result<String> {
	match env::var("PLUGIN_NAME") {
		Ok(name) => Ok(name),
		Err(_) => Err(syn::Error::new_spanned(fn_sig, "呜哇~PLUGIN_NAME都没有设置！杂鱼程序员！")),
	}
}
