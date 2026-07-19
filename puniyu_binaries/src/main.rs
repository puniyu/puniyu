#![allow(clippy::unwrap_used)]
use semver::Version;

pub(crate) const NAME: &str = "puniyu";
pub(crate) const VERSION: Version = puniyu_version::VERSION;
pub(crate) const ASSETS: &[u8] =
	include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/logo.png"));

#[tokio::main]
async fn main() -> Result<(), puniyu::AppError> {
	let cwd_dir = Box::leak(Box::new(std::env::current_dir()?));
	puniyu::App::builder()
		.name(NAME)
		.loader(
			puniyu_loader_builtin::Loader::new()
				.with_plugin(
					puniyu_plugin_puniyu::Plugin::with_name(NAME)
						.with_cwd_dir(cwd_dir)
						.with_version(VERSION)
						.with_git_sha(env!("VERGEN_GIT_SHA"))
						.with_repo(env!("CARGO_PKG_REPOSITORY")),
				)
				.with_plugin(puniyu_plugin_log::Plugin)
				.with_plugin(puniyu_plugin_server::Plugin)
				.with_plugin(puniyu_plugin_logo::Plugin::with_logo(ASSETS))
				.with_plugin(puniyu_plugin_event_bus::Plugin)
				.with_plugin(puniyu_plugin_access::Plugin)
				.with_plugin(puniyu_plugin_command::Plugin)
				.with_plugin(puniyu_plugin_task::Plugin),
		)
		.build()
		.run()
		.await
}
