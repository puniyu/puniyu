#[cfg(feature = "adapter")]
#[derive(thiserror::Error, Debug)]
pub enum Adapter {
	#[error("适配器: {0}不存在")]
	NotFound(String),
	#[error("适配器: {0}已存在")]
	Exists(String),
	#[error("适配器: 初始化失败: {0}")]
	Init(String),
}

#[cfg(feature = "plugin")]
#[derive(thiserror::Error, Debug)]
pub enum Plugin {
	#[error("插件: {0}不存在")]
	NotFound(String),
	#[error("插件: {0}已存在")]
	Exists(String),
	#[error("插件: 初始化失败: {0}")]
	Init(String),
}
