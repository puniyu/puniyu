use thiserror::Error;

#[derive(Error, Debug)]
pub enum Plugin {
    #[error("插件: {0}不存在")]
    NotFound(String),
    #[error("插件: {0}已存在")]
    Exists(String),
}
