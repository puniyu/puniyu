mod store;

#[cfg(feature = "bot")]
pub mod bot;

#[cfg(feature = "adapter")]
pub mod adapter;

#[cfg(feature = "task")]
pub mod task;

#[cfg(feature = "command")]
pub mod command;

#[cfg(feature = "server")]
pub mod server;

#[cfg(feature = "plugin")]
pub mod plugin;

#[cfg(feature = "cooldown")]
pub mod cooldown;

#[cfg(feature = "handler")]
pub mod handler;
mod hook;

#[cfg(feature = "handler")]
pub use handler::HandlerRegistry;
