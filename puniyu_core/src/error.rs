use puniyu_common::error::Config;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error {
	#[error("配置文件错误: {0}")]
	Config(#[from] Config),
}
