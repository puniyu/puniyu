use std::env;
use syn::Signature;

pub fn get_plugin_name(fn_sig: &Signature) -> syn::Result<String> {
	match env::var("PLUGIN_NAME") {
		Ok(name) => Ok(name),
		Err(_) => Err(syn::Error::new_spanned(fn_sig, "呜哇~PLUGIN_NAME都没有设置！杂鱼程序员！")),
	}
}
