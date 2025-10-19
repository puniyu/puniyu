use crate::{
	VERSION, common,
	common::format_duration,
	config::{config_watcher, init_config},
	logger::log_init,
	logger::{OwoColorize, debug, error, info},
};
use convert_case::{Case, Casing};
use figlet_rs::FIGfont;
use puniyu_adapter_builder::{AdapterBuilder, AdapterType};
use puniyu_adapter_registry::AdapterRegistry;
use puniyu_event_bus::init_event_bus;
use puniyu_plugin_builder::{PluginBuilder, PluginType};
use puniyu_plugin_registry::PluginRegistry;
use puniyu_task_registry::{SCHEDULER, init_scheduler};
pub use puniyu_utils::APP_NAME;
use puniyu_utils::path::{ADAPTER_DIR, PLUGIN_DIR};
use std::sync::{OnceLock, RwLock};
use std::{env, env::consts::DLL_EXTENSION};
use tokio::{fs, signal};

static REGISTERED_PLUGINS: OnceLock<RwLock<Vec<PluginType>>> = OnceLock::new();
static REGISTERED_ADAPTER: OnceLock<RwLock<Vec<AdapterType>>> = OnceLock::new();

pub struct App();

impl Default for App {
	fn default() -> Self {
		APP_NAME.get_or_init(|| String::from("puniyu"));
		Self {}
	}
}

impl App {
	pub fn new() -> Self {
		Self {}
	}
	pub fn with_name(&mut self, name: &str) -> &mut Self {
		APP_NAME.get_or_init(|| name.to_string());
		self
	}

	pub fn add_plugin(&mut self, plugin: &'static dyn PluginBuilder) -> &mut Self {
		let plugins = REGISTERED_PLUGINS.get_or_init(|| RwLock::new(Vec::new()));
		if let Ok(mut plugins_vec) = plugins.write() {
			plugins_vec.push(PluginType::from(plugin));
		}
		self
	}

	pub fn add_adapter(&mut self, adapter: &'static dyn AdapterBuilder) -> &mut Self {
		let adapters = REGISTERED_ADAPTER.get_or_init(|| RwLock::new(Vec::new()));
		if let Ok(mut adapters_vec) = adapters.write() {
			adapters_vec.push(AdapterType::from(adapter));
		}
		self
	}

	pub async fn run(&self) {
		print_start_log();
		init_config();
		log_init();
		let start_time = std::time::Instant::now();
		let app_name = APP_NAME.get().unwrap();
		init_app().await;
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

async fn init_app() {
	if env::var("APP_NAME").is_err() {
		unsafe {
			env::set_var("APP_NAME", APP_NAME.get().unwrap());
		}
	}
	config_watcher();
	let event_bus = init_event_bus();
	event_bus.lock().unwrap().run();
	init_scheduler().await;
	SCHEDULER.get().unwrap().start().await.unwrap();
	init_plugin().await;
	init_adapter().await;
}

async fn init_plugin() {
	if !PLUGIN_DIR.as_path().exists() {
		fs::create_dir(PLUGIN_DIR.as_path()).await.unwrap();
	}

	let plugins = REGISTERED_PLUGINS.get_or_init(|| RwLock::new(Vec::new()));

	let pattern = PLUGIN_DIR.join(format!("*plugin*.{}", DLL_EXTENSION));
	if let Ok(paths) = glob::glob(pattern.to_str().unwrap()) {
		for entry in paths.filter_map(Result::ok) {
			if let Ok(mut plugins_vec) = plugins.write() {
				plugins_vec.push(PluginType::from(entry));
			}
		}
	}

	let plugin_list = {
		let guard = plugins.read().unwrap();
		guard.clone()
	};

	PluginRegistry::load_plugins(plugin_list).await.unwrap_or_else(|e| {
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

async fn init_adapter() {
	if !ADAPTER_DIR.as_path().exists() {
		fs::create_dir(ADAPTER_DIR.as_path()).await.unwrap();
	}

	let adapters = REGISTERED_ADAPTER.get_or_init(|| RwLock::new(Vec::new()));

	let pattern = PLUGIN_DIR.join(format!("*adapter*.{}", DLL_EXTENSION));
	if let Ok(paths) = glob::glob(pattern.to_str().unwrap()) {
		for entry in paths.filter_map(Result::ok) {
			if let Ok(mut adapter_vec) = adapters.write() {
				adapter_vec.push(AdapterType::from(entry));
			}
		}
	}

	let adapter_list = {
		let guard = adapters.read().unwrap();
		guard.clone()
	};

	AdapterRegistry::load_adapters(adapter_list).await.unwrap_or_else(|e| {
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
