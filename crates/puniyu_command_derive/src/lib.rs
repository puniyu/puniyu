use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use puniyu_derive_utils::{FieldArraySetter, FieldSetter, get_plugin_name, parse_fields};
use quote::quote;
use syn::{
	self, Ident, ItemFn, Token, parse::Parse, parse::ParseStream, parse_macro_input,
	punctuated::Punctuated,
};

struct CommandArgs {
	name: syn::LitStr,
	args: Vec<syn::LitStr>,
	rank: syn::LitStr,
	desc: syn::LitStr,
}

impl CommandArgs {
	pub fn set_name(args: &mut Self, value: syn::LitStr) -> syn::Result<()> {
		args.name = value;
		Ok(())
	}

	pub fn set_args(args: &mut Self, value: syn::ExprArray) -> syn::Result<()> {
		let mut lit_strs = Vec::new();
		for expr in value.elems {
			if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. }) = expr {
				lit_strs.push(lit_str);
			} else {
				return Err(syn::Error::new_spanned(expr, "呜哇~args数组中只能包含字符串！杂鱼~"));
			}
		}
		args.args = lit_strs;
		Ok(())
	}

	pub fn set_rank(args: &mut Self, value: syn::LitStr) -> syn::Result<()> {
		args.rank = value;
		Ok(())
	}

	pub fn set_desc(args: &mut Self, value: syn::LitStr) -> syn::Result<()> {
		args.desc = value;
		Ok(())
	}
}

impl Default for CommandArgs {
	fn default() -> Self {
		Self {
			name: syn::LitStr::new("", proc_macro2::Span::call_site()),
			args: Vec::new(),
			rank: syn::LitStr::new("0", proc_macro2::Span::call_site()),
			desc: syn::LitStr::new("", proc_macro2::Span::call_site()),
		}
	}
}

impl Parse for CommandArgs {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		if input.is_empty() {
			return Err(syn::Error::new(input.span(), "呜~至少给人家一些参数嘛！杂鱼~"));
		}

		let fields = Punctuated::<syn::MetaNameValue, Token![,]>::parse_terminated(input)?;
		let args = parse_fields(
			&fields,
			&[
				("name", Self::set_name as FieldSetter<Self>),
				("rank", Self::set_rank as FieldSetter<Self>),
				("desc", Self::set_desc as FieldSetter<Self>),
			],
			&[("args", Self::set_args as FieldArraySetter<Self>)],
		)?;

		if args.name.value().is_empty() {
			return Err(syn::Error::new(input.span(), "诶嘿~name都不给！杂鱼程序员！"));
		}

		Ok(args)
	}
}

#[proc_macro_attribute]
pub fn command(args: TokenStream, item: TokenStream) -> TokenStream {
	let args = parse_macro_input!(args as CommandArgs);
	let input_fn = parse_macro_input!(item as ItemFn);
	let fn_name = &input_fn.sig.ident;
	let fn_vis = &input_fn.vis;
	let fn_sig = &input_fn.sig;
	let fn_block = &input_fn.block;

	let is_async = input_fn.sig.asyncness.is_some();
	if !is_async {
		return syn::Error::new_spanned(&input_fn.sig, "诶嘿~杂鱼函数连async都不会用吗？")
			.to_compile_error()
			.into();
	}

	if input_fn.sig.inputs.len() != 2 {
		return syn::Error::new_spanned(
			&input_fn.sig.inputs,
			"呜哇~命令函数必须有两个参数：&Bot, &EventContext！笨蛋！",
		)
		.to_compile_error()
		.into();
	}

	let struct_name_str = {
		let fn_name_str = fn_name.to_string();
		let pascal_case_name = fn_name_str.to_case(Case::Pascal);
		format!("{}Command", pascal_case_name)
	};

	let command_name = &args.name;
	let command_args: Vec<String> = args.args.iter().map(|lit_str| lit_str.value()).collect();
	let command_rank = &args.rank;
	let command_desc = &args.desc;

	let crate_name = match get_plugin_name(fn_sig) {
		Ok(name) => name,
		Err(err) => return err.to_compile_error().into(),
	};

	let struct_name = Ident::new(&struct_name_str, fn_name.span());

	let expanded = quote! {
		pub struct #struct_name;

		#fn_vis #fn_sig #fn_block

		use puniyu_core::async_trait;

		#[async_trait]
		impl ::puniyu_plugin::CommandBuilder for #struct_name {
			fn name(&self) -> &'static str {
				#command_name
			}

			fn description(&self) -> Option<&'static str> {
				if #command_desc.is_empty() {
					None
				} else {
					Some(#command_desc)
				}
			}

			fn args(&self) -> Vec<String> {
				vec![#(#command_args),*]
			}

			fn rank(&self) -> usize {
				#command_rank.parse().unwrap_or(0)
			}

			async fn run(&self, bot: &Bot, ev: &EventContext) -> ::puniyu_plugin::HandlerResult {
				#fn_name(bot, ev).await
			}
		}

		::puniyu_core::inventory::submit! {
			crate::CommandRegistry {
				plugin_name: #crate_name,
				builder: || -> Box<dyn ::puniyu_plugin::CommandBuilder> { Box::new(#struct_name {}) },
			}
		}
	};

	TokenStream::from(expanded)
}
