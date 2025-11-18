use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Config {
	#[error("配置文件错误: IO错误")]
	Io(#[from] io::Error),
	
	#[error("配置文件错误: 写入错误")]
	Write,
	
	#[error("配置文件错误: TOML解析错误")]
	Parse(#[from] toml::de::Error),
	
	#[error("配置文件错误: 删除节点错误")]
	RemoveNode,
	
	#[error("配置文件错误: TOML序列化错误")]
	Serialize(#[from] toml::ser::Error),
	
	#[error("配置文件错误: 反序列化错误")]
	Deserialize,
}
