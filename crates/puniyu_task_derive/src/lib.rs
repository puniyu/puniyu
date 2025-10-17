use convert_case::{Case, Casing};
use croner::Cron;
use proc_macro::TokenStream;
use puniyu_derive_utils::{FieldSetter, get_plugin_name, parse_fields};
use quote::quote;
use std::str::FromStr;
use syn::{
	self, Ident, ItemFn, Token, parse::Parse, parse::ParseStream, parse_macro_input,
	punctuated::Punctuated,
};

struct TaskArgs {
	cron: syn::LitStr,
}

impl Default for TaskArgs {
	fn default() -> Self {
		Self { cron: syn::LitStr::new("", proc_macro2::Span::call_site()) }
	}
}

impl TaskArgs {
	fn set_cron(args: &mut Self, value: syn::LitStr) -> syn::Result<()> {
		args.cron = value;
		Ok(())
	}
}
impl Parse for TaskArgs {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		if input.is_empty() {
			return Err(syn::Error::new(input.span(), "呜~至少给人家一个cron表达式嘛！杂鱼~"));
		}

		let fields = Punctuated::<syn::MetaNameValue, Token![,]>::parse_terminated(input)?;
		let args = parse_fields(&fields, &[("cron", Self::set_cron as FieldSetter<Self>)], &[])?;

		if args.cron.value().is_empty() {
			return Err(syn::Error::new(input.span(), "诶嘿~cron表达式都不给！杂鱼程序员！"));
		}

		Ok(args)
	}
}

#[proc_macro_attribute]
pub fn task(args: TokenStream, item: TokenStream) -> TokenStream {
	let args = parse_macro_input!(args as TaskArgs);
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

	if !input_fn.sig.inputs.is_empty() {
		return syn::Error::new_spanned(
			&input_fn.sig.inputs,
			"呜哇~杂鱼函数居然还想带参数？不行不行！",
		)
		.to_compile_error()
		.into();
	}

	let cron_value = args.cron.value();
	if Cron::from_str(&cron_value).is_err() {
		return syn::Error::new_spanned(&args.cron, "呜哇~, cron表达式都不会写吗？真是杂鱼呢~")
			.to_compile_error()
			.into();
	}

	let crate_name = match get_plugin_name(fn_sig) {
		Ok(name) => name,
		Err(err) => return err.to_compile_error().into(),
	};

	let cron_expr = &args.cron;

	let struct_name_str = {
		let fn_name_str = fn_name.to_string();
		let pascal_case_name = fn_name_str.to_case(Case::Pascal);
		format!("{}Task", pascal_case_name)
	};
	let struct_name = Ident::new(&struct_name_str, fn_name.span());

	let expanded = quote! {
	pub struct #struct_name;

	#fn_vis #fn_sig #fn_block

		use puniyu_core::async_trait;

		#[async_trait]
		impl ::puniyu_plugin::TaskBuilder for #struct_name {
			fn name(&self) -> &'static str {
			stringify!(#fn_name)
		}

			fn cron(&self) -> &'static str {
			#cron_expr
		}

			async fn run(&self) {
				#fn_name().await;
		}
	}

	::puniyu_core::inventory::submit! {
		crate::TaskRegistry  {
			plugin_name: #crate_name,
			builder: || -> Box<dyn ::puniyu_plugin::TaskBuilder> { Box::new(#struct_name {}) },
		}
	}

	};

	TokenStream::from(expanded)
}
