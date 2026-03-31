#[derive(zyn::Attribute)]
#[zyn("config")]
pub(crate) struct ConfigArgs {
	#[zyn(default)]
	pub name: Option<String>,
}

#[derive(zyn::Attribute)]
#[zyn("adapter")]
pub(crate) struct AdapterArgs {
	pub info: zyn::syn::Expr,
	pub api: zyn::syn::Expr,
	#[zyn(default)]
	pub server: Option<zyn::syn::Expr>,
}

#[derive(zyn::Attribute)]
#[zyn("hook")]
pub(crate) struct HookArgs {
	#[zyn(default)]
	pub name: Option<String>,
	#[zyn("type")]
	pub hook_type: Option<String>,
	#[zyn(default = "500")]
	pub priority: Option<u32>,
}
