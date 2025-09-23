use crate::config::{config_watcher, init_config};
use crate::logger::log_init;
use puniyu_registry::{PluginManager, plugin::task::init_task};
use puniyu_utils::path::PLUGIN_DIR;
use std::{env::consts::DLL_EXTENSION, ffi, process, time::Duration};
use tokio::{fs, signal};

pub struct Bot {
	/// 是否以守护进程运行
	is_daemon: bool,
}

impl Default for Bot {
	fn default() -> Self {
		init_config();
		log_init();
		Self { is_daemon: false }
	}
}

impl Bot {
	/// TODO: 添加自定义插件，编译绑定, 这部分后续让开发者自定义
	pub fn new() -> Self {
		Self::default()
	}
	/// 设置是否以守护进程运行
	pub fn daemon(mut self) -> Self {
		self.is_daemon = true;
		self
	}
	pub async fn run(&self) {
		let start_time = std::time::Instant::now();
		init_app().await;
		let duration = start_time.elapsed();
		let duration_str = format_duration(duration);
		log::info!("应用初始化完成，耗时: {}", duration_str);
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
	let mut plugins = Vec::new();
	if let Ok(mut read_dir) = fs::read_dir(PLUGIN_DIR.as_path()).await {
		while let Some(entry) = read_dir.next_entry().await.unwrap_or(None) {
			if let Some(file_name) = entry.file_name().to_str()
				&& file_name.starts_with("puniyu_plugin_")
				&& entry.path().extension() == Some(ffi::OsStr::new(DLL_EXTENSION))
				&& let Some(plugin_name) = file_name.split('.').next()
			{
				plugins.push(plugin_name.to_string());
			}
		}
	}

	PluginManager::load_plugins(plugins).await.unwrap_or_else(|e| {
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
