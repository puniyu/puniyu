use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error("server is already running")]
	AlreadyRunning,
	#[error("http component is already mounted")]
	AlreadyMounted,
	#[error("server is not available")]
	ServerUnavailable,
	#[error("server is not running")]
	NotRunning,
	#[error("server bind failed: {0}")]
	Bind(String),
	#[error("server failed: {0}")]
	Serve(#[source] std::io::Error),
	#[error("server task failed: {0}")]
	Task(#[from] tokio::task::JoinError),
}
