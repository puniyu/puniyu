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
mod handler;

#[cfg(feature = "handler")]
pub use handler::HandlerRegistry;

#[cfg(feature = "hook")]
mod hook;
#[cfg(feature = "hook")]
pub use hook::HookRegistry;
