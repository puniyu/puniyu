use thiserror::Error;
#[derive(Debug, Error)]
pub enum Error {
	#[error("Session异常")]
	Session,
	#[error("事件异常")]
	Event,
}
