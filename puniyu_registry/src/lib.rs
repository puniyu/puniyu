pub mod logger;
mod manger;
pub mod plugin;
pub use manger::PluginManager;
mod error;

pub use inventory;
pub use puniyu_macro::plugin;
pub use puniyu_macro::task;
