#[cfg(feature = "account")]
pub mod account;
#[cfg(feature = "adapter")]
pub mod adapter;
#[cfg(feature = "command")]
pub mod command;
#[cfg(feature = "plugin")]
pub mod plugin;
#[cfg(feature = "task")]
pub mod task;

#[cfg(feature = "server")]
pub type ServerType = std::sync::Arc<dyn Fn(&mut actix_web::web::ServiceConfig) + Send + Sync>;
