//! 注册表相关错误类型
//!
//! 提供注册表操作过程中可能出现的错误类型。

use thiserror::Error;

/// 注册表错误枚举
///
/// 定义了注册表操作中可能出现的各种错误类型。
///
/// # 变体
///
/// - `NotFound` - 未找到指定的项
/// - `Exists` - 项已存在
///
/// # 示例
///
/// ```rust
/// use puniyu_error::registry::Error;
///
/// fn get_item(name: &str) -> Result<(), Error> {
///     Err(Error::NotFound(name.to_string()))
/// }
///
/// fn register_item(name: &str) -> Result<(), Error> {
///     Err(Error::Exists(name.to_string()))
/// }
/// ```
#[derive(Error, Debug)]
pub enum Error {
    /// 未找到指定的项
    ///
    /// 当尝试获取或操作不存在的项时返回此错误。
    ///
    /// # 示例
    ///
    /// ```rust
    /// use puniyu_error::registry::Error;
    ///
    /// let error = Error::NotFound("my_item".to_string());
    /// assert_eq!(error.to_string(), "not found: my_item");
    /// ```
    #[error("not found: {0}")]
    NotFound(String),

    /// 项已存在
    ///
    /// 当尝试注册已存在的项时返回此错误。
    ///
    /// # 示例
    ///
    /// ```rust
    /// use puniyu_error::registry::Error;
    ///
    /// let error = Error::Exists("my_item".to_string());
    /// assert_eq!(error.to_string(), "exists: my_item");
    /// ```
    #[error("exists: {0}")]
    Exists(String),
}
