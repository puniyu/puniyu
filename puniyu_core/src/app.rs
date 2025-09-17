use crate::logger::log_init;
use puniyu_registry::{PluginManager, plugin::task::init_task};
use puniyu_utils::path::PLUGIN_DIR;
use std::{env::consts::DLL_EXTENSION, ffi};
use tokio::{fs, signal};

pub struct Bot {
	/// 是否以守护进程运行
	is_daemon: bool,
}

impl Default for Bot {
	fn default() -> Self {
		log_init();
		Self { is_daemon: false }
	}
}

impl Bot {
	/// TODO: 添加自定义插件，编译绑定
	pub fn new() -> Self {
		Self::default()
	}
	/// 设置是否以守护进程运行
	pub fn daemon(mut self) -> Self {
		self.is_daemon = true;
		self
	}
	pub async fn run(&self) {
		init_app().await;
		signal::ctrl_c().await.unwrap()
	}
}

async fn init_app() {
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

	PluginManager::load_plugins(plugins).await.unwrap()
}
