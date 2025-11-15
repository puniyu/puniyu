use crate::{
	app::{APP_CONFIG, AppConfig},
	bot::{BOT_CONFIG, BotConfig},
	group::{GROUP_CONFIG, GroupConfig},
};
use notify::{Config as WatcherConfig, Event, RecommendedWatcher, RecursiveMode, Watcher};
use puniyu_common::error::Config as Error;
use puniyu_common::path::LOG_DIR;
use puniyu_common::{path::CONFIG_DIR, toml::merge_config};
use puniyu_logger::{debug, error, info};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::{env, thread, time::Duration};

mod app;
mod bot;
mod group;

fn reload_config<T>(name: &str, config: &mut T) -> Result<(), Error>
where
	T: Default + DeserializeOwned,
{
	match puniyu_common::toml::read_config(CONFIG_DIR.as_path(), name) {
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

pub fn init_config_watcher() {
	let (tx, rx) = flume::bounded::<notify::Result<Event>>(50);

	let mut watcher = RecommendedWatcher::new(
		move |res| {
			tx.send(res).unwrap();
		},
		WatcherConfig::default()
			.with_follow_symlinks(false)
			.with_poll_interval(Duration::from_secs(200)),
	)
	.unwrap();
	watcher.watch(CONFIG_DIR.as_path(), RecursiveMode::NonRecursive).unwrap();

	thread::spawn(move || {
		debug!("[Config] 配置文件监听器已启动");
		for res in rx {
			match res {
				Ok(event) => {
					info!(
						"[Config] 文件变更: {}",
						event
							.paths
							.iter()
							.map(|p| p.display().to_string())
							.collect::<Vec<_>>()
							.join(", ")
					);

					for path in event.paths.iter() {
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
						}
					}
				}
				Err(e) => {
					error!("[Config] 监听错误: {}", e);
				}
			}
		}
	});
}
fn init_env() {
	let logger_config = Config::app().logger();
	env::var("LOGGER_ENABLE").unwrap_or_else(|_| {
		let logger_enable = logger_config.enable_file();
		unsafe {
			env::set_var("LOGGER_FILE_ENABLE", logger_enable.to_string());
		}
		logger_enable.to_string()
	});
	env::var("LOGGER_LEVEL").unwrap_or_else(|_| {
		let logger_level = logger_config.level();
		unsafe {
			env::set_var("LOGGER_LEVEL", logger_level);
		}
		logger_level.to_string()
	});

	env::var("LOGGER_PATH").unwrap_or_else(|_| {
		let logger_path = logger_config.path();
		unsafe {
			env::set_var("LOGGER_PATH", logger_path);
		}
		logger_path.to_string_lossy().to_string()
	});

	env::var("LOGGER_RETENTION_DAYS").unwrap_or_else(|_| {
		let logger_retention_days = logger_config.retention_days();
		unsafe {
			env::set_var("LOGGER_RETENTION_DAYS", logger_retention_days.to_string());
		}
		logger_retention_days.to_string()
	});
}
