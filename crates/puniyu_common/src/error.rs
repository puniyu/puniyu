use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error("配置文件错误: IO错误")]
	Io(#[from] io::Error),
	#[error("配置文件错误: TOML解析错误")]
	Parse(#[from] toml::de::Error),
	#[error("配置文件错误: TOML序列化错误")]
	Serialize(#[from] toml::ser::Error),
	#[error("配置文件错误: 反序列化错误")]
	Deserialize,
}
