pub mod json;
pub mod toml;
mod utils;
pub mod yaml;

use serde::{Deserialize, Serialize};
use std::env::current_dir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotConfig {
    /// 日志等级
    pub logger_level: String,
    /// 日志目录
    pub log_dir: String,
    /// 日志保留天数
    pub retention_days: u8,
}

impl Default for BotConfig {
    fn default() -> Self {
        Self {
            logger_level: "info".to_string(),
            log_dir: "logs".to_string(),
            retention_days: 7,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub bot: BotConfig,
}

impl Config {
    fn get_config_path() -> std::path::PathBuf {
        current_dir()
            .map(|p| p.join("config"))
            .unwrap_or_else(|_| std::path::PathBuf::from("./config"))
    }
    pub fn bot() -> BotConfig {
        toml::read_config::<BotConfig>(&Self::get_config_path(), "bot")
            .unwrap_or_else(|_| BotConfig::default())
    }
}
