use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("not found: {0}")]
    NotFound(String),
    #[error("exists: {0}")]
    Exists(String)
}