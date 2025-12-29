mod registry;
pub mod store;
pub use registry::ConfigRegistry;
mod app;
mod bot;
pub use bot::option::{BotOption, ReactiveMode};
mod group;

use crate::{
	app::{APP_CONFIG, AppConfig},
	bot::{BOT_CONFIG, BotConfig},
	group::{GROUP_CONFIG, GroupConfig},
};
use notify_debouncer_full::{DebounceEventResult, new_debouncer, notify};
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
			new_debouncer(Duration::from_secs(2), None, |res: DebounceEventResult| match res {
				Ok(events) => {
					for event in events.iter() {
						if !matches!(
							event.event.kind,
							notify::EventKind::Modify(_)
								| notify::EventKind::Create(_)
								| notify::EventKind::Remove(_)
						) {
							continue;
						}

						for path in event.event.paths.iter() {
							info!("[Config] 文件变更: {}", path.display());

							if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
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
											.map_err(|e| {
												error!("[Config] 重载Group配置失败: {}", e)
											})
											.unwrap();
									}
									_ => {}
								}

								if ConfigRegistry::all().iter().any(|c| c.path == *path)
									&& let Some(name) = path.file_stem().and_then(|s| s.to_str())
									&& let Some(dir) = path.parent()
									&& let Ok(value) = read_config::<toml::Value>(dir, name)
								{
									ConfigRegistry::update(path, value.clone());
									debug!("[Config] 更新配置: {:#?}", value.clone());
								}
							}
						}
					}
				}
				Err(e) => error!("[Config] 监听错误: {:?}", e),
			})
			.unwrap();

		debouncer.watch(CONFIG_DIR.as_path(), notify::RecursiveMode::Recursive).unwrap();

		thread::park();
	});
}
fn init_env() {
	let config = Config::app();
	if env::var("LOGGER_LEVEL").is_err() {
		let config = config.logger();
		let level = config.level();
		unsafe {
			env::set_var("LOGGER_LEVEL", level);
		}
	}

	if env::var("LOGGER_RETENTION_DAYS").is_err() {
		let config = config.logger();
		let retention_day = config.retention_days();
		unsafe {
			env::set_var("LOGGER_RETENTION_DAYS", retention_day.to_string());
		}
	}
	if env::var("LOGGER_FILE_ENABLE").is_err() {
		let config = config.logger();
		let file_logging = config.enable_file();
		unsafe {
			env::set_var("LOGGER_FILE_ENABLE", file_logging.to_string());
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
}
