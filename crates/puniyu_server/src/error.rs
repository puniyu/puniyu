use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error("http capability is already attached to a server")]
	AlreadyAttached,
	#[error("server is already running")]
	AlreadyRunning,
	#[error("http capability is not running")]
	NotRunning,
	#[error("server is draining")]
	Draining,
	#[error("http mount id exhausted")]
	MountIdExhausted,
	#[error("server state lock is poisoned")]
	Poisoned,
	#[error("server bind failed: {0}")]
	Bind(String),
	#[error("server task failed: {0}")]
	Task(#[from] tokio::task::JoinError),
}
