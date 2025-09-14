use crate::config::{app::AppConfig, bot::BotConfig, group::GroupConfig};
use crate::error::Config as Error;
use notify::{Config as WatcherConfig, Event, RecommendedWatcher, RecursiveMode, Watcher};
use puniyu_utils::path::CONFIG_DIR;
use puniyu_utils::utils::toml::merge_config;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;
use std::{env, thread};

mod app;
mod bot;
mod group;

pub(crate) fn reload_config<T>(name: &str) -> Result<T, Error>
where
    T: Default + serde::de::DeserializeOwned,
{
    match puniyu_utils::utils::toml::read_config(CONFIG_DIR.as_path(), name) {
        Ok(config) => Ok(config),
        Err(_) => Ok(T::default()),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub app: AppConfig,
    #[serde(default)]
    pub group: GroupConfig,
}

impl Config {
    /// 获取app配置
    ///
    /// # 返回值
    ///
    /// * `AppConfig` - app配置
    ///
    /// # 示例
    ///
    /// ```
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
    /// ```
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
    /// ```
    /// use puniyu_core::config::Config;
    /// let config = Config::bot();
    /// ```
    pub fn bot() -> BotConfig {
        BotConfig::get()
    }
}

pub fn init_config() {
    if !CONFIG_DIR.as_path().exists() {
        std::fs::create_dir_all(CONFIG_DIR.as_path()).unwrap();
    }
    let default_config = AppConfig::default();
    let user_config = AppConfig::get();
    let _ = merge_config(CONFIG_DIR.as_path(), "bot", &user_config, &default_config);
    init_env();
}

pub(crate) fn config_watcher() {
    thread::spawn(|| {
        let (tx, rx) = mpsc::channel::<notify::Result<Event>>();

        let mut watcher = RecommendedWatcher::new(
            tx,
            WatcherConfig::default()
                .with_follow_symlinks(false)
                .with_poll_interval(Duration::from_secs(200)),
        )
        .unwrap();

        watcher
            .watch(CONFIG_DIR.as_path(), RecursiveMode::NonRecursive)
            .unwrap();

        log::info!("[配置文件]配置文件监听器已启动");

        for res in rx {
            match res {
                Ok(event) => {
                    log::info!(
                        "[配置文件] 文件变更: {}",
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
                                    AppConfig::reload()
                                        .map_err(|e| {
                                            log::error!("[配置文件] 重载App配置失败: {}", e)
                                        })
                                        .unwrap();
                                }
                                "bot.toml" => {
                                    BotConfig::reload()
                                        .map_err(|e| {
                                            log::error!("[配置文件] 重载Bot配置失败: {}", e)
                                        })
                                        .unwrap();
                                }
                                "group.toml" => {
                                    GroupConfig::reload()
                                        .map_err(|e| {
                                            log::error!("[配置文件] 重载Group配置失败: {}", e)
                                        })
                                        .unwrap();
                                }
                                _ => {
                                    AppConfig::reload()
                                        .map_err(|e| {
                                            log::error!("[配置文件] 重载App配置失败: {}", e)
                                        })
                                        .unwrap();
                                    BotConfig::reload()
                                        .map_err(|e| {
                                            log::error!("[配置文件] 重载Bot配置失败: {}", e)
                                        })
                                        .unwrap();
                                    GroupConfig::reload()
                                        .map_err(|e| {
                                            log::error!("[配置文件] 重载Group配置失败: {}", e)
                                        })
                                        .unwrap();
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    log::error!("[配置文件] 监听错误: {}", e);
                }
            }
        }
    });
}

fn init_env() {
    let logger_config = Config::app().logger;
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
        logger_path.to_string()
    });

    env::var("LOGGER_RETENTION_DAYS").unwrap_or_else(|_| {
        let logger_retention_days = logger_config.retention_days();
        unsafe {
            env::set_var("LOGGER_RETENTION_DAYS", logger_retention_days.to_string());
        }
        logger_retention_days.to_string()
    });
}
