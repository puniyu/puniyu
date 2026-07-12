use thiserror::Error;

/// TOML 序列化/反序列化错误。
#[derive(Error, Debug)]
pub enum TomlError {
	#[error("serialization: {0}")]
	Ser(#[from] toml::ser::Error),

	#[error("deserialization: {0}")]
	De(#[from] toml::de::Error),
}

impl From<toml::ser::Error> for Error {
	fn from(e: toml::ser::Error) -> Self {
		Error::Toml(TomlError::Ser(e))
	}
}

impl From<toml::de::Error> for Error {
	fn from(e: toml::de::Error) -> Self {
		Error::Toml(TomlError::De(e))
	}
}

/// 配置错误。
#[derive(Error, Debug)]
pub enum Error {
	/// 未找到配置
	#[error("not found: Config")]
	NotFound,

	/// 配置已存在
	#[error("exists: Config")]
	Exists,

	/// toml 错误
	#[error("toml error: {0}")]
	Toml(#[from] TomlError),

	/// IO 错误
	#[error("io error: {0}")]
	Io(#[from] std::io::Error),
}
