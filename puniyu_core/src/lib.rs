pub mod bot;
pub mod common;
pub mod config;
pub mod error;
pub mod logger;
mod system;
pub use system::*;
mod app;
pub use app::{APP_NAME, App};
pub mod plugin;
mod version;
pub use plugin::get_plugin_info;

pub mod adapter;

pub use version::VERSION;

pub use async_trait::async_trait;
pub use puniyu_registry::{
	VERSION as ABI_VERSION,
	plugin::{
		builder::PluginBuilder, command::builder::CommandBuilder, task::builder::TaskBuilder,
	},
};

pub use inventory;
