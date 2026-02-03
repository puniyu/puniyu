use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error("接口未实现")]
	NotImpl,
	#[error("请求错误: {0}")]
	Request(#[from] reqwest::Error),
	#[error("IO错误: {0}")]
	Io(#[from] std::io::Error),
	#[error("其他错误: {0}")]
	Other(String),
}
