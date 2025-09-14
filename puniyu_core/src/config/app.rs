use crate::{config::reload_config, error::Config as Error};
use puniyu_utils::path::CONFIG_DIR;
use puniyu_utils::utils::toml;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, LazyLock, RwLock};

static APP_CONFIG: LazyLock<Arc<RwLock<AppConfig>>> =
    LazyLock::new(|| Arc::new(RwLock::new(AppConfig::get())));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggerConfig {
    /// 日志等级
    #[serde(default = "default_logger_level")]
    pub level: String,
    /// 日志路径
    #[serde(default = "default_logger_path")]
    pub path: String,
    /// 日志保留天数
    #[serde(default = "default_logger_retention_days")]
    pub retention_days: u8,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            level: default_logger_level(),
            path: default_logger_path(),
            retention_days: default_logger_retention_days(),
        }
    }
}

fn default_logger_level() -> String {
    String::from("info")
}

fn default_logger_path() -> String {
    String::from("logs")
}

fn default_logger_retention_days() -> u8 {
    7
}

impl LoggerConfig {
    pub fn level(&self) -> &str {
        self.level.as_str()
    }
    pub fn path(&self) -> &str {
        self.path.as_str()
    }
    pub fn retention_days(&self) -> u8 {
        self.retention_days
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// 日志等级
    #[serde(default)]
    pub logger: LoggerConfig,
}

impl AppConfig {
    pub fn get() -> Self {
        toml::read_config::<AppConfig>(CONFIG_DIR.as_path(), "bot")
            .unwrap_or_else(|_| AppConfig::default())
    }
    /// 获取日志等级
    pub fn logger(&self) -> LoggerConfig {
        let config = APP_CONFIG.read().unwrap();
        config.logger.clone()
    }

    pub fn reload() -> Result<(), Error> {
        let new_config = reload_config("app").map_err(|_| Error::Read)?;
        let mut config = APP_CONFIG.write().map_err(|_| Error::Write)?;
        *config = new_config;
        Ok(())
    }
}
