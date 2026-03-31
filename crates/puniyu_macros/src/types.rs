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

#[derive(zyn::Attribute)]
#[zyn("plugin")]
pub(crate) struct PluginArg {
	pub desc: Option<String>,
	pub prefix: Option<String>,
	pub server: Option<zyn::syn::Expr>,
}

#[derive(zyn::Attribute)]
#[zyn("task")]
pub(crate) struct TaskArgs {
	pub name: Option<String>,
	pub cron: String,
}

#[derive(zyn::Attribute)]
#[zyn("arg")]
pub(crate) struct ArgType {
	pub name: String,
	#[zyn("type")]
	pub arg_type: Option<String>,
	pub mode: Option<String>,
	pub required: Option<bool>,
	pub desc: Option<String>,
}

#[derive(zyn::Attribute)]
#[zyn("command")]
pub(crate) struct CommandArgs {
	pub name: String,
	pub priority: Option<u32>,
	pub desc: Option<String>,
	pub alias: Option<Vec<zyn::syn::LitStr>>,
	#[zyn(default = "all")]
	pub permission: Option<String>,
}
