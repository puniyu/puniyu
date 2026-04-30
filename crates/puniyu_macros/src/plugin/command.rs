use crate::{ArgType, CommandArgs, common::validate_async};
use zyn::{ToTokens, zyn};

pub fn command(item: zyn::syn::ItemFn, cfg: CommandArgs) -> zyn::TokenStream {
	let fn_sig = item.sig.clone();
	if let Err(err) = validate_async(&fn_sig) {
		return err.to_compile_error();
	}
	if let Err(err) = validate_command_args(&fn_sig) {
		return err.to_compile_error();
	}
	if let Err(err) = validate_command_return_type(&fn_sig) {
		return err.to_compile_error();
	}

	let fn_name = &item.sig.ident;
	let struct_name = zyn! { {{ fn_name | pascal | ident: "{}Command" }} };
	let plugin_name = zyn! { env!("CARGO_PKG_NAME") };
	let command_name = zyn! { {{ cfg.name | str }} };
	let command_priority = match cfg.priority {
		Some(p) => zyn! { {{ p }} },
		None => zyn! { 500u32 },
	};
	let command_desc = match &cfg.desc {
		Some(desc) => zyn! { Some({{ desc | str }}) },
		None => zyn! { None },
	};
	let command_permission = match cfg.permission.as_deref().unwrap_or("all") {
		"all" => zyn! { ::puniyu_plugin::command::Permission::All },
		"master" => zyn! { ::puniyu_plugin::command::Permission::Master },
		"owner" => zyn! { ::puniyu_plugin::command::Permission::Owner },
		"admin" => zyn! { ::puniyu_plugin::command::Permission::Admin },
		invalid => {
			return zyn::syn::Error::new_spanned(
				&item.sig.ident,
				format!("invalid command permission: {invalid}"),
			)
			.to_compile_error();
		}
	};
	let command_alias = {
		let aliases = cfg.alias.as_deref().unwrap_or(&[]);
		if aliases.is_empty() {
			zyn! { ::std::vec![] }
		} else {
			let mut ts = zyn::TokenStream::new();
			for alias in aliases {
				ts.extend(zyn! { {{ alias }}, }.to_token_stream());
			}
			zyn! { ::std::vec![{{ ts }}] }
		}
	};
	let args_tokens = {
		let mut ts = zyn::TokenStream::new();
		for attr in &item.attrs {
			if !attr.path().is_ident("arg") {
				continue;
			}
			let list = match attr.meta.require_list() {
				Ok(l) => l,
				Err(e) => return e.to_compile_error(),
			};
			let args: zyn::meta::Args = match zyn::syn::parse2(list.tokens.clone()) {
				Ok(a) => a,
				Err(e) => return e.to_compile_error(),
			};
			let arg = match ArgType::from_args(&args) {
				Ok(a) => a,
				Err(e) => return e.emit_as_item_tokens(),
			};
			let arg_name = &arg.name;
			let constructor = match arg.arg_type.as_deref().unwrap_or("string") {
				"integer" => zyn! { ::puniyu_plugin::command::Arg::integer({{ arg_name | str }}) },
				"boolean" => zyn! { ::puniyu_plugin::command::Arg::boolean({{ arg_name | str }}) },
				_ => zyn! { ::puniyu_plugin::command::Arg::string({{ arg_name | str }}) },
			};
			let mode_method = match arg.mode.as_deref().unwrap_or("positional") {
				"optional" => zyn! { .optional() },
				_ => zyn! { .positional() },
			};
			let required_method = if arg.required.unwrap_or(false) {
				zyn! { .required() }.to_token_stream()
			} else {
				zyn::TokenStream::new()
			};
			let desc_method = match &arg.desc {
				Some(desc) => zyn! { .description({{ desc | str }}) }.to_token_stream(),
				None => zyn::TokenStream::new(),
			};
			ts.extend(
				zyn! { #constructor #mode_method #required_method #desc_method, }.to_token_stream(),
			);
		}
		ts
	};

	zyn! {
		{{ item }}

		struct {{ struct_name }};

		#[::puniyu_plugin::__private::async_trait]
		impl ::puniyu_plugin::__private::Command for {{ struct_name }} {
			fn name(&self) -> &str {
				#command_name
			}

			fn description(&self) -> Option<&str> {
				#command_desc
			}

			fn priority(&self) -> u32 {
				#command_priority
			}

			fn args(&self) -> ::std::vec::Vec<::puniyu_plugin::command::Arg<'_>> {
				::std::vec![{{ args_tokens }}]
			}

			fn alias(&self) -> ::std::vec::Vec<&str> {
				#command_alias
			}

			fn permission(&self) -> ::puniyu_plugin::command::Permission {
				#command_permission
			}

			#[inline]
			async fn execute(
				&self,
				ctx: &::puniyu_plugin::context::MessageContext,
			) -> ::puniyu_plugin::Result<::puniyu_plugin::command::CommandAction> {
				{{ fn_name }}(ctx).await
			}
		}

		::puniyu_plugin::__private::inventory::submit! {
			crate::CommandRegistry {
				plugin_name: {{ plugin_name }},
				builder: || -> ::std::sync::Arc<dyn ::puniyu_plugin::__private::Command> {
					::std::sync::Arc::new({{ struct_name }} {})
				}
			}
		}
	}
	.to_token_stream()
}

