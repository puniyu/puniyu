use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {

	#[error("protocol version mismatch: expected {expected}, got {got}")]
	VersionMismatch { expected: u8, got: u8 },

	#[error("unknown frame type: {0}")]
	UnknownFrameType(u8),

	#[error("empty frame")]
	EmptyFrame,

	// === 编解码 ===
	#[error("encode error: {0}")]
	Encode(#[from] rmp_serde::encode::Error),

	#[error("decode error: {0}")]
	Decode(#[from] rmp_serde::decode::Error),

	// === 传输层 ===
	#[error("io error: {0}")]
	Io(#[from] std::io::Error),

	// === 请求/响应 ===
	#[error("remote error: {0}")]
	Remote(String),

	#[error("request timeout")]
	Timeout,

	#[error("channel closed")]
	ChannelClosed,
}
