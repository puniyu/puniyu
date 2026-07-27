use thiserror::Error;

/// 调度器错误。
#[derive(Error, Debug)]
pub enum Error {

	/// Cron 表达式无效。
	#[error("invalid task schedule for '{task}': {message}")]
	InvalidSchedule { task: String, message: String },
}
