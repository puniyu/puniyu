use crate::{
	VERSION, common,
	common::format_duration,
	logger::log_init,
	logger::{OwoColorize, debug, error, info},
};
use convert_case::{Case, Casing};
use figlet_rs::FIGfont;
use puniyu_builder::adapter::AdapterBuilder;
use puniyu_builder::plugin::{PluginBuilder, PluginType};
pub use puniyu_common::APP_NAME;
use puniyu_common::path::{DATA_DIR, PLUGIN_DATA_DIR, PLUGIN_DIR, set_working_dir};
use puniyu_config::{init_config, init_config_watcher};
use puniyu_event_bus::init_event_bus;
use puniyu_registry::{AdapterRegistry, PluginRegistry};
use puniyu_task::{SCHEDULER, init_scheduler};
use std::env::current_dir;
use std::path::{Path, PathBuf};
use std::{env, env::consts::DLL_EXTENSION};
use tokio::{fs, signal};

pub struct AppBuilder {
	app_name: String,
	working_dir: PathBuf,
	plugins: Vec<&'static dyn PluginBuilder>,
	adapters: Vec<&'static dyn AdapterBuilder>,
}

impl Default for AppBuilder {
	fn default() -> Self {
		Self {
			app_name: String::from("puniyu"),
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
		set_working_dir(self.working_dir.as_path());
		APP_NAME.get_or_init(|| self.app_name.clone());
		App { plugins: self.plugins.clone(), adapters: self.adapters.clone() }
	}
}
pub struct App {
	plugins: Vec<&'static dyn PluginBuilder>,
	adapters: Vec<&'static dyn AdapterBuilder>,
}

impl App {
	pub async fn run(&self) {
		print_start_log();
		init_config();
		log_init();
		let start_time = std::time::Instant::now();
		let app_name = APP_NAME.get().unwrap();
		init_app(self.plugins.clone(), self.adapters.clone()).await;
		let duration = start_time.elapsed();
		let duration_str = format_duration(duration);
		info!(
			"{} 初始化完成，耗时: {}",
			app_name.to_case(Case::Lower).fg_rgb::<64, 224, 208>(),
			duration_str.fg_rgb::<255, 127, 80>()
		);
		#[cfg(feature = "server")]
		{
			use crate::config::Config;
			use std::net::IpAddr;
			let config = Config::app().server();
			let host = IpAddr::V4(config.host().parse().unwrap());
			let port = config.port();
			puniyu_server::run_server_spawn(Some(host), Some(port)).await;
		}
		signal::ctrl_c().await.unwrap();
		info!(
			"{} 本次运行时间: {}",
			app_name.to_case(Case::Lower).fg_rgb::<64, 224, 208>(),
			common::uptime().fg_rgb::<255, 127, 80>()
		);
	}
}

async fn init_app(
	plugins: Vec<&'static dyn PluginBuilder>,
	adapters: Vec<&'static dyn AdapterBuilder>,
) {
	use crate::config::Config;
	if env::var("APP_NAME").is_err() {
		unsafe {
			env::set_var("APP_NAME", APP_NAME.get().unwrap());
		}
	}
	if env::var("HTTP_HOST").is_err() {
		unsafe {
			env::set_var("HTTP_HOST", Config::app().server().host());
		}
	}
	if env::var("HTTP_PORT").is_err() {
		unsafe {
			env::set_var("HTTP_PORT", Config::app().server().port().to_string());
		}
	}
	if !DATA_DIR.as_path().exists() {
		fs::create_dir(DATA_DIR.as_path()).await.unwrap();
	}
	init_config_watcher();
	let event_bus = init_event_bus();
	event_bus.lock().unwrap().run();
	init_scheduler().await;
	SCHEDULER.get().unwrap().start().await.unwrap();
	init_plugin(plugins).await;
	init_adapter(adapters).await;
}

async fn init_plugin(plugins: Vec<&'static dyn PluginBuilder>) {
	if !PLUGIN_DIR.as_path().exists() {
		fs::create_dir(PLUGIN_DIR.as_path()).await.unwrap();
	}

	if !PLUGIN_DATA_DIR.as_path().exists() {
		fs::create_dir(PLUGIN_DATA_DIR.as_path()).await.unwrap();
	}
	let mut plugins_list =
		plugins.iter().map(|p| PluginType::Builder(*p)).collect::<Vec<PluginType>>();

	let pattern = PLUGIN_DIR.join(format!("*.{}", DLL_EXTENSION));
	if let Ok(paths) = glob::glob(pattern.to_str().unwrap()) {
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
