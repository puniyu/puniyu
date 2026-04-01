mod adapter;
mod common;
mod plugin;
mod types;
pub(crate) use types::*;

use zyn::{ToTokens, syn::spanned::Spanned};

#[zyn::attribute]
pub fn adapter_config(
	#[zyn(input)] item: zyn::syn::ItemStruct,
	args: zyn::Args,
) -> zyn::TokenStream {
	let cfg = match ConfigArgs::from_args(&args) {
		Ok(cfg) => cfg,
		Err(err) => bail!("{err}"),
	};
	adapter::config(item, cfg)
}


#[zyn::attribute(debug(pretty))]
pub fn adapter_hook(
	#[zyn(input)] item: zyn::syn::ItemFn,
	args: zyn::Args,
) -> proc_macro::TokenStream {
	let cfg = match HookArgs::from_args(&args) {
		Ok(cfg) => cfg,
		Err(err) => bail!("{err}"),
	};
	adapter::hook(item, cfg)
}

#[zyn::attribute(debug(pretty))]
pub fn adapter(#[zyn(input)] item: zyn::syn::ItemFn, args: zyn::Args) -> proc_macro::TokenStream {
	let cfg = match AdapterArgs::from_args(&args) {
		Ok(cfg) => cfg,
		Err(err) => bail!("{err}"),
	};
	adapter::adapter(item, cfg)
}

#[zyn::attribute(debug(pretty))]
pub fn plugin(#[zyn(input)] item: zyn::syn::ItemFn, args: zyn::Args) -> proc_macro::TokenStream {
	let cfg = match PluginArg::from_args(&args) {
		Ok(cfg) => cfg,
		Err(err) => bail!("{err}"),
	};
	plugin::plugin(item, cfg)
}

#[zyn::attribute(debug(pretty))]
pub fn plugin_config(
	#[zyn(input)] item: zyn::syn::ItemStruct,
	args: zyn::Args,
) -> proc_macro::TokenStream {
	let cfg = match ConfigArgs::from_args(&args) {
		Ok(cfg) => cfg,
		Err(err) => bail!("{err}"),
	};
	plugin::config(item, cfg)
}

#[zyn::attribute(debug(pretty))]
pub fn command(#[zyn(input)] item: zyn::syn::ItemFn, cfg: zyn::Args) -> proc_macro::TokenStream {
	let cfg = match CommandArgs::from_args(&cfg) {
		Ok(cfg) => cfg,
		Err(err) => bail!("{err}"),
	};
	plugin::command(item, cfg)
}

#[zyn::attribute(debug(pretty))]
pub fn arg(#[zyn(input)] item: zyn::syn::ItemFn) -> proc_macro::TokenStream {
	item.to_token_stream()
}

#[zyn::attribute(debug(pretty))]
pub fn task(#[zyn(input)] item: zyn::syn::ItemFn, args: zyn::Args) -> proc_macro::TokenStream {
	let cfg = match TaskArgs::from_args(&args) {
		Ok(cfg) => cfg,
		Err(err) => bail!("{err}"),
	};
	plugin::task(item, cfg)
}

#[zyn::attribute(debug(pretty))]
pub fn plugin_hook(
	#[zyn(input)] item: zyn::syn::ItemFn,
	args: zyn::Args,
) -> proc_macro::TokenStream {
	let cfg = match HookArgs::from_args(&args) {
		Ok(cfg) => cfg,
		Err(err) => bail!("{err}"),
	};
	plugin::hook(item, cfg)
}
