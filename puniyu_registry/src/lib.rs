pub mod logger;
pub mod plugin;
pub use plugin::manger::PluginManager;
pub mod adapter;
mod error;
pub use adapter::manger::AdapterManger;
pub mod bot;
pub mod library;
mod utils;
mod version;
pub use version::VERSION;

pub use inventory;
pub use puniyu_plugin_derive::{plugin, task};
