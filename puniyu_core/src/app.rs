use crate::VERSION;
use crate::config::{config_watcher, init_config};
use crate::logger::log_init;
use figlet_rs::FIGfont;
use puniyu_registry::{PluginManager, plugin::task::init_task};
use puniyu_utils::path::PLUGIN_DIR;
use std::{env::consts::DLL_EXTENSION, ffi, process, time::Duration};
use tokio::{fs, signal};

const APP_NAME: &str = "PuniYu";

pub struct Bot {}

impl Default for Bot {
	fn default() -> Self {
		print_welcome();
		init_config();
		log_init();
		Self {}
	}
}

impl Bot {
	/// TODO: 添加自定义插件，编译绑定, 这部分后续让开发者自定义
	pub fn new() -> Self {
		Self {}
	}
	pub async fn run(&self) {
		let start_time = std::time::Instant::now();
		init_app().await;
		let duration = start_time.elapsed();
		let duration_str = format_duration(duration);
		log::info!("{} 初始化完成，耗时: {}", APP_NAME, duration_str);
		signal::ctrl_c().await.unwrap()
	}
}

async fn init_app() {
	config_watcher();
	init_plugin().await;
	init_task().await;
}

async fn init_plugin() {
	if !PLUGIN_DIR.as_path().exists() {
		fs::create_dir_all(PLUGIN_DIR.as_path()).await.unwrap();
	}

	// 收集插件名称
	let mut plugin_names = Vec::new();
	if let Ok(mut read_dir) = fs::read_dir(PLUGIN_DIR.as_path()).await {
		while let Some(entry) = read_dir.next_entry().await.unwrap_or(None) {
			if let Some(file_name) = entry.file_name().to_str()
				&& file_name.starts_with("puniyu_plugin_")
				&& entry.path().extension() == Some(ffi::OsStr::new(DLL_EXTENSION))
				&& let Some(plugin_name) = file_name.split('.').next()
			{
				plugin_names.push(plugin_name.to_string());
			}
		}
	}

	PluginManager::load_plugins(plugin_names).await.unwrap_or_else(|e| {
		log::error!("插件加载失败: {:?}", e);
		process::exit(1);
	});
}

fn format_duration(duration: Duration) -> String {
	let minutes = duration.as_secs() / 60;
	let seconds = duration.as_secs() % 60;
	let milliseconds = duration.subsec_millis();

	let mut result = String::new();

	if minutes > 0 {
		result.push_str(&format!("{}分", minutes));
	}

	if seconds > 0 {
		result.push_str(&format!("{}秒", seconds));
	}

	if milliseconds > 0 {
		result.push_str(&format!("{}毫秒", milliseconds));
	}

	if result.is_empty() {
		result.push_str("0秒");
	}

	result
}

fn print_welcome() {
	let standard_font = FIGfont::standard().unwrap();
	let art_text = standard_font.convert(APP_NAME).unwrap();
	println!("{}", art_text);
	println!("{} 启动中...", APP_NAME);
	println!("版本: {}", VERSION);
	println!("Github: {}", env!("CARGO_PKG_REPOSITORY"));
}
