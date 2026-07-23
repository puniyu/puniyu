#![allow(clippy::unwrap_used)]

use std::{io, str::FromStr};

use convert_case::{Case, Casing};
use semver::Version;

const NAME: &str = "puniyu";
const VERSION: Version = puniyu_version::VERSION;
const ASSETS: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/logo.png"));

#[tokio::main]
async fn main() {
	let cwd_dir = Box::leak(Box::new(std::env::current_dir().unwrap()));

	puniyu_app::App::init(NAME, cwd_dir, VERSION);
	banner();
	log_init();

	let app = puniyu::App::builder()
		.name(NAME)
		.on_start(load)
		.loader(
			puniyu_loader_builtin::Loader::new()
				.with_service(puniyu_service_config::Service)
				.with_service(puniyu_service_server::Service)
				.with_service(puniyu_service_event::Service)
				.with_service(puniyu_service_command::Service)
				.with_service(puniyu_service_task::Service)
				.with_plugin(puniyu_plugin_config::Plugin)
				.with_plugin(puniyu_plugin_event::Plugin)
				.with_plugin(puniyu_plugin_logo::Plugin::with_logo(ASSETS))
				.with_plugin(puniyu_plugin_access::Plugin::new())
				.with_plugin(puniyu_plugin_command::Plugin::new()),
		)
		.build();

	if let Err(error) = app.run().await {
		log::error!("{error}");
	}
}

fn banner() {
	use figlet_rs::FIGlet;
	let app_name = NAME.to_case(Case::Lower);
	println!("{} starting...", app_name);
	if let Ok(standard_font) = FIGlet::standard()
		&& let Some(art_text) = standard_font.convert(app_name.as_str())
	{
		println!("{}", art_text);
	} else {
		println!("{}", app_name);
	}
	println!("Version: {}", VERSION);
	println!("Git SHA: {}", env!("VERGEN_GIT_SHA"));
	println!("Github: {}", env!("CARGO_PKG_REPOSITORY"));
}

fn log_init() {
	use log::LevelFilter;
	use puniyu_config::app::AppConfig;
	use puniyu_logger::LoggerOptions;
	use puniyu_path::log_dir;
	use std::env;

	let config =
		AppConfig::from_path(puniyu_path::config_dir().join("app").with_extension("toml")).logger();
	let log_level = env::var("LOGGER_LEVEL").unwrap_or(config.level().to_string());
	let log_path = log_dir().to_string_lossy().to_string();
	let log_retention_days = config.retention_days();
	let is_file_logging = config.enable_file();
	let options = LoggerOptions::default()
		.with_prefix(puniyu_app::App::name())
		.with_level(LevelFilter::from_str(log_level.as_str()).unwrap_or(LevelFilter::Info))
		.with_file_logging(is_file_logging)
		.with_log_directory(log_path)
		.with_retention_days(log_retention_days);
	puniyu_logger::init(Some(options));
}

async fn load() {
	init_dir().await.unwrap()
}

async fn init_dir() -> io::Result<()> {
	let dirs = [
		puniyu_path::app_dir(),
		puniyu_path::adapter_dir(),
		puniyu_path::data_dir(),
		puniyu_path::config_dir(),
		puniyu_path::assets_dir(),
		puniyu_path::plugin_dir(),
		puniyu_path::log_dir(),
		puniyu_path::temp_dir(),
	];
	for dir in dirs {
		tokio::fs::create_dir_all(dir).await?;
	}
	Ok(())
}
