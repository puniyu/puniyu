use thiserror::Error;

/// 调度器错误。
#[derive(Error, Debug)]
pub enum Error {
	/// 调度器尚未初始化。
	#[error("scheduler not initialized")]
	NotInitialized,

	/// 调度器已初始化，无法重复初始化。
	#[error("scheduler already initialized")]
	AlreadyInitialized,

	/// 项不存在。
	#[error("not found: {0}")]
	NotFound(String),

	/// 项已存在。
	#[error("exists: {0}")]
	Exists(String),

	/// 创建调度器失败。
	#[error("failed to create scheduler: {0}")]
	Create(String),

	/// 启动调度器失败。
	#[error("failed to start scheduler: {0}")]
	Start(String),

	/// 关闭调度器失败。
	#[error("failed to shutdown scheduler: {0}")]
	Shutdown(String),
}
