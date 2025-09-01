pub mod json;
pub mod toml;
mod utils;
pub mod yaml;

use crate::path::CONFIG_DIR;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggerConfig {
    /// 日志等级
    pub level: String,
    /// 日志路径
    pub path: String,
    /// 日志保留天数
    pub retention_days: u8,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            path: "logs".to_string(),
            retention_days: 7,
        }
    }
}

impl LoggerConfig {
    fn get() -> Self {
        toml::read_config::<LoggerConfig>(CONFIG_DIR.as_path(), "bot")
            .unwrap_or_else(|_| LoggerConfig::default())
    }
    pub fn level(&self) -> String {
        Self::get().level
    }
    pub fn path(&self) -> String {
        Self::get().path
    }
    pub fn retention_days(&self) -> u8 {
        Self::get().retention_days
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BotConfig {
    /// 日志等级
    pub logger: LoggerConfig,
}

impl BotConfig {
    pub fn get() -> Self {
        toml::read_config::<BotConfig>(CONFIG_DIR.as_path(), "bot")
            .unwrap_or_else(|_| BotConfig::default())
    }
    /// 获取日志等级
    pub fn logger_level(&self) -> LoggerConfig {
        Self::get().logger
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    /// redis主机
    pub host: String,
    /// redis端口
    pub port: u16,
    /// redis密码
    pub password: String,
    /// redis数据库
    pub db: u8,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 6379,
            password: "".to_string(),
            db: 0,
        }
    }
}

impl RedisConfig {
    pub fn get() -> Self {
        toml::read_config::<RedisConfig>(CONFIG_DIR.as_path(), "redis")
            .unwrap_or_else(|_| RedisConfig::default())
    }
    pub fn host(&self) -> String {
        Self::get().host
    }
    pub fn port(&self) -> u16 {
        Self::get().port
    }
    pub fn password(&self) -> String {
        Self::get().password
    }
    pub fn db(&self) -> u8 {
        Self::get().db
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub bot: BotConfig,
}

impl Config {
    /// 获取bot配置
    ///
    /// # 返回值
    ///
    /// * `BotConfig` - bot配置
    ///
    /// # 示例
    ///
    /// ```
    /// use puniyu_utils::config::Config;
    /// let config = Config::bot();
    /// ```
    pub fn bot() -> BotConfig {
        BotConfig::get()
    }

    /// 获取redis配置
    ///
    /// # 返回值
    ///
    /// * `RedisConfig` - redis配置
    ///
    /// # 示例
    ///
    /// ```
    /// use puniyu_utils::config::Config;
    /// let config = Config::redis();
    /// ```
    pub fn redis() -> RedisConfig {
        RedisConfig::get()
    }
}
