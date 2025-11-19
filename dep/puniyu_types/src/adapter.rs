mod types;
pub use types::*;
mod error;
pub use error::Error;
mod api;
pub use api::AdapterApi;


pub type Result<T> = std::result::Result<T, Error>;