use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error("插件: {0}不存在")]
	NotFound(String),
	#[error("插件: {0}已存在")]
	Exists(String),
	#[error("插件: 初始化失败: {0}")]
	Init(String),
}
