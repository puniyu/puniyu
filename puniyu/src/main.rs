use bytes::Bytes;
use convert_case::{Case, Casing};
use figlet_rs::FIGlet;
use puniyu_core::App;
use puniyu_core::Version;

const fn app_name() -> &'static str {
	env!("CARGO_PKG_NAME")
}

const fn app_version() -> Version {
	Version {
		major: const_str::parse!(env!("CARGO_PKG_VERSION_MAJOR"), u64),
		minor: const_str::parse!(env!("CARGO_PKG_VERSION_MINOR"), u64),
		patch: const_str::parse!(env!("CARGO_PKG_VERSION_PATCH"), u64),
	}
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
	print_start_log();
	App::builder()
		.with_app_name(app_name())
		.with_app_logo(Bytes::from_static(include_bytes!(concat!(
			env!("CARGO_MANIFEST_DIR"),
			"/assets/logo.png"
		))))
		.with_handler(puniyu_handler_command::Handler)
		.with_loader(
			puniyu_loader_builtin::BuiltinLoader::new()
				.with_adapter(puniyu_adapter_console::Adapter::default())
				.with_plugin(puniyu_plugin_basic::Plugin)
				.with_plugin(puniyu_plugin_echo::Plugin),
		)
		.build()
		.run()
		.await
}

fn print_start_log() {
	let app_name = app_name().to_case(Case::Lower);
	if let Ok(standard_font) = FIGlet::standard()
		&& let Some(art_text) = standard_font.convert(app_name.as_str())
	{
		println!("{}", art_text);
	} else {
		println!("{}", app_name);
	}

	println!("{} starting...", app_name.to_case(Case::Lower));
	println!("Version: {}", app_version());
	println!("Git SHA: {}", env!("VERGEN_GIT_SHA"));
	println!("Github: {}", env!("CARGO_PKG_REPOSITORY"));
}
