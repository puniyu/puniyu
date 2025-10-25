mod app;
pub mod error;
pub mod path;
pub mod system;
pub mod toml;

pub use app::APP_NAME;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
