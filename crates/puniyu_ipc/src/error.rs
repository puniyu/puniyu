use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("transport error: {0}")]
	Transport(#[from] io::Error),
	#[error("frame is invalid: {0}")]
	InvalidFrame(&'static str),
	#[error("frame length mismatch: declared {declared}, actual {actual}")]
	FrameLengthMismatch { declared: usize, actual: usize },
	#[error("frame is too large: {0} bytes")]
	FrameTooLarge(usize),
	#[error("unsupported protocol version: {0}")]
	UnsupportedVersion(u8),
	#[error("unknown message kind: {0}")]
	UnknownKind(u8),
	#[error("failed to encode message payload: {0}")]
	Encode(#[from] rmp_serde::encode::Error),
	#[error("failed to decode message payload: {0}")]
	Decode(#[from] rmp_serde::decode::Error),
	#[error("value encode/decode error: {0}")]
	Value(#[from] rmpv::ext::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
