use thiserror::Error;

/// 调度器错误。
#[derive(Error, Debug)]
pub enum Error {
	/// Cron 表达式无效。
	#[error("invalid cron expression for '{task}': {message}")]
	InvalidSchedule { task: String, message: String },

	/// 创建调度器失败。
	#[error("failed to create scheduler: {0}")]
	SchedulerCreate(String),

	/// 启动调度器失败。
	#[error("failed to start scheduler: {0}")]
	SchedulerStart(String),

	/// 停止调度器失败。
	#[error("failed to shutdown scheduler: {0}")]
	SchedulerShutdown(String),

	/// 添加任务失败。
	#[error("failed to add job for '{task}': {message}")]
	JobAdd { task: String, message: String },

	/// 移除任务失败。
	#[error("failed to remove job for '{task}': {message}")]
	JobRemove { task: String, message: String },
}
