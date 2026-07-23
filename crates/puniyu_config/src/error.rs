use thiserror::Error;

/// 配置错误。
#[derive(Error, Debug)]
pub enum Error {
	/// toml 错误
	#[error("toml error: {0}")]
	Toml(#[from] toml::de::Error),

	/// IO 错误
	#[error("io error: {0}")]
	Io(#[from] std::io::Error),
}
