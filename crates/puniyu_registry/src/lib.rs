extern crate core;

#[cfg(feature = "adapter")]
mod adapter;
#[cfg(feature = "adapter")]
pub use adapter::AdapterRegistry;
pub mod error;

#[cfg(feature = "task")]
mod task;
#[cfg(feature = "task")]
pub use task::TaskRegistry;
#[cfg(feature = "plugin")]
mod plugin;

#[cfg(feature = "plugin")]
pub use plugin::PluginRegistry;

#[cfg(feature = "command")]
pub use puniyu_command::CommandRegistry;
mod server;
#[cfg(feature = "server")]
pub use server::ServerRegistry;
