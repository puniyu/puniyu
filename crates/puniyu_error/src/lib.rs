//! # puniyu_error
//!
//! 错误类型定义库，提供配置和注册表相关的统一错误处理。
//!
//! ## 概述
//!
//! `puniyu_error` 提供了 Puniyu 框架中常用的错误类型定义，包括：
//!
//! - **配置错误（config）** - 处理配置文件读写和解析错误
//! - **注册表错误（registry）** - 处理注册表操作相关错误
//!
//! ## 特性
//!
//! - 🎯 **类型安全** - 使用 Rust 类型系统确保错误处理的正确性
//! - 🔧 **模块化** - 通过特性标志按需启用错误类型
//! - 🔄 **统一接口** - 提供统一的 `Result` 类型别名
//! - 📦 **易于使用** - 基于 `thiserror` 提供清晰的错误信息
//!
//! ## 使用方式
//!
//! ### 使用配置错误
//!
//! ```rust,ignore
//! use puniyu_error::config::Error;
//!
//! fn load_config() -> Result<(), Error> {
//!     // 配置加载逻辑
//!     Ok(())
//! }
//! ```
//!
//! ### 使用注册表错误
//!
//! ```rust,ignore
//! use puniyu_error::registry::Error;
//!
//! fn register_item() -> Result<(), Error> {
//!     // 注册逻辑
//!     Ok(())
//! }
//! ```
//!
//! ### 使用通用 Result 类型
//!
//! ```rust
//! use puniyu_error::Result;
//!
//! fn do_something() -> Result {
//!     Ok(())
//! }
//!
//! fn do_something_with_value() -> Result<String> {
//!     Ok("success".to_string())
//! }
//! ```

#[cfg(feature = "config")]
pub mod config;
#[cfg(feature = "registry")]
pub mod registry;

/// 通用 Result 类型别名
///
/// 提供一个便捷的 Result 类型，使用 `Box<dyn Error + Send + Sync>` 作为错误类型。
///
/// # 类型参数
///
/// - `T` - 成功时的返回值类型，默认为 `()`
///
/// # 示例
///
/// ```rust
/// use puniyu_error::Result;
///
/// fn operation() -> Result {
///     Ok(())
/// }
///
/// fn operation_with_value() -> Result<i32> {
///     Ok(42)
/// }
/// ```
pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
