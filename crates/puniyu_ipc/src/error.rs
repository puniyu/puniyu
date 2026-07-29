use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("protocol version mismatch: expected {expected}, got {got}")]
	VersionMismatch { expected: u8, got: u8 },

	#[error("unknown service: {0}")]
	UnknownService(String),

	#[error("encode error: {0}")]
	Encode(#[from] rmp_serde::encode::Error),

	#[error("decode error: {0}")]
	Decode(#[from] rmp_serde::decode::Error),

	#[error("io error: {0}")]
	Io(#[from] std::io::Error),

	#[error("service error: {0}")]
	Service(String),
}
