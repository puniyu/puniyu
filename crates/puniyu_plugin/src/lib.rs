pub mod path;
pub mod account;
pub mod bot;
pub mod contact;
pub mod element;
pub mod event;
pub mod hook;
pub mod sender;
pub mod server;
pub mod context;
pub mod command;
pub mod task;
pub mod prelude;
pub mod message;
mod types;
mod macros;
mod version;
pub use puniyu_error::Result;
pub use version::*;
pub use types::*;
pub use puniyu_api::{app_name, app_version};

#[doc(hidden)]
pub mod __private {
    pub use async_trait::async_trait;
    pub use puniyu_plugin_core::Plugin;
    pub use puniyu_api::inventory;
    pub use puniyu_config::{Config, ConfigInfo};
    pub use puniyu_hook::Hook;
    pub use puniyu_task::Task;
    pub use puniyu_server::ServerFunction;
    pub use puniyu_command::Command;
    pub use toml;
}
