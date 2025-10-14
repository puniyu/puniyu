use thiserror::Error;

#[derive(Error, Debug)]
pub enum Plugin {
	#[error("插件: {0}不存在")]
	NotFound(String),
	#[error("插件: {0}已存在")]
	Exists(String),
	#[error("插件: 初始化失败: {0}")]
	Init(String),
}

#[derive(Error, Debug)]
pub enum Adapter {
	#[error("适配器: {0}不存在")]
	NotFound(String),
	#[error("适配器: {0}已存在")]
	Exists(String),
	#[error("适配器: 初始化失败: {0}")]
	Init(String),
}

#[derive(Error, Debug)]
pub enum Library {
	#[error("库: {0}不存在")]
	NotFound(String),
	#[error("库: {0}已存在")]
	Exists(String),
	#[error("库: {0}关闭失败")]
	Close(String),
}
