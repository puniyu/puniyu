use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error("库: {0}不存在")]
	NotFound(String),
	#[error("库: {0}已存在")]
	Exists(String),
	#[error("库: {0}关闭失败")]
	Close(String),
}
