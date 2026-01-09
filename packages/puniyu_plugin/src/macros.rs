pub use puniyu_macros::{command, plugin, plugin_config as config, server, task};
pub use puniyu_types::{contact_friend, contact_group};
pub use puniyu_types::{message, segment};

pub mod proc_macro {
    pub use async_trait::async_trait;
    pub use actix_web::web::ServiceConfig;
    pub use inventory;
    pub use puniyu_config::ConfigRegistry;
    pub use puniyu_registry::command::CommandRegistry;
    pub use puniyu_registry::plugin::VERSION as ABI_VERSION;
    pub use puniyu_types::config::Config;
    pub use puniyu_types::plugin::PluginBuilder;
    pub use puniyu_types::task::TaskBuilder;
    pub use puniyu_types::command::CommandBuilder;
    pub use puniyu_types::command::{Arg, ArgMode, ArgType, ArgValue, HandlerAction, HandlerResult};
    pub use puniyu_types::hook::HookBuilder;
    pub use serde_json;
    pub use toml;
    pub use puniyu_types::version::Version;
    pub use crate::context::*;
    pub use crate::server::*;
}