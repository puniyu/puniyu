use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	/// 处理器未找到。
	#[error("not found: {0}")]
	NotFound(String),

	/// 处理器已存在。
	#[error("exists: {0}")]
	Exists(String),
}
