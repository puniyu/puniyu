use thiserror::Error;

/// 机器人模块错误。
#[derive(Error, Debug)]
pub enum Error {
    /// 未找到机器人。
    #[error("not found: Bot")]
    NotFound,

    /// 机器人已存在。
    #[error("exists: Bot")]
    Exists,
}