fn validate_command_args(fn_sig: &zyn::syn::Signature) -> zyn::syn::Result<()> {
	use zyn::syn::spanned::Spanned;
	let fn_name = &fn_sig.ident;
	let inputs = &fn_sig.inputs;
	if inputs.len() != 1 {
		return Err(zyn::syn::Error::new(
			fn_sig.span(),
			format!("function `{}` must have exactly 1 parameter, found {}", fn_name, inputs.len()),
		));
	}
	let arg = inputs.first().unwrap();
	let pat_type = match arg {
		zyn::syn::FnArg::Typed(pt) => pt,
		zyn::syn::FnArg::Receiver(_) => {
			return Err(zyn::syn::Error::new(
				arg.span(),
				"command function parameter must not be `self`",
			));
		}
	};
	let inner_type = match pat_type.ty.as_ref() {
		zyn::syn::Type::Reference(r) => &r.elem,
		_ => {
			return Err(zyn::syn::Error::new(
				pat_type.ty.span(),
				"command function parameter must be a reference: `&MessageContext`",
			));
		}
	};
	let is_valid = match inner_type.as_ref() {
		zyn::syn::Type::Path(tp) => {
			let last = tp.path.segments.last().map(|s| s.ident.to_string());
			let full = tp.path.to_token_stream().to_string().replace(' ', "");
			last.as_deref() == Some("MessageContext")
				|| matches!(
					full.as_str(),
					"puniyu_plugin::context::MessageContext"
						| "::puniyu_plugin::context::MessageContext"
				)
		}
		_ => false,
	};
	if !is_valid {
		return Err(zyn::syn::Error::new(
			inner_type.span(),
			"command function parameter type must be `MessageContext` or `puniyu_plugin::context::MessageContext`",
		));
	}
	Ok(())
}

fn validate_command_return_type(fn_sig: &zyn::syn::Signature) -> zyn::syn::Result<()> {
	use zyn::syn::spanned::Spanned;
	let err = |span| {
		Err(zyn::syn::Error::new(
			span,
			"command function must return `puniyu_plugin::Result<CommandAction>` or `puniyu_plugin::Result<puniyu_plugin::command::CommandAction>`",
		))
	};
	let zyn::syn::ReturnType::Type(_, ty) = &fn_sig.output else {
		return err(fn_sig.span());
	};
	let zyn::syn::Type::Path(tp) = ty.as_ref() else {
		return err(ty.span());
	};
	let actual = tp.path.to_token_stream().to_string().replace(' ', "");
	if matches!(
		actual.as_str(),
		"puniyu_plugin::Result<CommandAction>"
			| "::puniyu_plugin::Result<CommandAction>"
			| "puniyu_plugin::Result<puniyu_plugin::command::CommandAction>"
			| "::puniyu_plugin::Result<puniyu_plugin::command::CommandAction>"
			| "puniyu_plugin::Result<::puniyu_plugin::command::CommandAction>"
			| "::puniyu_plugin::Result<::puniyu_plugin::command::CommandAction>"
	) {
		Ok(())
	} else {
		err(ty.span())
	}
}
