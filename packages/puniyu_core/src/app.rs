use crate::{
	VERSION, common,
	logger::log_init,
	logger::{OwoColorize, debug, error, info},
};
use convert_case::{Case, Casing};
use figlet_rs::FIGfont;
use puniyu_bus::{EVENT_BUS, EventBusExt, init_event_bus};
pub use puniyu_common::APP_NAME;
use puniyu_common::path::{DATA_DIR, PLUGIN_DATA_DIR, PLUGIN_DIR, RESOURCE_DIR, WORKING_DIR};
use puniyu_config::{init_config, start_config_watcher};
use puniyu_registry::{adapter::AdapterRegistry, plugin::PluginRegistry};
use puniyu_types::adapter::AdapterBuilder;
use puniyu_types::plugin::{PluginBuilder, PluginType};
use std::env::current_dir;
use std::path::{Path, PathBuf};
use std::{env, env::consts::DLL_EXTENSION};
use tokio::{fs, signal};

pub struct AppBuilder {
	app_name: String,
	app_logo: Vec<u8>,
	working_dir: PathBuf,
	plugins: Vec<&'static dyn PluginBuilder>,
	adapters: Vec<&'static dyn AdapterBuilder>,
}

impl Default for AppBuilder {
	fn default() -> Self {
		Self {
			app_name: String::from("puniyu"),
			app_logo: include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/logo.png"))
				.to_vec(),
			working_dir: current_dir().unwrap(),
			plugins: Vec::new(),
			adapters: Vec::new(),
		}
	}
}

impl AppBuilder {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn with_name(&mut self, name: &str) -> &mut Self {
		self.app_name = name.to_string();
		self
	}

	pub fn with_logo(&mut self, logo: Vec<u8>) -> &mut Self {
		self.app_logo = logo;
		self
	}

	pub fn with_working_dir(&mut self, path: &Path) -> &Self {
		self.working_dir = path.to_path_buf();
		self
	}

	pub fn with_plugin(&mut self, plugin: &'static dyn PluginBuilder) -> &mut Self {
		self.plugins.push(plugin);
		self
	}

	pub fn with_adapter(&mut self, adapter: &'static dyn AdapterBuilder) -> &mut Self {
		self.adapters.push(adapter);
		self
	}

	pub fn build(&self) -> App {
		WORKING_DIR.get_or_init(|| self.working_dir.clone());
		APP_NAME.get_or_init(|| self.app_name.clone());
		App {
			app_name: self.app_name.clone(),
			app_logo: self.app_logo.clone(),
			plugins: self.plugins.clone(),
			adapters: self.adapters.clone(),
		}
	}
}

pub struct App {
	app_name: String,
	app_logo: Vec<u8>,
	plugins: Vec<&'static dyn PluginBuilder>,
	adapters: Vec<&'static dyn AdapterBuilder>,
}

impl App {
	pub async fn run(&self) {
		use crate::common::format_duration;
		use std::time::Duration;
		print_start_log();
		init_config();
		log_init();
		let start_time = std::time::Instant::now().elapsed();
		let app_name = self.app_name.clone();
		init_app(self.plugins.clone(), self.adapters.clone()).await;
		let logo_path = RESOURCE_DIR.join("logo.png");
		if !logo_path.exists() {
			fs::write(&logo_path, &self.app_logo).await.expect("写入logo失败");
			puniyu_server::LOGO.get_or_init(|| self.app_logo.clone());
		}
		start_config_watcher();
		let duration_str = format_duration(start_time);
		info!(
			"{} 初始化完成，耗时: {}",
			app_name.to_case(Case::Lower).fg_rgb::<64, 224, 208>(),
			duration_str.fg_rgb::<255, 127, 80>()
		);
		#[cfg(feature = "server")]
		{
			use crate::config::Config;
			use std::net::IpAddr;
			let config = Config::app();
			let config = config.server();
			let host = IpAddr::V4(config.host().parse().unwrap());
			let port = config.port();
			puniyu_server::run_server_spawn(Some(host), Some(port));
		}
		signal::ctrl_c().await.unwrap();
		info!(
			"{} 本次运行时间: {}",
			app_name.to_case(Case::Lower).fg_rgb::<64, 224, 208>(),
			format_duration(Duration::from_secs(common::uptime())).fg_rgb::<255, 127, 80>()
		);
	}
}

async fn init_app(
	plugins: Vec<&'static dyn PluginBuilder>,
	adapters: Vec<&'static dyn AdapterBuilder>,
) {
	if !DATA_DIR.as_path().exists() {
		fs::create_dir(DATA_DIR.as_path()).await.unwrap();
	}

	if !RESOURCE_DIR.as_path().exists() {
		fs::create_dir(RESOURCE_DIR.as_path()).await.unwrap();
	}
	init_event_bus();
	let event_bus = EVENT_BUS.get().unwrap();
	event_bus.run();
	init_plugin(plugins).await;
	init_adapter(adapters).await;
}

async fn init_plugin(plugins: Vec<&'static dyn PluginBuilder>) {
	if !PLUGIN_DIR.as_path().exists() {
		fs::create_dir(PLUGIN_DIR.as_path()).await.expect("Failed to create plugin directory");
	}

	if !PLUGIN_DATA_DIR.as_path().exists() {
		fs::create_dir(PLUGIN_DATA_DIR.as_path())
			.await
			.expect("Failed to create plugin data directory");
	}
	let mut plugins_list =
		plugins.iter().map(|p| PluginType::Builder(*p)).collect::<Vec<PluginType>>();

	let pattern = PLUGIN_DIR.join(format!("*.{}", DLL_EXTENSION));
	if let Ok(paths) = glob::glob(pattern.to_str().expect("Failed to parse plugin path")) {
		for entry in paths.filter_map(Result::ok) {
			plugins_list.push(entry.into());
		}
	}

	PluginRegistry::load_plugins(plugins_list).await.unwrap_or_else(|e| {
		error!("插件加载失败: {:?}", e);
	});

	let plugin_count = PluginRegistry::get_all_plugins().len();
	debug!(
		"{}: {} {}",
		"共加载".fg_rgb::<135, 206, 250>(),
		plugin_count,
		"个插件".fg_rgb::<135, 206, 250>()
	)
}

async fn init_adapter(adapters: Vec<&'static dyn AdapterBuilder>) {
	AdapterRegistry::load_adapters(adapters).await.unwrap_or_else(|e| {
		error!("适配器加载失败: {:?}", e);
	});

	let adapter_count = AdapterRegistry::get_all_adapters().len();

	debug!(
		"{}: {} {}",
		"共加载".fg_rgb::<135, 206, 250>(),
		adapter_count,
		"个适配器".fg_rgb::<135, 206, 250>()
	)
}

fn print_start_log() {
	let app_name = APP_NAME.get().unwrap();
	let standard_font = FIGfont::standard().unwrap();
	let art_text = standard_font.convert(app_name.to_case(Case::Pascal).as_str()).unwrap();
	println!("{}", art_text);
	println!("{} 启动中...", app_name.to_case(Case::Lower));
	println!("版本: {}", VERSION);
	println!("Github: {}", env!("CARGO_PKG_REPOSITORY"));
}
