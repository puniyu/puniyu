//! 动态库加载错误类型

use std::path::PathBuf;
use thiserror::Error;

/// 动态库操作错误枚举
///
/// 定义了动态库加载、卸载等操作中可能出现的各种错误类型。
///
/// # 示例
///
/// ```rust
/// use puniyu_library::Error;
///
/// fn check_library(name: &str) -> Result<(), Error> {
///     Err(Error::NotFound(name.to_string()))
/// }
/// ```
#[derive(Error, Debug)]
pub enum Error {
	/// 库不存在
	///
	/// 当尝试获取或操作不存在的库时返回此错误。
	#[error("library not found: {0}")]
	NotFound(String),

	/// 库已存在
	///
	/// 当尝试加载已存在的库时返回此错误。
	#[error("library already exists: {0}")]
	Exists(String),

	/// 库正在使用中
	///
	/// 当尝试卸载正在使用的库时返回此错误。
	#[error("library in use: {0}")]
	InUse(String),

	/// 库加载失败
	///
	/// 当动态库加载失败时返回此错误。
	#[error("failed to load library: {path} - {source}")]
	LoadFailed {
		/// 库文件路径
		path: PathBuf,
		/// 底层错误
		source: libloading::Error,
	},

	/// 无效的库路径
	///
	/// 当提供的路径无效或无法提取文件名时返回此错误。
	#[error("invalid library path: {0}")]
	InvalidPath(PathBuf),
}
