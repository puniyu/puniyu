mod registry;
pub mod store;
pub use registry::ConfigRegistry;
mod app;
mod bot;
mod group;

use crate::{
	app::{APP_CONFIG, AppConfig},
	bot::{BOT_CONFIG, BotConfig},
	group::{GROUP_CONFIG, GroupConfig},
};
use notify_debouncer_mini::{DebounceEventResult, new_debouncer, notify};
use puniyu_common::Error;
use puniyu_common::path::LOG_DIR;
use puniyu_common::{
	APP_NAME,
	path::CONFIG_DIR,
	toml::{merge_config, read_config},
};
use puniyu_logger::{debug, error, info};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::{env, thread, time::Duration};

fn reload_config<T>(name: &str, config: &mut T) -> Result<(), Error>
where
	T: Default + DeserializeOwned,
{
	match read_config(CONFIG_DIR.as_path(), name) {
		Ok(new_config) => {
			*config = new_config;
			Ok(())
		}
		Err(_) => {
			*config = T::default();
			Ok(())
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config;
impl Config {
	/// 获取app配置
	///
	/// # 返回值
	///
	/// * `AppConfig` - app配置
	///
	/// # 示例
	///
	/// ```rust, ignore
	/// use puniyu_core::config::Config;
	/// let config = Config::app();
	/// ```
	pub fn app() -> AppConfig {
		AppConfig::get()
	}

	/// 获取群组配置
	///
	/// # 返回值
	/// * `GroupConfig` - 群组配置
	///
	/// # 示例
	///
	/// ```rust, ignore
	/// use puniyu_core::config::Config;
	/// let config = Config::group();
	/// ```
	pub fn group() -> GroupConfig {
		GroupConfig::get()
	}
	/// 获取bot配置
	///
	/// # 返回值
	///
	/// * `BotConfig` - bot配置
	///
	/// # 示例
	///
	/// ```rust, ignore
	/// use puniyu_core::config::Config;
	/// let config = Config::bot();
	/// ```
	pub fn bot() -> BotConfig {
		BotConfig::get()
	}

}

pub fn init_config() {
	if !CONFIG_DIR.as_path().exists() {
		std::fs::create_dir_all(CONFIG_DIR.as_path())
			.unwrap_or_else(|_| error!("[配置文件] 初始化配置文件失败"));
	}
	if !LOG_DIR.as_path().exists() {
		std::fs::create_dir_all(LOG_DIR.as_path())
			.unwrap_or_else(|_| error!("[配置文件] 初始化日志目录失败"));
	}
	merge_config(CONFIG_DIR.as_path(), "app", &AppConfig::default(), &AppConfig::get())
		.unwrap_or_else(|e| {
			error!("[配置文件] 合并APP配置失败: {}", e);
		});
	merge_config(CONFIG_DIR.as_path(), "group", &GroupConfig::default(), &GroupConfig::get())
		.unwrap_or_else(|e| {
			error!("[配置文件] 合并Group配置失败: {}", e);
		});
	merge_config(CONFIG_DIR.as_path(), "bot", &BotConfig::default(), &BotConfig::get())
		.unwrap_or_else(|e| {
			error!("[配置文件] 合并Bot配置失败: {}", e);
		});
	init_env();
}

pub fn start_config_watcher() {
	thread::spawn(|| {
		debug!("[Config] 配置文件监听器已启动");

		let mut debouncer =
			new_debouncer(Duration::from_secs(2), |res: DebounceEventResult| match res {
				Ok(events) => {
					let paths = events
						.iter()
						.map(|e| e.path.display().to_string())
						.collect::<Vec<String>>();

					info!("[Config] 文件变更: {}", paths.join(", "));

					for event in events.iter() {
						if let Some(file_name) = event.path.file_name().and_then(|n| n.to_str()) {
							match file_name {
								"app.toml" => {
									reload_config("app", &mut *APP_CONFIG.write().unwrap())
										.map_err(|e| error!("[Config] 重载App配置失败: {}", e))
										.unwrap();
								}
								"bot.toml" => {
									reload_config("bot", &mut *BOT_CONFIG.write().unwrap())
										.map_err(|e| error!("[Config] 重载Bot配置失败: {}", e))
										.unwrap();
								}
								"group.toml" => {
									reload_config("group", &mut *GROUP_CONFIG.write().unwrap())
										.map_err(|e| error!("[Config] 重载Group配置失败: {}", e))
										.unwrap();
								}
								_ => {
									reload_config("app", &mut *APP_CONFIG.write().unwrap())
										.map_err(|e| error!("[Config] 重载App配置失败: {}", e))
										.unwrap();
									reload_config("bot", &mut *BOT_CONFIG.write().unwrap())
										.map_err(|e| error!("[Config] 重载Bot配置失败: {}", e))
										.unwrap();
									reload_config("group", &mut *GROUP_CONFIG.write().unwrap())
										.map_err(|e| error!("[Config] 重载Group配置失败: {}", e))
										.unwrap();
								}
							}

							if ConfigRegistry::all().iter().any(|c| c.path == event.path)
								&& let Some(name) = event.path.file_stem().and_then(|s| s.to_str())
								&& let Some(dir) = event.path.parent()
								&& let Ok(value) = read_config::<toml::Value>(dir, name)
							{
								ConfigRegistry::update(&event.path, value.clone());
								debug!("[Config] 更新配置: {:#?}", value.clone());
							}
						}
					}
				}
				Err(e) => error!("[Config] 监听错误: {}", e),
			})
			.unwrap();

		debouncer
			.watcher()
			.watch(CONFIG_DIR.as_path(), notify::RecursiveMode::Recursive)
			.unwrap();

		thread::park();
	});
}
fn init_env() {
	env::var("APP_NAME").unwrap_or_else(|_| {
		let app_name = APP_NAME.get().unwrap();
		unsafe {
			env::set_var("APP_NAME", app_name);
		}
		app_name.to_owned()
	});
}
