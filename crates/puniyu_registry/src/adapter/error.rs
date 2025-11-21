use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error("适配器: {0}不存在")]
	NotFound(String),
	#[error("适配器: {0}已存在")]
	Exists(String),
	#[error("适配器: 初始化失败: {0}")]
	Init(String),
}