#[cfg(feature = "config")]
pub mod config;
#[cfg(feature = "registry")]
pub mod registry;

pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
