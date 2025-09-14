use thiserror::Error;

#[derive(Error, Debug)]
pub enum Plugin {
    #[error("插件: {0}不存在")]
    NotFound(String),
    #[error("插件: {0}已存在")]
    Exists(String),
    #[error("插件: {0}初始化失败")]
    InitializationFailed(String),
    #[error("插件: {0}ABI版本不匹配, 插件版本: {1}, 运行时版本: {2}")]
    ABIVersion(String, String, String),
}

#[derive(Error, Debug)]
pub enum Adapter {
    #[error("适配器: {0}不存在")]
    NotFound(String),
    #[error("适配器: {0}已存在")]
    Exists(String),
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
