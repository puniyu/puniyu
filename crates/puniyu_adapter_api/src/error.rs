#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Adapter(#[from] Box<dyn std::error::Error + Send + Sync>),
}