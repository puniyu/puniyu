//! 配置相关错误类型
//!
//! 提供配置文件读写和解析过程中可能出现的错误类型。

use std::io;
use thiserror::Error;

/// 配置错误枚举
///
/// 定义了配置操作中可能出现的各种错误类型。
///
/// # 变体
///
/// - `Io` - I/O 操作错误（文件读写等）
/// - `Deserialize` - TOML 反序列化错误
/// - `Serialize` - TOML 序列化错误
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_error::config::Error;
/// use std::fs;
///
/// fn load_config(path: &str) -> Result<String, Error> {
///     let content = fs::read_to_string(path)?;
///     Ok(content)
/// }
/// ```
#[derive(Error, Debug)]
pub enum Error {
    /// I/O 操作错误
    ///
    /// 当文件读写操作失败时返回此错误。
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    /// TOML 反序列化错误
    ///
    /// 当 TOML 配置文件解析失败时返回此错误。
    #[error("TOML parse error: {0}")]
    Deserialize(#[from] toml::de::Error),

    /// TOML 序列化错误
    ///
    /// 当将配置序列化为 TOML 格式失败时返回此错误。
    #[error("TOML serialize error: {0}")]
    Serialize(#[from] toml::ser::Error),
}
